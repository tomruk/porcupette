use color_eyre::config::HookBuilder;
use copypasta::ClipboardProvider;
use eyre::eyre;
use notify_rust::Notification;
use rustyline::error::ReadlineError;
use std::thread::sleep;
use std::{process::exit, time::Duration};

use crate::config::{config_wizard, read_config};

mod config;
mod util;

fn main() {
    HookBuilder::new()
        .display_env_section(false)
        .display_location_section(false)
        .install()
        .unwrap();

    let url = std::env::args().nth(1).unwrap_or_else(|| {
        if let Err(e) = config_wizard() {
            if let Some(e) = e.downcast_ref::<ReadlineError>() {
                match e {
                    ReadlineError::Eof | ReadlineError::Interrupted => {
                        println!("Canceled");
                        exit(0);
                    }
                    _ => {}
                }
            }

            eprintln!("Error: {:?}", e);
            exit(1);
        }
        exit(0);
    });

    let config = match read_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            exit(1);
        }
    };

    let notify_short = Duration::from_secs(2);
    let notify_long = Duration::from_secs(12);

    let notify = |prompt: &str, timeout: Duration| {
        println!("{prompt}");
        if config.notify {
            Notification::new()
                .appname("Porcupette")
                .body(prompt)
                .timeout(timeout)
                .show()
                .unwrap();
        }
    };

    let enotify = |prompt: &str, timeout: Duration| {
        eprintln!("{prompt}");
        if config.notify {
            Notification::new()
                .appname("Porcupette")
                .summary("Error!")
                .body(prompt)
                .timeout(timeout)
                .show()
                .unwrap();
        }
    };

    if !is_http_or_file(&url) {
        enotify(&format!("The protocol of the URL '{url}' is not one of these types:\nhttp://\nhttps://\nfile://\n\nNo operation was done"), notify_long);
        exit(1);
    }

    if config.run_command {
        if let Err(e) = run_command(url, config.command) {
            enotify(
                &format!("Error while executing the command: {e}"),
                notify_long,
            );
            exit(2);
        };
        notify("Command execution was successful", notify_short);
    } else {
        copy_to_clipboard(url).unwrap_or_else(|e| {
            enotify(
                &format!("Error while copying to clipboard: {e}"),
                notify_long,
            );
            exit(2);
        });
        notify("Copied to clipboard", notify_short);
    }
}

fn run_command(url: String, mut command: String) -> eyre::Result<()> {
    println!("Running: {}", command);
    command.find("%U").ok_or(eyre!("%U wasn't found"))?;
    command = command.replacen("%U", &url, 1);

    let exit_status = execute::command(command).status()?;
    if !exit_status.success() {
        if let Some(code) = exit_status.code() {
            return Err(eyre!("Command was quit with status code {code}"));
        }
        return Err(eyre!(
            "Command was quit but status code couldn't be retrieved"
        ));
    }
    Ok(())
}

fn copy_to_clipboard(url: String) -> eyre::Result<()> {
    let mut c = copypasta::ClipboardContext::new().map_err(|e| eyre!(e))?;
    c.set_contents(url).map_err(|e| eyre!(e))?;

    c.get_contents();
    sleep(Duration::from_millis(600));
    Ok(())
}

fn is_http_or_file(url: &str) -> bool {
    let url = url.to_lowercase();

    // In Porcupine, file:/// protocol is omitted, but Porcupette doesn't do that.
    // An attacker might be trying you to execute a local file (somehow) downloaded from web. This seems very unlikely, but still a consideration.

    if url.starts_with("http://") || url.starts_with("https://") || url.starts_with("file://") {
        return true;
    }
    false
}
