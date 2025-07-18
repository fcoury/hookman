use std::fs;
use tempfile::TempDir;

// Helper function to create a temporary git repository
fn setup_test_repo() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let git_dir = temp_dir.path().join(".git");
    fs::create_dir_all(&git_dir).unwrap();
    fs::create_dir_all(git_dir.join("hooks")).unwrap();
    temp_dir
}

#[test]
fn test_hookman_workflow() {
    let temp_dir = setup_test_repo();
    let repo_path = temp_dir.path();

    // Test init command
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hookman"))
        .current_dir(repo_path)
        .arg("init")
        .output()
        .expect("Failed to execute init command");

    if !output.status.success() {
        eprintln!("Init command failed!");
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    assert!(output.status.success());
    assert!(repo_path.join(".hookman").exists());
    assert!(repo_path.join(".hookman/hooks").exists());
    assert!(repo_path.join(".hookman/config.toml").exists());

    // Test add command
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hookman"))
        .current_dir(repo_path)
        .args(&[
            "add",
            "pre-commit",
            "echo 'test'",
            "--id",
            "test",
            "--description",
            "Test hook",
        ])
        .output()
        .expect("Failed to execute add command");

    assert!(output.status.success());

    // Verify the hook was added
    let hook_file = repo_path.join(".hookman/hooks/pre-commit.toml");
    assert!(hook_file.exists());
    let content = fs::read_to_string(&hook_file).unwrap();
    assert!(content.contains("id = \"test\""));
    assert!(content.contains("command = \"echo 'test'\""));
    assert!(content.contains("description = \"Test hook\""));

    // Test list command
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hookman"))
        .current_dir(repo_path)
        .arg("list")
        .output()
        .expect("Failed to execute list command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pre-commit"));
    assert!(stdout.contains("[test]"));
    assert!(stdout.contains("echo 'test'"));

    // Test apply command
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hookman"))
        .current_dir(repo_path)
        .arg("apply")
        .output()
        .expect("Failed to execute apply command");

    assert!(output.status.success());

    // Verify the git hook was created
    let git_hook = repo_path.join(".git/hooks/pre-commit");
    assert!(git_hook.exists());
    let content = fs::read_to_string(&git_hook).unwrap();
    assert!(content.contains("Generated by hookman"));
    assert!(content.contains("echo 'test'"));

    // Verify the hook is executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&git_hook).unwrap();
        let permissions = metadata.permissions();
        assert_eq!(permissions.mode() & 0o111, 0o111); // Check execute bits
    }

    // Test remove command
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hookman"))
        .current_dir(repo_path)
        .args(&["remove", "pre-commit", "test"])
        .output()
        .expect("Failed to execute remove command");

    assert!(output.status.success());

    // Verify the hook was removed
    let hook_file = repo_path.join(".hookman/hooks/pre-commit.toml");
    let content = fs::read_to_string(&hook_file).unwrap();
    assert!(!content.contains("id = \"test\""));
}

#[test]
fn test_not_in_git_repo() {
    let temp_dir = TempDir::new().unwrap();

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hookman"))
        .current_dir(temp_dir.path())
        .arg("init")
        .output()
        .expect("Failed to execute init command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Not in a Git repository"));
}

#[test]
fn test_hookman_not_initialized() {
    let temp_dir = setup_test_repo();

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_hookman"))
        .current_dir(temp_dir.path())
        .args(&["add", "pre-commit", "echo test", "--id", "test"])
        .output()
        .expect("Failed to execute add command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Hookman not initialized"));
}
