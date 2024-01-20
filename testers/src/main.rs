use anyhow::Result;
use clap::{Args, Parser, Subcommand};
mod commands;
use commands::{
    build::build,
    exec::{exec, exec_all},
    gen::{gen_seed, gen},
    tester::{tester, tester_all},
    vis::vis,
    score::{score_all, eprint_score},
    run::{run, run_all},
};
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build,
    GenSeed(Num),
    Gen,
    Exec(Num),
    Vis(Num),
    Tester(Num),
    Score(Num),

    // expansion
    ExecAll,
    ScoreAll,
    TesterAll,
    Run(Num),
    RunAll,
    //VisAll,
}

#[derive(Args)]
struct Num {
    num: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let contest_dir = std::env::var("CONTEST_DIR")?;
    let solver_path = std::env::var("SOLVER")?;

    let status = match &cli.command {
        Commands::Build => build(contest_dir),
        Commands::GenSeed(e) => gen_seed(contest_dir, e.num),
        Commands::Gen => gen(contest_dir),
        Commands::Exec(e) => exec(e.num, &contest_dir, &solver_path),
        Commands::Vis(e) => vis(e.num),
        Commands::Tester(e) => tester(e.num, &contest_dir, &solver_path),
        Commands::Score(e) => eprint_score(e.num, contest_dir),
        Commands::ExecAll => exec_all(8, contest_dir, solver_path),
        Commands::ScoreAll => score_all(contest_dir),
        Commands::TesterAll => tester_all(8, contest_dir, solver_path),
        Commands::Run(e) => run(e.num, &contest_dir, &solver_path),
        Commands::RunAll => run_all(8, contest_dir, solver_path),
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
