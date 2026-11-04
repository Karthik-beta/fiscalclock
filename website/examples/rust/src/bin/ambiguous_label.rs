use std::error::Error;
use std::fs;

use fiscalclock::{Details, FiscalCalendar};

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("two-schemes.json")?)?;

    if let Err(error) = cal.parse("FY2024") {
        println!("{}", error.code);
        if let Some(Details::Candidates(candidates)) = &error.details {
            for c in candidates {
                println!(
                    "{} {} {} {}",
                    c.scheme,
                    c.period.year,
                    c.period.start,
                    c.period.last_day()
                );
            }
        }
    }
    Ok(())
}
