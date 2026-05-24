/// Does input fixing by the `self` value.
pub trait InputFix: Sized {
    /// Fix the `self` input by applying the `operation` function to it.
    ///
    /// The function expects to take ownership over the `self` item and then, convert it to the
    /// 'fixed value'. Note that the value type can change like:
    /// ```
    /// // convertion: String => Vec<String> => String
    /// let string = String::from("   words     and spaces ! ");
    /// let string = string
    ///     .input_fix(|s| {
    ///         s.split_whitespace()
    ///             .map(|word| word.to_string())
    ///             .collect::<Vec<_>>()
    ///     })
    ///     .input_fix(|v| v.join(" "));
    /// assert_eq!(string, "words and spaces");
    /// ```
    ///
    /// You must try to let complex convertions in a single [`InputFix::input_fix`] block, like
    /// this:
    ///
    /// ```
    /// let arr: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    /// let primal_or_double_negative = arr.input_fix(|mut arr| {
    ///     let is_prime = |value: &i32| {
    ///         if *value < 2 {
    ///             return false;
    ///         }
    ///         for i in 2..*value {
    ///             if *value % i == 0 {
    ///                 return false;
    ///             }
    ///         }
    ///         true
    ///     };
    ///     for item in &mut arr {
    ///         if !is_prime(item) {
    ///             *item *= -2;
    ///         }
    ///     }
    ///     arr
    /// });
    /// let expected = [-2, 2, 3, -8, 5, -12, 7];
    /// assert_eq!(primal_or_double_negative, expected);
    /// ```
    fn input_fix<F: FnOnce(Self) -> T, T>(self, operation: F) -> T {
        operation(self)
    }
}

// implement InputFix for any type that implements [`Sized`].
impl<T: Sized> InputFix for T {}

#[cfg(test)]
mod test {
    use super::InputFix;

    #[test]
    fn string_fix() {
        let string = String::from("   words     and spaces! ");
        let string = string
            .input_fix(|s| {
                s.split_ascii_whitespace()
                    .map(|word| word.to_string())
                    .collect::<Vec<_>>()
            })
            .input_fix(|v| v.join(" "));
        assert_eq!(string, "words and spaces!");
    }

    #[test]
    fn int_fix() {
        let mut int = 0;
        for i in 0..10 {
            int = int.input_fix(|val| val + 1);
            assert_eq!(int, i + 1);
        }
    }

    #[test]
    fn complex_fix() {
        let arr: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
        let primal_or_double_negative = arr.input_fix(|mut arr| {
            let is_prime = |value: &i32| {
                if *value < 2 {
                    return false;
                }
                for i in 2..*value {
                    if *value % i == 0 {
                        return false;
                    }
                }
                true
            };
            for item in &mut arr {
                if !is_prime(item) {
                    *item *= -2;
                }
            }
            arr
        });
        let expected = [-2, 2, 3, -8, 5, -12, 7];
        assert_eq!(primal_or_double_negative, expected);
    }
}
