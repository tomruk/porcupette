use eyre::eyre;
use std::{fs::OpenOptions, io::Write, process::Command};

// For debugging purposes
#[allow(dead_code)]
pub fn write_args() {
    let mut f = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open("/tmp/args")
        .unwrap();

    let args = std::env::args();
    let args: Vec<String> = args.collect();

    f.write_all(args.join(" ").as_bytes()).unwrap();
    f.write_all(b"\n\n").unwrap();
}

pub fn is_default_browser() -> eyre::Result<bool> {
    let output = Command::new("xdg-settings")
        .args(["get", "default-web-browser"])
        .output()?;
    let output = String::from_utf8(output.stdout)?;
    Ok(output.find("porcupette.desktop").is_some())
}

pub fn set_default_browser() -> eyre::Result<()> {
    let output = Command::new("xdg-settings")
        .args(["set", "default-web-browser", "porcupette.desktop"])
        .output()?;
    let status = output.status;

    if !status.success() {
        let stderr = String::from_utf8(output.stderr)?;
        return Err(eyre!("xdg-settings failed:\n{stderr}\n"));
    }
    Ok(())
}
