use eyre::eyre;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

use crate::util::{is_default_browser, set_default_browser};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub notify: bool,
    pub run_command: bool,
    pub command: String,
}

pub fn config_wizard() -> eyre::Result<()> {
    let mut rl = rustyline::Editor::<()>::new()?;

    println!("Configuring Porcupette\n");

    let prompt = if is_default_browser()? {
        "Porcupette is your default browser. Would you still like to set it as your default browser? y/N "
    } else {
        "Porcupette isn't your default browser. Would you like to set it as your default browser? y/N "
    };

    loop {
        let line = rl.readline(prompt)?;
        let line = line.trim();

        if line == "y" || line == "Y" {
            set_default_browser()?;
            break;
        } else if line == "n" || line == "N" || line == "" {
            break;
        }

        println!("Invalid input. Y or N needed");
    }

    let mut config = Config {
        notify: false,
        run_command: false,
        command: String::new(),
    };

    println!("\nWhat should I do with the URL?\n");

    loop {
        let line = rl.readline("1: Copy to clipboard\n2: Run a command\n\nChoose: 1 or 2 ")?;
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
        let line = rl.readline("Should I notify you about it? Y/n ")?;
        let line = line.trim();

        if line == "y" || line == "Y" || line == "" {
            config.notify = true;
            break;
        } else if line == "n" || line == "N" {
            config.notify = false;
            break;
        }

        println!("Invalid input. Y or N needed");
    }

    let f = if cfg!(windows) {
        match OpenOptions::new()
            .write(true)
            .truncate(true)
            .append(false)
            .open("./porcupette.yml")
        {
            Ok(f) => f,
            Err(e) => return Err(e.into()),
        }
    } else {
        let mut home = dirs::home_dir().ok_or(eyre!("home directory couldn't be found"))?;
        home = home.join(".config/porcupette.yml");
        let home = home.to_str().unwrap();

        match OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .append(false)
            .open(home)
        {
            Ok(f) => f,
            Err(e) => return Err(e.into()),
        }
    };

    serde_yaml::to_writer(f, &config)?;
    println!("Configuration was written");
    Ok(())
}

pub fn read_config() -> eyre::Result<Config> {
    let config_path = if cfg!(windows) {
        "./porcupette.yml".to_string()
    } else {
        let mut home = dirs::home_dir().ok_or(eyre!("home directory couldn't be found"))?;
        home = home.join(".config/porcupette.yml");
        home.to_str().unwrap().to_string()
    };
    let config = config::Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()?;

    return Ok(config.try_deserialize::<Config>()?);
}
