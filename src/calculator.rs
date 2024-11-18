use crate::cli::{ParallelArgs, PowerDissipationArgs, SeriesArgs};
use anyhow::Result;

pub fn handle_series(args: SeriesArgs) -> Result<()> {
    let total = args.values.iter().sum::<f64>();
    println!("Total Series Resistance: {} ohms", total);
    Ok(())
}

pub fn handle_parallel(args: ParallelArgs) -> Result<()> {
    let total = 1.0 / args.values.iter().map(|r| 1.0 / r).sum::<f64>();
    println!("Total Parallel Resistance: {} ohms", total);
    Ok(())
}

pub fn handle_power_dissipation(args: PowerDissipationArgs) -> Result<()> {
    let power = (args.voltage * args.voltage) / args.resistance;
    println!("Power Dissipation: {:.2} watts", power);
    Ok(())
}
