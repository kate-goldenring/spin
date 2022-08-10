use std::process::Stdio;
use anyhow::{anyhow,Result};
use tokio::process::Command;
use tracing::log;

use crate::commands::plugins::get_spin_plugins_directory;

// TODO: Add capability to distinguish between standalone binaries and pluigns
// TODO: Should this be a struct to maintain consistency across subcommands?

pub async fn execute_external_subcommand(args: Vec<String>) -> Result<()> {
    // TODO: What environmental variables should be passed.

    let path = get_spin_plugins_directory()?
        .join(args.first().unwrap())
        .join(args.first().unwrap());
    let mut command = Command::new(path);
    command.stdin(Stdio::inherit())
        .stdout(Stdio::inherit());
    if args.len() > 1 {
        command.args(&args[1..]);
    }
    log::info!("Executing command {:?}", command);
    let clone_result = command.output().await?;
    // match clone_result {
    //     Ok(_) => println!("Was spawned :)"),
    //     Err(e) => {
    //         eprintln!("External subcommand failed: {}", e.kind());
    //     }
    // }
    // Ok(())
    match clone_result.status.success() {
        true => {
            // TODO: remove later
            println!("Successful executing command");
            Ok(())
        }
        false => Err(anyhow!(
            "Error executing command {:?}: {}",
            command,
            String::from_utf8(clone_result.stderr)
                .unwrap_or_else(|_| "(cannot get error)".to_owned())
        )),
    }
}
