//! Collection of assertions and assertion-related helpers.

use std::panic;
use std::path::Path;

use crate::fs as rfs;

/// Gathers all files in the current working directory that have the extension `ext`, and counts
/// the number of lines within that contain a match with the regex pattern `re`.
pub fn count_regex_matches_in_files_with_extension(re: &regex::Regex, ext: &str) -> usize {
    let fetched_files = shallow_find_files(cwd(), |path| has_extension(path, ext));

    let mut count = 0;
    for file in fetched_files {
        let content = rfs::read_to_string(file);
        count += content.lines().filter(|line| re.is_match(&line)).count();
    }

    count
}

/// Read the contents of a file that cannot simply be read by
/// [`read_to_string`][crate::fs::read_to_string], due to invalid UTF-8 data, then assert
/// that it contains `expected`.
#[track_caller]
pub fn invalid_utf8_contains<P: AsRef<Path>, S: AsRef<str>>(path: P, expected: S) {
    let buffer = rfs::read(path.as_ref());
    let expected = expected.as_ref();
    if !String::from_utf8_lossy(&buffer).contains(expected) {
        eprintln!("=== FILE CONTENTS (LOSSY) ===");
        eprintln!("{}", String::from_utf8_lossy(&buffer));
        eprintln!("=== SPECIFIED TEXT ===");
        eprintln!("{}", expected);
        panic!("specified text was not found in file");
    }
}

/// Read the contents of a file that cannot simply be read by
/// [`read_to_string`][crate::fs::read_to_string], due to invalid UTF-8 data, then assert
/// that it does not contain `expected`.
#[track_caller]
pub fn invalid_utf8_not_contains<P: AsRef<Path>, S: AsRef<str>>(path: P, expected: S) {
    let buffer = rfs::read(path.as_ref());
    let expected = expected.as_ref();
    if String::from_utf8_lossy(&buffer).contains(expected) {
        eprintln!("=== FILE CONTENTS (LOSSY) ===");
        eprintln!("{}", String::from_utf8_lossy(&buffer));
        eprintln!("=== SPECIFIED TEXT ===");
        eprintln!("{}", expected);
        panic!("specified text was unexpectedly found in file");
    }
}

/// Assert that `actual` is equal to `expected`.
#[track_caller]
pub fn assert_equals<A: AsRef<str>, E: AsRef<str>>(actual: A, expected: E) {
    let actual = actual.as_ref();
    let expected = expected.as_ref();
    if actual != expected {
        eprintln!("=== ACTUAL TEXT ===");
        eprintln!("{}", actual);
        eprintln!("=== EXPECTED ===");
        eprintln!("{}", expected);
        panic!("expected text was not found in actual text");
    }
}

/// Assert that `haystack` contains `needle`.
#[track_caller]
pub fn assert_contains<H: AsRef<str>, N: AsRef<str>>(haystack: H, needle: N) {
    let haystack = haystack.as_ref();
    let needle = needle.as_ref();
    if !haystack.contains(needle) {
        eprintln!("=== HAYSTACK ===");
        eprintln!("{}", haystack);
        eprintln!("=== NEEDLE ===");
        eprintln!("{}", needle);
        panic!("needle was not found in haystack");
    }
}

/// Assert that `haystack` does not contain `needle`.
#[track_caller]
pub fn assert_not_contains<H: AsRef<str>, N: AsRef<str>>(haystack: H, needle: N) {
    let haystack = haystack.as_ref();
    let needle = needle.as_ref();
    if haystack.contains(needle) {
        eprintln!("=== HAYSTACK ===");
        eprintln!("{}", haystack);
        eprintln!("=== NEEDLE ===");
        eprintln!("{}", needle);
        panic!("needle was unexpectedly found in haystack");
    }
}

/// Assert that all files in `dir1` exist and have the same content in `dir2`
pub fn assert_recursive_eq(dir1: impl AsRef<Path>, dir2: impl AsRef<Path>) {
    let dir2 = dir2.as_ref();
    rfs::read_dir_entries(dir1, |entry_path| {
        let entry_name = entry_path.file_name().unwrap();
        if entry_path.is_dir() {
            assert_recursive_eq(&entry_path, &dir2.join(entry_name));
        } else {
            let path2 = dir2.join(entry_name);
            let file1 = rfs::read(&entry_path);
            let file2 = rfs::read(&path2);

            // We don't use `assert_eq!` because they are `Vec<u8>`, so not great for display.
            // Why not using String? Because there might be minified files or even potentially
            // binary ones, so that would display useless output.
            assert!(
                file1 == file2,
                "`{}` and `{}` have different content",
                entry_path.display(),
                path2.display(),
            );
        }
    });
}
