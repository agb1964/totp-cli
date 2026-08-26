mod config;
mod totp;

use clap::Parser;
use config::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let code = match &cli.command {
        Commands::Generate { issuer, account } => totp::run_generate(issuer, account),
        Commands::Verify { secret, code } => totp::run_verify(secret, code),
    };
    std::process::exit(code);
}
