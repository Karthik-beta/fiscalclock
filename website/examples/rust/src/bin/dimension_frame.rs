use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar, Level};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let fy2023 = cal.period(Level::Year, 2023, None)?;
    let days = cal.dimension(fy2023.start, fy2023.end_exclusive)?;
    println!("{}", days.len());
    for quarter in cal.children(&fy2023, Level::Quarter)? {
        let count = days.iter().filter(|day| day.location.quarter == quarter).count();
        println!("{} {}", cal.format(&quarter, None)?, count);
    }

    Ok(())
}
