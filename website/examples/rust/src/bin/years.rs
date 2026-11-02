use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    for year in cal.years(2021, 2024)? {
        println!("{} {} {:?}", year.year, year.start, year.weeks);
    }

    Ok(())
}
