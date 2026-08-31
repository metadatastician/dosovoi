// SPDX-License-Identifier: MPL-2.0

use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

fn sandbox() -> (PathBuf, PathBuf) {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock after epoch")
        .as_nanos();
    let directory =
        std::env::temp_dir().join(format!("dosovoi-cli-test-{}-{unique}", std::process::id()));
    fs::create_dir_all(&directory).expect("create test sandbox");
    let config = directory.join("config");
    (directory, config)
}

fn run(config: &PathBuf, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_dosovoi"));
    command.arg("--config").arg(config).args(args);
    command.output().expect("run dosovoi")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("UTF-8 output")
}

#[test]
fn help_documents_non_agency_boundary_and_commands() {
    let output = Command::new(env!("CARGO_BIN_EXE_dosovoi"))
        .arg("help")
        .output()
        .expect("run help");
    assert!(output.status.success());
    let text = stdout(&output);
    assert!(text.contains("deterministic, non-agentic"));
    assert!(text.contains("signal <caution|high> --reason TEXT"));
    assert!(text.contains("Mascot state is not a safety"));
}

#[test]
fn representative_commands_persist_warning_across_mode_and_care() {
    let (directory, config) = sandbox();

    let preview = run(
        &config,
        &[
            "preview",
            "--pet",
            "puff",
            "--mode",
            "detachment",
            "--ascii",
        ],
    );
    assert!(preview.status.success());
    assert!(stdout(&preview).contains("Puff mascot in a resting posture"));

    let signal = run(
        &config,
        &[
            "signal",
            "caution",
            "--reason",
            "Review generated command before execution",
        ],
    );
    assert!(signal.status.success());
    assert!(stdout(&signal).starts_with("WARNING [CAUTION]"));
    assert!(stdout(&signal).contains("Signal source: explicit user input"));

    assert!(run(&config, &["mode", "flow"]).status.success());
    let care = run(&config, &["care", "on"]);
    assert!(care.status.success());
    let care_text = stdout(&care);
    assert!(care_text.starts_with("WARNING [CAUTION]"));
    assert!(care_text.contains("Mode: care"));

    let status = run(&config, &["status"]);
    let status_text = stdout(&status);
    assert!(status_text.contains("Review generated command before execution"));
    assert!(status_text.contains("Care: on"));

    fs::remove_dir_all(directory).expect("remove isolated test sandbox");
}

#[test]
fn unsignalled_and_high_risk_outputs_preserve_the_evidence_boundary() {
    let (directory, config) = sandbox();
    let flow = run(&config, &["preview", "--mode", "flow"]);
    assert!(flow.status.success());
    assert!(stdout(&flow)
        .starts_with("Risk signal: none supplied; no safety assessment performed.\n\n"));

    let high = run(
        &config,
        &[
            "signal",
            "high",
            "--reason",
            "Separate system reported a high-risk condition",
        ],
    );
    assert!(high.status.success());
    let high_text = stdout(&high);
    assert!(high_text.starts_with("WARNING [HIGH RISK]"));
    assert!(high_text.contains("Signal source: explicit user input\n\n"));
    assert!(!high_text.contains("(\\_/)"));
    fs::remove_dir_all(directory).expect("remove isolated test sandbox");
}

#[test]
fn high_signal_requires_a_reason() {
    let (directory, config) = sandbox();
    let output = run(&config, &["signal", "high"]);
    assert!(!output.status.success());
    assert!(String::from_utf8(output.stderr)
        .expect("UTF-8 error")
        .contains("requires --reason"));
    fs::remove_dir_all(directory).expect("remove isolated test sandbox");
}

#[test]
fn no_color_overrides_configuration() {
    let (directory, config) = sandbox();
    fs::write(
        &config,
        "enabled=true\nmascot=lens\nrender=plain\ncolor=true\nmotion=false\n\
         default_mode=serenity\ncare_plain_text=true\ncare_disable_color=true\n",
    )
    .expect("write test config");
    let output = Command::new(env!("CARGO_BIN_EXE_dosovoi"))
        .env("NO_COLOR", "1")
        .arg("--config")
        .arg(&config)
        .args(["signal", "high", "--reason", "Named signal"])
        .output()
        .expect("run dosovoi");
    let text = stdout(&output);
    assert!(text.starts_with("WARNING [HIGH RISK]"));
    assert!(!text.contains('\u{1b}'));
    fs::remove_dir_all(directory).expect("remove isolated test sandbox");
}

#[test]
fn disabling_is_immediate_but_does_not_erase_active_warning() {
    let (directory, config) = sandbox();
    assert!(run(
        &config,
        &["signal", "high", "--reason", "Separate system signal"]
    )
    .status
    .success());
    let output = run(&config, &["disable"]);
    let text = stdout(&output);
    assert!(text.starts_with("WARNING [HIGH RISK]"));
    assert!(text.contains("Mascot status: disabled."));
    assert!(!text.to_ascii_lowercase().contains("come back"));
    fs::remove_dir_all(directory).expect("remove isolated test sandbox");
}
