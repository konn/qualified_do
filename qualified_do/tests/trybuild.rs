#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/case01.rs");
    t.pass("tests/case02.rs");
    t.pass("tests/case03.rs");
    t.pass("tests/case04.rs");
    t.pass("tests/case05.rs");
    t.compile_fail("tests/error_case01.rs");
    t.compile_fail("tests/error_case02.rs");
}
