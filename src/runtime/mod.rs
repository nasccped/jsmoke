mod common;
mod global_context;
mod global_printer;
mod no_subcommand_err;
mod operations;
mod runtime_output;

use crate::cli::{App, subcommands::Subcommand};
use crate::runtime::runtime_output::{FailureConstraint, RuntimeOutputUtils};
use crate::utils::notifiers::{NotifyFailure, NotifySuccess};
use global_context::GlobalContext;
use global_printer::GlobalPrinter;
use no_subcommand_err::NoSubcommand;
use operations::OperationTrait;
use runtime_output::RuntimeOutput;
use std::{cell::RefCell, ops::Deref, process::ExitCode, rc::Rc};

/// Type alias for [`GlobalContext`] mutable cell.
type Context = Rc<RefCell<GlobalContext>>;

/// Run the jsmoke program based on the [`App`] inner fields.
pub fn run(app: App) -> ExitCode {
    let ctx = context_from_app(&app);
    let out = pipe::get_piped_output(app.subcommand, ctx.clone());
    notify_ouput(&out, ctx.clone());
    out.get_exit_code()
}

/// Generates a [`Context`] item and set it up based on an [`App`] reference.
fn context_from_app(app: &App) -> Context {
    let mut gctx = GlobalContext::default();
    gctx.with_force(app.force);
    gctx.with_verbose(app.verbose);
    Rc::new(RefCell::new(gctx))
}

/// Notify the provided [`RuntimeOutput`] based on [`Context`] inner fields. It automatically
/// changes the [`GlobalPrinter`] stderr field to the designated [`Result`] variant. Also prints
/// verbose if necessary.
fn notify_ouput(out: &RuntimeOutput, ctx: Context) {
    let mut ctx = ctx.borrow_mut();
    let verb = ctx.is_verbose();
    let mut printer = ctx.get_mut_printer();
    match &out {
        Ok(ok) => {
            let success: &dyn NotifySuccess = ok.deref();
            printer.print_success(success);
        }
        Err(e) => {
            printer.set_stderr(true);
            let failure: &dyn NotifyFailure = e.deref();
            printer.print_failure(failure);
        }
    }
    if verb {
        printer.print_verbose(out.as_verbose())
    }
}

mod pipe {
    //! # Pipe
    //!
    //! Stores the [`Subcommand`] to [`OperationTrait`] object pipe generating.
    use super::{
        Context, FailureConstraint, NoSubcommand, OperationTrait, RuntimeOutput, Subcommand,
        operations::New as NewOperation,
    };
    use crate::cli::subcommands::New;

    /// Type alias for [`Result`] over a subcommand and a dynamic [`FailureConstraint`].
    ///
    /// That's better than just [`Result<Subcommand, NoSubcommand>`] since report functions expects dyn
    /// trait objects instead of concrete type.
    type SubcommandOrFailure = Result<Subcommand, Box<dyn FailureConstraint>>;

    /// Type alias for [`Result`] over operation and failure.
    /// Used on the following context:
    /// ```no_run
    /// match OperationOrFailure::get_type() {
    ///     Ok(oper) => {
    ///         // run operation and get ouput...
    ///     }
    ///     Err(e) => {
    ///         // handle operation gen. error
    ///     }
    /// }
    /// ```
    type OperationOrFailure = Result<Box<dyn OperationTrait>, Box<dyn FailureConstraint>>;

    /// Runs a 'pipe' over provided inputs to generate the final [`RuntimeOutput`].
    pub fn get_piped_output(subcmd: Option<Subcommand>, context: Context) -> RuntimeOutput {
        let subcmd = subcommand_or_err(subcmd)?;
        let oper = operation_from_subcommand(subcmd)?;
        oper.run(context.clone())
    }

    /// Generates a [`Result`] over an optional [`Subcommand`]:
    /// - [`Ok`] if some
    /// - [`Err`] ([`NoSubcommand`]) otherwise
    fn subcommand_or_err(sub: Option<Subcommand>) -> SubcommandOrFailure {
        sub.ok_or_else(|| Box::new(NoSubcommand) as Box<dyn FailureConstraint>)
    }

    /// Creates an [`OperationOrFailure`] over a [`Subcommand`].
    fn operation_from_subcommand(subcommand: Subcommand) -> OperationOrFailure {
        let trynew = |x: Box<New>| NewOperation::try_from(*x).map(|o| o.into_boxed());
        match subcommand {
            Subcommand::New(n) => trynew(n),
            other => unreachable!("{:?} operation not implemented...", other),
        }
    }
}
