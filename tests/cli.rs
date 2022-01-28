/* Test ideas

[x] What happens when the file doesn’t exist?
[x] What is the output when there is no match?
[ ] Does our program exit with an error when we forget one (or both) arguments?
[x] Happy path
*/

use assert_cmd::prelude::*; // methods on commands
use predicates::prelude::*; // used for assertions
use std::process::Command; // run programs
use assert_fs::prelude::*; // use to create temp files
use std::error::Error;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert().failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn no_arguments() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.assert().failure()
        .stderr(predicate::str::contains("The following required arguments were not provided:"));

    Ok(())
}

#[test]
fn pattern_is_empty_string() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    let file = create_sample_file()?;

    cmd.arg("").arg(file.path());
    cmd.assert().success()
        .stderr(predicate::str::contains(""));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    let file = create_sample_file()?;

    cmd.arg("test").arg(file.path());
    cmd.assert().success()
        .stdout(predicate::str::contains("test\nAnother test"));

    Ok(())
}

fn create_sample_file() -> Result<assert_fs::NamedTempFile, Box<dyn Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    Ok(file)
}