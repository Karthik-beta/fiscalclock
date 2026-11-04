use std::error::Error;
use std::fs;

use fiscalclock::FiscalCalendar;

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;
    println!("{}", cal.calendar_id());
    Ok(())
}
