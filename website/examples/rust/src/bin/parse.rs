use std::error::Error;
use std::fs;

use fiscalclock::FiscalCalendar;

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let week = cal.parse("FY2023 W53")?;
    println!(
        "{} {} {:?} {} {} {}",
        week.level.name(),
        week.year,
        week.index,
        week.start,
        week.last_day(),
        week.has_extra_week
    );

    Ok(())
}
