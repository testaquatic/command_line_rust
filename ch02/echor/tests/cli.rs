use std::fs;

use assert_cmd::Command;

fn run(args: &[&str], expected_file: &str) -> Result<(), anyhow::Error> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("fail");

    let stdout: String = output.stdout.try_into()?;
    assert_eq!(stdout, expected);

    Ok(())
}

/// echor
/// "Usage"가 포함된 간단한 안내를 표시해야 한다.
#[test]
fn dies_no_runs() -> Result<(), anyhow::Error> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));

    Ok(())
}

/// echor hello
/// 인수가 주어졌을 때 성공적으로 종료해야 한다.
#[test]
fn runs() -> Result<(), anyhow::Error> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();

    Ok(())
}

/// echor "Hello there"
#[test]
fn hello() -> Result<(), anyhow::Error> {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

/// echor Hello there
#[test]
fn hello2() -> Result<(), anyhow::Error> {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

/// echor "Hello  there" -n
#[test]
fn hello1_no_newline() -> Result<(), anyhow::Error> {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

/// echor -n Hello there
#[test]
fn hello2_no_newline() -> Result<(), anyhow::Error> {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
