use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar, Level};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let q4 = cal.period(Level::Quarter, 2023, Some(4))?;
    for period in cal.children(&q4, Level::Period)? {
        println!("{} {:?} {}", cal.format(&period, None)?, period.weeks, period.has_extra_week);
    }

    Ok(())
}
