use std::error::Error;
use std::fs;

use fiscalclock::FiscalCalendar;

fn main() -> Result<(), Box<dyn Error>> {
    let cal = FiscalCalendar::from_json(&fs::read_to_string("retail-4-5-4.json")?)?;

    let explanation = cal.explain("2024-02-03".parse()?)?;
    for step in explanation.steps {
        println!("{}: {}", step.level.name(), step.result);
    }

    Ok(())
}
