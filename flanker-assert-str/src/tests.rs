use alloc::string::{String, ToString};
use core::num::ParseFloatError;
use core::str::FromStr;
use crate::assert_str_loopback;

#[test]
fn test_assert_str_loopback() {
    assert_str_loopback(&0.5);
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)`
  left: `Faulty(0.8)`,
 right: `Faulty(0.9)`")]
fn test_assert_str_loopback_panics() {
    #[derive(Debug, PartialEq)]
    struct Faulty(f64);

    impl ToString for Faulty {
        fn to_string(&self) -> String {
            (self.0 + 0.1).to_string()
        }
    }

    impl FromStr for Faulty {
        type Err = ParseFloatError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(f64::from_str(s)?))
        }
    }

    assert_eq!("0.6", Faulty(0.5).to_string());
    assert_eq!(Faulty(0.5), Faulty::from_str("0.5").unwrap());
    assert_str_loopback(&Faulty(0.8));
}