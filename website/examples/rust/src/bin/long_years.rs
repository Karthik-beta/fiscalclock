use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar, Level};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let long_years: Vec<String> = cal
        .years(2006, 2023)?
        .iter()
        .filter(|year| year.weeks == Some(53))
        .map(|year| year.year.to_string())
        .collect();
    println!("{}", long_years.join(" "));

    let extra = cal.period(Level::Week, 2023, Some(53))?;
    println!("{} {} {}", extra.start, extra.last_day(), extra.has_extra_week);

    for year in [2022, 2023] {
        println!("{} {:?}", year, cal.period(Level::Period, year, Some(12))?.weeks);
    }

    Ok(())
}
