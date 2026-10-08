use super::{json, packlist, tempdir, touch};

#[test]
fn files_field_does_not_force_include_changelog_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    touch(root, "package.json");
    touch(root, "dist/index.js");
    touch(root, "CHANGES.md");
    touch(root, "CHANGELOG.md");
    touch(root, "HISTORY.md");
    touch(root, "NOTICE.md");

    let manifest = json!({
        "name": "x",
        "version": "0.0.0",
        "files": ["dist/**"],
    });
    let mut out = packlist(root, &manifest).unwrap();
    out.sort();

    assert_eq!(out, vec!["dist/index.js".to_string(), "package.json".into()]);
}

#[test]
fn files_field_does_not_force_include_files_that_only_start_with_readme_or_license() {
    let dir = tempdir().unwrap();
    let root = dir.path();
    touch(root, "package.json");
    touch(root, "dist/index.js");
    touch(root, "README.md");
    touch(root, "README.md~");
    touch(root, "README_INTERNAL.md");
    touch(root, "LICENSE");
    touch(root, "LICENSE-MIT");
    touch(root, "licence.txt");

    let manifest = json!({
        "name": "x",
        "version": "0.0.0",
        "files": ["dist/**"],
    });
    let mut out = packlist(root, &manifest).unwrap();
    out.sort();

    assert_eq!(
        out,
        vec![
            "LICENSE".to_string(),
            "README.md".into(),
            "dist/index.js".into(),
            "licence.txt".into(),
            "package.json".into(),
        ],
    );
}
