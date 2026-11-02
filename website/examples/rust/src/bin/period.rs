use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar, Level};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let q4 = cal.period(Level::Quarter, 2023, Some(4))?;
    println!(
        "{} {} {} {} {:?} {}",
        q4.start, q4.end_exclusive, q4.last_day(), q4.days, q4.weeks, q4.has_extra_week
    );

    Ok(())
}
