use std::{fs, path::PathBuf};

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    std::process::Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|output| !output.is_empty())
}

fn command_output_or_unknown(program: &str, args: &[&str]) -> String {
    command_output(program, args).unwrap_or_else(|| "unknown".to_string())
}

fn current_release_url(version: &str, repository: &str) -> Option<String> {
    let expected_tag = format!("v{version}");
    let tags = command_output("git", &["tag", "--points-at", "HEAD"])?;
    let has_expected_tag = tags.lines().any(|tag| tag.trim() == expected_tag);

    has_expected_tag.then(|| format!("{repository}/-/releases/tag/{expected_tag}"))
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=PROFILE");
    println!("cargo:rerun-if-env-changed=RUSTC");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
    println!("cargo:rerun-if-changed=.git/packed-refs");

    // 目标平台
    let target = std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=TARGET={target}");

    // 构建 profile
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_PROFILE={profile}");

    // Git commit hash（短）
    let git_hash = command_output_or_unknown("git", &["rev-parse", "--short=7", "HEAD"]);
    println!("cargo:rustc-env=GIT_HASH={git_hash}");

    // Git commit 日期
    let git_date = command_output_or_unknown("git", &["log", "-1", "--format=%cs"]);
    println!("cargo:rustc-env=GIT_DATE={git_date}");

    // rustc 版本
    let rustc_version = command_output_or_unknown("rustc", &["--version"]);
    println!("cargo:rustc-env=RUSTC_VERSION_INFO={rustc_version}");

    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
    let repository = std::env::var("CARGO_PKG_REPOSITORY").unwrap_or_default();

    let short_version = format!("{version} ({git_hash} {git_date})");

    let mut detail_lines = vec![
        rustc_version,
        format!("target: {target}"),
        format!("profile: {profile}"),
    ];

    if let Some(release_url) = current_release_url(&version, &repository) {
        detail_lines.push(release_url);
    }

    let clap_long_version = format!("{short_version}\n{}", detail_lines.join("\n"));
    let command_long_version = format!("cnb-rs {short_version}\n{}", detail_lines.join("\n"));

    let out_dir = match std::env::var("OUT_DIR") {
        Ok(out_dir) => PathBuf::from(out_dir),
        Err(err) => panic!("OUT_DIR is set by cargo: {err}"),
    };
    let version_info = format!(
        "pub const SHORT_VERSION: &str = {short_version:?};\n\
         pub const CLAP_LONG_VERSION: &str = {clap_long_version:?};\n\
         pub const COMMAND_LONG_VERSION: &str = {command_long_version:?};\n"
    );

    if let Err(err) = fs::write(out_dir.join("version_info.rs"), version_info) {
        panic!("failed to write generated version info: {err}");
    }
}
