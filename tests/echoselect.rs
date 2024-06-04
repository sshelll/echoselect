use assert_cmd::Command;

#[test]
fn test_empty_input_by_call() {
    Command::cargo_bin("echoselect")
        .unwrap()
        .args(&["-d a"])
        .assert()
        .stderr("no input provided\n")
        .code(1);
}
