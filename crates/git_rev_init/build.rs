fn git_rev(len: usize) -> String {
    std::str::from_utf8(
        &std::process::Command::new("git")
            .args(["rev-parse", &format!("--short={}", len), "HEAD"])
            .output()
            .expect("Failed to call `git rev-parse ...`")
            .stdout,
    )
    .expect("Invalid git revision encoding")
    .trim()
    .to_string()
}

fn git_dir() -> String {
    std::str::from_utf8(
        &std::process::Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .output()
            .expect("Failed to call `git rev-parse --git-dir`")
            .stdout,
    )
    .expect("Invalid path encoding")
    .trim()
    .to_string()
}

fn main() {
    // Container builds receive a source archive without the checkout's .git directory.
    println!("cargo:rerun-if-env-changed=AMBIENT_BUILD_REVISION");
    if let Ok(revision) = std::env::var("AMBIENT_BUILD_REVISION") {
        assert!(revision.len() == 40 && revision.bytes().all(|c| c.is_ascii_hexdigit()));
        let marker =
            std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("source-revision");
        std::fs::write(&marker, &revision).unwrap();
        println!("cargo:rustc-env=REV_FULL={revision}");
        println!("cargo:rustc-env=REV_10={}", &revision[..10]);
        println!("cargo:rustc-env=GIT_LOG_HEAD={}", marker.display());
        return;
    }
    println!("cargo:rustc-env=REV_FULL={}", git_rev(40));
    println!("cargo:rustc-env=REV_10={}", git_rev(10));
    println!(
        "cargo:rerun-if-changed={}",
        std::path::Path::new(&git_dir()).join("logs/HEAD").display()
    );
    // This is a hack to get cargo to rebuild when the git log changes
    // Also generating the path here so it's platform independent (include_bytes! expects a platform-specific path!)
    println!(
        "cargo:rustc-env=GIT_LOG_HEAD={}",
        std::path::Path::new(&git_dir()).join("logs/HEAD").display()
    );
}
