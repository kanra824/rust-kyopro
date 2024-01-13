use clap::{Args, Parser, Subcommand};
use anyhow::Result;
mod commands;
use commands::{build::build, exec::{exec, exec_all}, gen::gen, tester::{tester, tester_all}, vis::vis};
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
    Vis(Vis),
    Tester(Tester),

    // expansion
    ExecAll,
    TesterAll,
    //VisAll,
    // Run(Run),
    // RunAll,
}

#[derive(Args)]
struct Exec {
    num: usize,
}

#[derive(Args)]
struct Vis {
    num: usize,
}

#[derive(Args)]
struct Tester {
    num: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let contest_dir = std::env::var("CONTEST_DIR")?;
    let solver_path = std::env::var("SOLVER")?;

    let status = match &cli.command {
        Commands::Build => build(),
        Commands::Gen => gen(),
        Commands::Exec(e) => exec(e.num, &contest_dir, &solver_path),
        Commands::Vis(e) => vis(e.num),
        Commands::Tester(e) => tester(e.num, &contest_dir, &solver_path),
        Commands::TesterAll => tester_all(8, contest_dir, solver_path),
        Commands::ExecAll => exec_all(8, contest_dir, solver_path),
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
