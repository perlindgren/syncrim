// An example of a test that should panic (fail)
// Useful to assert that illegal models and/or states does not pass unnoticed
#[test]
#[should_panic(expected = "assertion failed")]
fn should_fail() {
    assert!(false)
}
