use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let week_label = cal.label_columns().iter().position(|c| c == "label_nrf_week").unwrap();
    for row in cal.dimension("2024-02-02".parse()?, "2024-02-05".parse()?)? {
        let at = row.location;
        println!(
            "{} {} {:?} {:?} {:?} {}",
            at.date,
            at.year.year,
            at.quarter.index,
            at.period.index,
            at.week.index,
            row.labels[week_label].as_deref().unwrap_or("")
        );
    }

    Ok(())
}
