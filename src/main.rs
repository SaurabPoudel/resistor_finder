use clap::Parser;
mod calculator;
mod cli;
mod color_code;
mod e_series;
mod resistor;

use anyhow::Result;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::ColorCode(args) => color_code::handle_color_code(args),
        Commands::Series(args) => calculator::handle_series(args),
        Commands::Parallel(args) => calculator::handle_parallel(args),
        Commands::FindNearest(args) => e_series::handle_find_nearest(args),
        Commands::PowerDissipation(args) => calculator::handle_power_dissipation(args),
    }
}
