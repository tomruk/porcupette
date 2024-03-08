use eyre::{eyre, Context};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

use crate::util::{is_default_browser, set_default_browser};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub notify: bool,
    pub print: bool,
    pub run_command: bool,
    pub command: String,
}

pub fn config_wizard() -> eyre::Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;

    println!("Configuring Porcupette\n");

    match is_default_browser() {
        Ok(v) => {
            let (prompt, default_yes) = if v {
                ("Porcupette is your default browser. Would you still like to set it as your default browser? y/N ", false)
            } else {
                ("Porcupette is not your default browser. Would you like to set it as your default browser? Y/n ", true)
            };

            loop {
                let line = rl.readline(prompt)?;
                let line = line.trim();

                if line == "y" || line == "Y" || (line == "" && default_yes) {
                    set_default_browser()?;
                    break;
                } else if line == "n" || line == "N" || (line == "" && !default_yes) {
                    break;
                }

                println!("\nInvalid input. y or n needed:");
            }
        }
        Err(e) => {
            eprintln!(
                "Couldn't determine whether Porcupette is set as the default browser or not."
            );
            eprintln!("Error: {:?}", e);
            eprintln!("Try to manually set the porcupette executable as your default browser. If you cannot, please open an issue on GitHub.\n");
        }
    }

    let mut config = Config {
        notify: false,
        print: false,
        run_command: false,
        command: String::new(),
    };

    println!("\nWhat should I do with the URL?\n");

    loop {
        let line = rl.readline("1: Copy to clipboard\n2: Run a command\n\n1 or 2: ")?;
        let line = line.trim();

        if line == "1" {
            break;
        } else if line == "2" {
            let line = loop {
                let line = rl.readline("Command to run (Use %U for URL substitution): ")?;
                if let Some(_) = line.find("%U") {
                    break line;
                }
                println!("%U was not found in the command. Type it again.");
            };
            config.run_command = true;
            config.command = line;
            break;
        }

        println!("Invalid input. 1 or 2 needed");
    }

    loop {
        let line = rl.readline("How should I notify you about it? B/N/P/B\n\
        B: Both\n\
        N: Notify -> Notify, but don't print to console. Prevents Porcupette's stdout/stderr from conflicting with other programs (since it can get executed from CLI programs).\n\
        P: Print -> If notifications don't appear, this might be good for debugging.\n\
        I: Neither\n")?;

        match line.trim() {
            "B" | "b" => {
                config.notify = true;
                config.print = true;
            }
            "N" | "n" => config.notify = true,
            "P" | "p" => config.print = true,
            "I" | "i" => {}
            _ => {
                println!("Invalid input. N, P, or B needed.");
            }
        }
        break;
    }

    let f = if cfg!(windows) {
        let config_path = "./porcupette.yml";
        match OpenOptions::new()
            .write(true)
            .truncate(true)
            .append(false)
            .open(config_path)
            .wrap_err(format!("Failed to create/open {config_path}"))
        {
            Ok(f) => f,
            Err(e) => return Err(e.into()),
        }
    } else {
        let home = dirs::home_dir().ok_or(eyre!("Home directory couldn't be found"))?;
        let config_path = home.join(".config/porcupette.yml");
        let config_path = config_path.to_str().unwrap();

        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .append(false)
            .open(config_path)
            .wrap_err_with(|| format!("Failed to create/open: {config_path}"))
        {
            Ok(f) => f,
            Err(e) => return Err(e.into()),
        }
    };

    serde_yaml::to_writer(f, &config)?;
    println!("Configuration is written");
    Ok(())
}

pub fn read_config() -> eyre::Result<Config> {
    let config_path = if cfg!(windows) {
        "./porcupette.yml".to_string()
    } else {
        let home = dirs::home_dir().ok_or(eyre!("Home directory couldn't be found"))?;
        home.join(".config/porcupette.yml")
            .to_str()
            .unwrap()
            .to_string()
    };
    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()?;

    return Ok(config.try_deserialize::<Config>()?);
}
