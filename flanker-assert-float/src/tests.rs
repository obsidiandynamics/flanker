use crate::{assert_relative, check_relative};

#[test]
fn test_assert_relative() {
    assert_relative(0.0, 0.0, 0.0);
    assert_relative(0.0, 0.1, 1.0);
    assert_relative(0.0, 0.01, 1.0);
    assert_relative(0.01, 0.01, 0.0);
    assert_relative(0.01, 0.02, 0.5);
    assert_relative(0.0, -0.0, 0.0);
    assert_relative(0.0, -0.1, 1.0);
    assert_relative(-0.01, 0.02, 1.5);
    assert_relative(0.01, -0.02, 1.5);
    assert_relative(-0.01, -0.02, 0.5);
    assert_relative(-0.1, 0.1, 2.0);
}

#[test]
#[should_panic(expected = "assertion failed: (left ≉ right) left: 0, right: 0.1, gamma: 1")]
fn test_assert_relative_panics() {
    assert_relative(0.0, 0.1, 1.0 - f64::EPSILON);
}

#[test]
fn test_check_relative() {
    assert_eq!(
        "(left ≉ right) left: 0, right: 0.1, gamma: 1",
        check_relative(0.0, 0.1, 1.0 - f64::EPSILON).err().unwrap()
    );
    assert_eq!(
        "(left ≉ right) left: 0, right: 0.01, gamma: 1",
        check_relative(0.0, 0.01, 1.0 - f64::EPSILON).err().unwrap()
    );
    assert_eq!(
        "(left ≉ right) left: 0.01, right: 0.02, gamma: 0.5",
        check_relative(0.01, 0.02, 0.5 - f64::EPSILON)
            .err()
            .unwrap()
    );
    assert_eq!(
        "(left ≉ right) left: 0, right: -0.1, gamma: 1",
        check_relative(0.0, -0.1, 1.0 - f64::EPSILON).err().unwrap()
    );
    assert_eq!(
        "(left ≉ right) left: -0.01, right: 0.02, gamma: 1.5",
        check_relative(-0.01, 0.02, 1.5 - f64::EPSILON)
            .err()
            .unwrap()
    );
    assert_eq!(
        "(left ≉ right) left: 0.01, right: -0.02, gamma: 1.5",
        check_relative(0.01, -0.02, 1.5 - f64::EPSILON)
            .err()
            .unwrap()
    );
    assert_eq!(
        "(left ≉ right) left: -0.01, right: -0.02, gamma: 0.5",
        check_relative(-0.01, -0.02, 0.5 - f64::EPSILON)
            .err()
            .unwrap()
    );
    assert_eq!(
        "(left ≉ right) left: -0.1, right: 0.1, gamma: 2",
        check_relative(-0.1, 0.1, 2.0 - f64::EPSILON)
            .err()
            .unwrap()
    );
}