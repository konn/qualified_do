use std::path::Path;

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    let tests_dir = Path::new("tests");
    let successes = tests_dir.join("successes");

    let mut successes = successes
        .read_dir()
        .unwrap()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    successes.sort_by_key(|a| a.file_name());
    for entry in successes {
        t.pass(entry.path());
    }

    let failures = tests_dir.join("failures");
    let mut failures = failures
        .read_dir()
        .unwrap()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    failures.sort_by_key(|a| a.file_name());
    for entry in failures {
        t.compile_fail(entry.path());
    }
}
