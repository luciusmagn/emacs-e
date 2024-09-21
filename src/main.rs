use clap::{Parser, Subcommand};

use std::path::PathBuf;
use std::process::Command;

/// Run your Emacs in the background
/// easily -- Unbeatable startup times :)
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// edit a file or a folder
    filename: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// start the background emacs session
    Start,
    /// edit a file (default)
    Edit {
        /// edit a file or a folder
        filename: Option<PathBuf>,
    },
    /// restart the session
    Restart,
    /// kill the session -- goodbye :(
    Kill,
}

fn main() {
    let cli = Cli::parse();

    match (cli.filename, cli.command) {
        (_, Some(Commands::Start)) => {
            if Command::new("emacs")
                .arg("--daemon")
                .spawn()
                .is_err()
            {
                eprintln!("failed to start daemon");
            }
        }
        // TODO: Don't be dirty and remember the
        // pid of the emacs daemon we spawned
        (_, Some(Commands::Restart)) => {
            if Command::new("killall")
                .arg("emacs")
                .spawn()
                .is_err()
            {
                eprintln!("failed to stop daemon");
            }
            if Command::new("emacs")
                .arg("--daemon")
                .spawn()
                .is_err()
            {
                eprintln!("failed to start daemon");
            }
        }
        (_, Some(Commands::Kill)) => {
            if Command::new("killall")
                .arg("emacs")
                .spawn()
                .is_err()
            {
                eprintln!("failed to stop daemon");
            }
        }
        (f1, Some(Commands::Edit { filename: f2 })) => {
            let filename = f1.or(f2);

            let filename = match filename {
                // lmao for filenames that
                // are not valid utf-8
                Some(f) => f.display().to_string(),
                None => ".".into(),
            };

            if Command::new("emacsclient")
                .arg("-c")
                .arg(filename)
                .spawn()
                .is_err()
            {
                eprintln!("failed to spawn emacsclient");
            }
        }
        _ => {
            if Command::new("emacsclient")
                .arg("-c")
                .arg(".")
                .spawn()
                .is_err()
            {
                eprintln!("failed to spawn emacsclient");
            }
        }
    }
}
