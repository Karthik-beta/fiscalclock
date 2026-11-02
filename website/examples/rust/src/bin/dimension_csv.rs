use std::error::Error;
use std::fs;

use fiscalclock::{FiscalCalendar};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    print!("{}", cal.dimension_csv("2024-02-03".parse()?, "2024-02-05".parse()?)?);

    Ok(())
}
