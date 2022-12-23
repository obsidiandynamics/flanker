use crate::Lazy;

#[test]
fn lifecycle() {
    let mut lazy = Lazy::new(|| 42);
    assert_eq!(None, lazy.try_get());
    assert_eq!(42, *lazy.get());
    assert_eq!(Some(&42), lazy.try_get());
}

#[test]
fn default() {
    let mut lazy = crate::default::<i32>();
    assert_eq!(None, lazy.try_get());
    assert_eq!(0, *lazy.get());
}