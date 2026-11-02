use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let here = cal.locate("2024-02-03".parse()?)?;
    println!("{} {:?} {:?} {:?}", here.year.year, here.quarter.index, here.period.index, here.week.index);
    println!("{} {} {} {}", here.day_of_year, here.day_of_quarter, here.day_of_period, here.day_of_week);

    Ok(())
}
