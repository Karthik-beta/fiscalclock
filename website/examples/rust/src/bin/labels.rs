use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar, Level};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let label = cal.format(&cal.period(Level::Period, 2023, Some(12))?, None)?;
    println!("{label}");

    let parsed = cal.parse(&label)?;
    println!(
        "{} {} {:?} {} {}",
        parsed.level.name(),
        parsed.year,
        parsed.index,
        parsed.start,
        parsed.last_day()
    );

    Ok(())
}
