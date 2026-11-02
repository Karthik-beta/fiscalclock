use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar, Level};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let quarters = cal.children(&cal.period(Level::Year, 2023, None)?, Level::Quarter)?;
    for quarter in &quarters {
        println!("{} {} {}", quarter.start, quarter.end_exclusive, quarter.days);
    }

    println!("{}", quarters.windows(2).all(|pair| pair[0].end_exclusive == pair[1].start));

    Ok(())
}
