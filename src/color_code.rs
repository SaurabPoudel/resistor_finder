use anyhow::Result;
use std::collections::HashMap;

use crate::cli::ColorCodeArgs;
use crate::resistor::Resistor;

lazy_static::lazy_static! {
    static ref COLOR_VALUES: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("black", 0);
        m.insert("brown", 1);
        m.insert("red", 2);
        m.insert("orange", 3);
        m.insert("yellow", 4);
        m.insert("green", 5);
        m.insert("blue", 6);
        m.insert("violet", 7);
        m.insert("gray", 8);
        m.insert("white", 9);
        m
    };

    static ref TOLERANCE_VALUES: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        m.insert("brown", 1.0);
        m.insert("red", 2.0);
        m.insert("gold", 5.0);
        m.insert("silver", 10.0);
        m
    };
}

pub fn handle_color_code(args: ColorCodeArgs) -> Result<()> {
    if args.colors.len() != 4 && args.colors.len() != 5 {
        anyhow::bail!("Invalid number of color bands. Must be 4 or 5.");
    }

    let resistor = decode_colors(&args.colors)?;
    println!(
        "Resistance: {} ohms, {}% tolerance",
        resistor.value, resistor.tolerance
    );
    Ok(())
}

fn decode_colors(colors: &[String]) -> Result<Resistor> {
    let mut value = 0.0;

    for color in colors.iter().take(colors.len() - 2) {
        let digit = COLOR_VALUES
            .get(color.as_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid color: {}", color))?;
        value = value * 10.0 + *digit as f64;
    }

    let mult_color = &colors[colors.len() - 2];
    let multiplier = 10.0f64.powi(
        *COLOR_VALUES
            .get(mult_color.as_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid multiplier color: {}", mult_color))?
            as i32,
    );

    let tolerance_color = &colors[colors.len() - 1];
    let tolerance = TOLERANCE_VALUES
        .get(tolerance_color.as_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid tolerance color: {}", tolerance_color))?;

    Ok(Resistor::new(value * multiplier, *tolerance))
}
