use std::error::Error;
use std::fs;
use assert_cmd::Command;
// use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("_echor")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("_echor")?;
    cmd.arg("hello")
        .assert()
        .success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let outfile = "./tests/expected/hello1.txt";
    run(&["Hello there"], outfile)
}

#[test]
fn hello1_with_no_newline() -> TestResult {
    let outfile = "./tests/expected/hello1.n.txt";
    run(&["Hello  there", "-n"], outfile)
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "./tests/expected/hello2.txt")
}

#[test]
fn hello2_with_no_newline() -> TestResult {
    let outfile = "./tests/expected/hello2.n.txt";
    run(&["-n", "Hello", "there"], outfile)
}

fn run(args: &[&str], input_file_name: &str) -> Result<(), Box<dyn Error>> {
    let expected = fs::read_to_string(input_file_name)?;
    let mut cmd = Command::cargo_bin("_echor")?;
    cmd.args(args).assert()
        .success()
        .stdout(expected);
    Ok(())
}