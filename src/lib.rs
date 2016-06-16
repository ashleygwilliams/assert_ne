/// Asserts that two expressions are not equal to each other.
///
/// On panic, this macro will print the values of the expressions with their
/// debug representations.
///
/// # Examples
///
/// ```should panic
/// # #[macro_use] extern crate assert_ne;
///
/// let a = 3;
/// let b = 1 + 2;
/// assert_ne!(a, b);
/// ```
#[macro_export]
macro_rules! assert_ne {
    ($left:expr , $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if (*left_val == *right_val) {
                    panic!("assertion failed: `(left !== right)` \
                           (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}
