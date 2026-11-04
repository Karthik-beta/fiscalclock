use std::error::Error;
use std::fs;

use fiscalclock::FiscalCalendar;

fn main() -> Result<(), Box<dyn Error>> {
    match FiscalCalendar::from_json(&fs::read_to_string("missing-choices.json")?) {
        Ok(_) => println!("valid"),
        Err(errors) => {
            for error in errors.as_slice() {
                println!("{} {}", error.path, error.code);
            }
        }
    }
    Ok(())
}
