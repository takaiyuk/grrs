use anyhow::Result;
use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::fixture::FileWriteStr; // Write files
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<()> {
    // When we run our program with a file that doesn't exist, we should exit with a non-zero.
    let mut cmd = Command::cargo_bin("takaiyuk-grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<()> {
    // When we run grrs with a file that contains the pattern, it should print the lines that.
    let file = assert_fs::NamedTempFile::new("find_content_in_file.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("takaiyuk-grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn find_empty_string_in_file() -> Result<()> {
    // When we run grrs with an empty string as the pattern, it should print all lines in the file.
    let file = assert_fs::NamedTempFile::new("find_empty_string_in_file.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("takaiyuk-grrs")?;
    cmd.arg("").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains(
        "A test\nActual content\nMore content\nAnother test",
    ));

    Ok(())
}
