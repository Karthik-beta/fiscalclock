use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar, Level};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("two-schemes.json")?)?;

    let year = cal.period(Level::Year, 2023, None)?;
    println!("{}", cal.format(&year, None)?);
    println!("{}", cal.format(&year, Some("by_end"))?);

    Ok(())
}
