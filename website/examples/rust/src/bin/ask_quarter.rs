use std::error::Error;
use std::fs;

use fiscalclock::FiscalCalendar;

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let here = cal.locate("2024-02-03".parse()?)?;
    println!("{}", cal.format(&here.quarter, None)?);
    println!("{}", cal.call_json("locate", r#"{"date": "2024-02-03"}"#));

    Ok(())
}
