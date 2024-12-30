use std::path::Path;

fn list_rs<P: AsRef<Path>>(p: P) -> Vec<std::fs::DirEntry> {
    let mut entries = std::fs::read_dir(p)
        .unwrap()
        .filter_map(|a| {
            a.ok().and_then(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| (ext == "rs").then_some(e))
            })
        })
        .collect::<Vec<_>>();
    entries.sort_by_key(|a| a.file_name());
    entries
}

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    let tests_dir = Path::new("tests");
    let successes = tests_dir.join("successes");
    let successes = list_rs(successes);
    for entry in successes {
        t.pass(entry.path());
    }

    let failures = tests_dir.join("failures");
    let failures = list_rs(failures);
    for entry in failures {
        t.compile_fail(entry.path());
    }
}
