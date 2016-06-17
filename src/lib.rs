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
                if *left_val == *right_val {
                    panic!("assertion failed: `(left != right)` \
                           (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}

#[test]
#[should_panic]
fn it_panics_when_the_values_are_equal() {
  assert_ne!(3, 3);
}

#[test]
fn it_does_not_panic_when_the_values_are_not_equal () {
  assert_ne!(3, 4);
}

