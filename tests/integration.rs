use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn pipeline_preserves_all_non_blank_lines() {
    let input = "apple\nbanana\n\norange\ncar\ntruck\n";

    let mut child = Command::new(env!("CARGO_BIN_EXE_semanticat"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn semanticat");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();

    for expected_line in ["apple", "banana", "orange", "car", "truck"] {
        assert!(
            stdout.contains(expected_line),
            "missing line: {expected_line}"
        );
    }
}

#[test]
fn verbose_flag_enables_debug_logging_on_stderr() {
    let input = "apple\nbanana\n\norange\ncar\ntruck\n";

    let mut child = Command::new(env!("CARGO_BIN_EXE_semanticat"))
        .arg("-v")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn semanticat");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("DEBUG"),
        "expected debug-level logs on stderr with -v, got: {stderr}"
    );
}

#[test]
fn double_verbose_flag_enables_trace_logging_on_stderr() {
    let input = "apple\nbanana\n\norange\ncar\ntruck\n";

    let mut child = Command::new(env!("CARGO_BIN_EXE_semanticat"))
        .arg("-vv")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn semanticat");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("TRACE"),
        "expected trace-level logs on stderr with -vv, got: {stderr}"
    );
}

#[test]
fn default_verbosity_omits_debug_logging_on_stderr() {
    let input = "apple\nbanana\n\norange\ncar\ntruck\n";

    let mut child = Command::new(env!("CARGO_BIN_EXE_semanticat"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn semanticat");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        !stderr.contains("DEBUG"),
        "did not expect debug-level logs on stderr by default, got: {stderr}"
    );
}
