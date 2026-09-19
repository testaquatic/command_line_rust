use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn true_ok() -> Result<(), anyhow::Error> {
    let mut cmd = Command::cargo_bin("true")?;
    cmd.assert().success();

    Ok(())
}

#[test]
fn false_not_ok() -> Result<(), anyhow::Error> {
    let mut cmd = Command::cargo_bin("false")?;
    cmd.assert().failure();

    Ok(())
}

#[test]
fn runs() -> Result<(), anyhow::Error> {
    let mut cmd = Command::cargo_bin("hello")?;
    let output = cmd.output()?;
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout)?;
    assert_eq!(stdout, "Hello, world!\n");

    Ok(())
}
