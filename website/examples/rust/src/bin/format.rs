use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar, Level};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let week = cal.period(Level::Week, 2023, Some(53))?;
    println!("{}", cal.format(&week, None)?);
    println!("{}", cal.format(&cal.parent(&week)?, Some("nrf"))?);

    Ok(())
}
