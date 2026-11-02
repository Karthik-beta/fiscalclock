use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar, Level};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let here = cal.locate("2024-02-03".parse()?)?;
    println!(
        "{} | {} | {}",
        cal.format(&here.year, None)?,
        cal.format(&here.quarter, None)?,
        cal.format(&here.week, None)?
    );

    let fy2023 = cal.period(Level::Year, 2023, None)?;
    println!("{} {} {:?}", fy2023.start, fy2023.end_exclusive, fy2023.weeks);

    Ok(())
}
