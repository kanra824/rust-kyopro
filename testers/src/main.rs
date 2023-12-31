use clap::{Args, Parser, Subcommand};
mod commands;
use commands::{build::build, exec::exec, gen::gen};
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    Gen,
    Exec(Exec),
}

#[derive(Args)]
struct Exec {
    num: usize,
    command: String,
    args: Option<Vec<String>>,
}

fn main() {
    let cli = Cli::parse();

    let status = match &cli.command {
        Commands::Build => build(),
        Commands::Gen => gen(),
        Commands::Exec(e) => {
            let current_dir = env::current_dir()?;
            let contest_dir = env::var("CONTEST_DIR")?;
            let contest_dir = current_dir.join(&contest_dir);
            let solver_dir = current_dir.parent().unwrap().to_str().unwrap();
            //exec(e.num, e.command.clone(), e.args.clone());
        },
    };

    match status {
        Ok(status) => {
            if status.success() {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to execute process.");
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }
}
