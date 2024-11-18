use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "resistor_finder")]
#[command(about = "A CLI tool for resistor calculations and lookups")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    ColorCode(ColorCodeArgs),
    Series(SeriesArgs),
    Parallel(ParallelArgs),
    FindNearest(FindNearestArgs),
    PowerDissipation(PowerDissipationArgs),
}

#[derive(Args)]
pub struct ColorCodeArgs {
    #[arg(long, required = true, num_args = 4..=5)]
    pub colors: Vec<String>,
}

#[derive(Args)]
pub struct SeriesArgs {
    #[arg(long, required = true, num_args = 1..)]
    pub values: Vec<f64>,
}

#[derive(Args)]
pub struct ParallelArgs {
    #[arg(long, required = true, num_args = 1..)]
    pub values: Vec<f64>,
}

#[derive(Args)]
pub struct FindNearestArgs {
    #[arg(long, required = true)]
    pub value: f64,
}

#[derive(Args)]
pub struct PowerDissipationArgs {
    #[arg(long, required = true)]
    pub voltage: f64,
    #[arg(long, required = true)]
    pub resistance: f64,
}
