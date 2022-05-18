#![allow(unused_must_use)]
use clap::Parser;
use colored::Colorize;
use std::{env, process::Command, time::Duration};

#[derive(Debug, Clone, Parser)]
#[clap(version, author, about)]
struct Args {
    #[clap(short = 'd', long)]
    /// command to run on difference detected
    on_diff: String,

    #[clap(short, long)]
    /// command to run
    command: String,

    #[clap(short, long, parse(try_from_str=duration_str::parse), default_value= "10s")]
    /// sleep between running the commands (time between running two commands )
    sleep: Duration,

    #[clap(short, long)]
    /// wether will continue or close after change detected
    remain: bool,
    #[clap(long)]
    /// shell to run commands ( default to $SHELL or sh )
    shell: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut prev_out = None;
    let fall_back_shell = env::var("SHELL").unwrap_or("sh".into());
    let shell = args.shell.clone().unwrap_or(fall_back_shell);
    loop {
        let out = Command::new(&shell)
            .arg("-c")
            .arg(&args.command)
            .output()
            .unwrap()
            .stdout;

        clearscreen::clear();

        print_status(&args);
        print!("{}", String::from_utf8_lossy(&out));

        if let Some(prev) = prev_out {
            if out != prev {
                Command::new(&shell)
                    .arg("-c")
                    .arg(&args.on_diff)
                    .output()
                    .unwrap();
                if !args.remain {
                    break;
                }
            }
        }
        prev_out = Some(out);
        std::thread::sleep(args.sleep);
    }
}

fn print_status(args: &Args) {
    let total_string = format!(
        "running command '{}' each '{}', on difference will run : '{}'",
        args.command.on_green(),
        format!("{:?}", args.sleep).on_red(),
        args.on_diff.on_cyan()
    );
    println!("{}", total_string.on_white().bold());
}
