use std::error::Error;
use std::fs;

use fiscalclock::{Date, FiscalCalendar};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    println!("{:?}", Date::parse_iso("2024-02-03T23:30:00"));

    let date = Date::from_ymd(2024, 2, 3).ok_or("no such date")?;
    println!("{}", cal.format(&cal.locate(date)?.week, None)?);

    Ok(())
}
