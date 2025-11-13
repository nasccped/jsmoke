# JSmoke documentation

A more deep understanding about JSmoke, it's features and philosophy.

## Table of contents

- [A little about](#a-little-about)
  - [Naming](#naming)
  - [Why does JSmoke exists?](#why-does-jsmoke-exists)
- [Kojamp, its predecessor](#kojamp-its-predecessor)
- [Features](#features)
  - [Non essential features](#non-essential-features)
- [License](#license)

## A little about

JSmoke is a Java project manager intended to be simple and run fast
enough that you forget you're using a manager.

### Naming

The _"JSmoke"_ comes from _"Java smoke"_. A cup of coffee blazingly
hot (written in Rust, btw).

### Why does JSmoke exists?

When writing Java programs, I noticed that a [Makefile] was kinda
boring to rewrite every time a new project was created (still better
than passing file paths by the cmdline). Even using [Maven] or
[Gradle] looks bad. It's a too-heavy tooling for my simple todo-list.

So I though:

> Imagine if the people could create and run it's projects just by
> using the terminal in the simplest way...
>
> No overloaded and fancy IDEs. No heavy-tooling overhead. Just you,
> the code and the final result + useful security/workflow features
> that won't take more than millisecs[^millisecs] to run.

Then, JSmoke was born.

## Kojamp, its predecessor

[Kojamp] was the first version of this software but was discontinued.
It was intended to offer support for both `Java` and `Kotlin`
applications.

The confuse Java support turned impossible to implement `Kotlin`
features. Also, due to aesthetic code principles and implementing
I've decided to redo the program from scratch with a new look.

> [!NOTE]
>
> Even discontinued, [Kojamp] still works _(not the way I want, but
> it does the job 😁)_.

## Features

Since JSmoke is a manager it provides similar features already
provided in other managers, such as [Maven] or [Gradle].

Here's a small check list to stay tuned with the project progress:

- [ ] 🆕 create new projects
- [ ] 🏗️ build _(compile)_ projects
- [ ] 🏎️ run project _(bytecode)_
- [ ] 🧼 clear generated files _(bytecode)_
- [ ] 📰 `stderr`[^stderr] report logging

### Non essential features

Here's other features that contains lower priority but are already in
the implementing queue:

- [ ] **create workspace file:** create a new workspace file (like
  `pom.xml`[^pom.xml] in Java or `Cargo.toml`[^cargo.toml] in Rust)
  to handle security checks and execute specific tasks before
  compile/run (version/`javac` args).
- [ ] **`javac`/`java` version locking:** lock version project to
  regex matching versions
    - any matching:
      - `x.y.z`: major, minor and patch will be `x`, `y` and `z`
        respectively.
    - minimum version:
      - `^=1.0.0`|`1.0.0`: greater or equals than...
      - `^1.0.0`: explicit greater than...
    - maximum version:
      - `-=1.0.0`: less or equals than...
      - `-1.0.0`: explicit less than...
    - range:
      - `<MIN>~<MAX>`: any version between the `<MIN>` and `<MAX>`
        (**note that** `<MIN>` and `<MAX>` will follow all rules
        mentioned above - even _(ex/in)clusive_ ones).

## License

This project is under the [MIT License]. You can read the License
file by [clicking here](https://github.com/nasccped/jsmoke/blob/main/LICENSE)!

[Maven]: https://maven.apache.org/
[Gradle]: https://gradle.org/
[Makefile]: https://www.gnu.org/software/make/manual/make.html
[Kojamp]: https://github.com/nasccped/kojamp
[MIT License]: https://opensource.org/license/mit

[^millisecs]: `millisecs` stands for milliseconds, one thousandth
  (`0.001` or <code>10<sup>−3</sup></code> or `1/1000`) of a second
  (or `1000` microseconds).
[^stderr]: `stderr`, or **Standard Error**, is one of three standard
  data streams in Unix-like operating systems (the others being
  `stdin` for input and `stdout` for output). It is a dedicated
  channel for programs to output error messages, warnings, and
  diagnostic information.
[^pom.xml]: The `pom.xml` file, which stands for _"Project Object
  Model"_, is the fundamental configuration file used by **Apache
  Maven**, a build automation tool primarily for Java projects. It is
  an XML file located in the root directory of a Maven project.
[^cargo.toml]: `Cargo.toml` is the manifest file used by Cargo,
  Rust's package manager and build system. It is a crucial component
  of any Rust project managed by Cargo, defining the project's
  configuration and dependencies.
