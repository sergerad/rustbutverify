use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn cycle_stdout_exit_code() {
    // Create a command to run the binary with cargo run.
    let mut child = Command::new("cargo")
        .args(["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Write to stdin.
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"A\tB\nB\tA\n").unwrap();
    }

    // Get the output.
    let output = child.wait_with_output().unwrap();

    // Check the exit code.
    assert_eq!("exit status: 1", output.status.to_string());

    // Check the stdout.
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout != "A\nB\n" && stdout != "B\nA\n" {
        panic!("stdout is wrong, got: {}", stdout);
    }
}

#[test]
fn no_cycle_stdout_exit_code() {
    // Create a command to run the binary with cargo run.
    let mut child = Command::new("cargo")
        .args(["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Write to stdin.
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"A\tB\nB\tC\n").unwrap();
    }

    // Get the output.
    let output = child.wait_with_output().unwrap();

    // Check the exit code.
    assert_eq!("exit status: 0", output.status.to_string());

    // Check the stdout.
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!("", stdout);
}

#[test]
fn invalid_input_stdout_exit_code() {
    // Create a command to run the binary with cargo run.
    let mut child = Command::new("cargo")
        .args(["run"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // Write to stdin.
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"\n\t\n\tA\tB\nB\tC\n").unwrap();
    }

    // Get the output.
    let output = child.wait_with_output().unwrap();

    // Check the exit code.
    assert_eq!("exit status: 2", output.status.to_string());

    // Check the stdout.
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!("", stdout);
}
