use crate::cli::FindNearestArgs;
use anyhow::Result;

const E12_SERIES: [f64; 12] = [1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2];

pub fn handle_find_nearest(args: FindNearestArgs) -> Result<()> {
    let nearest = find_nearest_e12(args.value);
    println!("Nearest E12 resistor value: {} ohms", nearest);
    Ok(())
}

fn find_nearest_e12(value: f64) -> f64 {
    let magnitude = value.log10().floor();
    let normalized = value / 10.0f64.powf(magnitude);

    let nearest_base = E12_SERIES
        .iter()
        .min_by(|&&a, &b| {
            (a - normalized)
                .abs()
                .partial_cmp(&(b - normalized).abs())
                .unwrap()
        })
        .unwrap();

    nearest_base * 10.0f64.powf(magnitude)
}
