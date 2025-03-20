use std::env;
use std::process::{exit, Command, Stdio};

use log::debug;

fn main() {
    let mut args = env::args();
    debug!("executing; cmd=albino; args={:?}", args);

    let cmd = args.nth(1).unwrap_or_else(|| "--help".into());
    let args = args.collect::<Vec<_>>();

    match cmd.as_str() {
        "--help" | "-h" | "help" | "-?" => {
            println!("Commands:");
            println!("  build          # compile the source code file");
            println!("  exec           # execute the bytecode file");
            println!("  run            # build and execute");
            println!("");
        }
        "--version" | "-v" | "version" => {
            println!(
                "albino {}, whitebase {}",
                albino::version(),
                whitebase::version()
            );
        }
        _ => {
            let current_exe = env::current_exe();
            let exe_suffix = current_exe
                .as_ref()
                .ok()
                .and_then(|name| name.extension())
                .and_then(|exe| exe.to_str())
                .unwrap_or_default();
            let command = format!("albino-{}{}", cmd, exe_suffix);
            let mut command = match current_exe {
                Ok(path) => {
                    let p = path.parent().unwrap_or(&path).join(&command);
                    if p.exists() {
                        Command::new(p)
                    } else {
                        Command::new(&command)
                    }
                }
                Err(_) => Command::new(&command),
            };
            let command = command
                .args(args.as_slice())
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            match command {
                Ok(status) if status.success() => {}
                Ok(status) => exit(status.code().unwrap_or(127)),
                Err(err) => {
                    println!("{err}");
                    exit(127);
                }
            }
        }
    }
}
