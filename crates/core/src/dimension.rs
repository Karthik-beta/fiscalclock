use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use crate::calendar::{FiscalCalendar, Location};
use crate::date::Date;
use crate::error::Error;
use crate::json::{write_date, write_list, write_str};

/// One day of the date dimension.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DimensionRow {
    /// Where the day falls at every level.
    pub location: Location,
    /// One label per entry of [`FiscalCalendar::label_columns`], in the same
    /// order; `None` when the label cannot be formatted, such as a `{yyyy}`
    /// year number outside 1–9999.
    pub labels: Vec<Option<String>>,
}

/// The fixed columns of a dimension row, before the label columns.
pub const FIXED_COLUMNS: [&str; 21] = [
    "date",
    "calendar_id",
    "fiscal_year",
    "quarter",
    "period",
    "week",
    "day_of_year",
    "day_of_quarter",
    "day_of_period",
    "day_of_week",
    "year_start",
    "year_last_day",
    "quarter_start",
    "quarter_last_day",
    "period_start",
    "period_last_day",
    "week_start",
    "week_last_day",
    "year_has_extra_week",
    "period_has_extra_week",
    "week_is_extra_week",
];

/// One cell of a dimension row, borrowing its text from the calendar or the row.
enum Cell<'a> {
    Text(&'a str),
    Date(Date),
    Int(i64),
    Bool(bool),
    Null,
}

impl Cell<'_> {
    fn write_csv(&self, out: &mut String) {
        match *self {
            Cell::Text(s) => csv_field(out, s),
            // The ISO form of a date has no character that CSV quotes.
            Cell::Date(d) => {
                let _ = write!(out, "{d}");
            }
            Cell::Int(n) => {
                let _ = write!(out, "{n}");
            }
            Cell::Bool(b) => out.push_str(if b { "true" } else { "false" }),
            Cell::Null => {}
        }
    }

    fn write_json(&self, out: &mut String) {
        match *self {
            Cell::Text(s) => write_str(out, s),
            Cell::Date(d) => write_date(out, d),
            Cell::Int(n) => {
                let _ = write!(out, "{n}");
            }
            Cell::Bool(b) => out.push_str(if b { "true" } else { "false" }),
            Cell::Null => out.push_str("null"),
        }
    }
}

impl FiscalCalendar {
    /// One row per day in `[start, end_exclusive)`, at most 146,097 days (400 years).
    ///
    /// # Errors
    ///
    /// `INVALID_ARGUMENT` at `end_exclusive` when `start` is after it or the
    /// range is longer than 146,097 days. `OUT_OF_RANGE` at `start`, or at
    /// `end_exclusive` for the range's last day, when that day's fiscal year is
    /// outside the supported range. An empty range returns no rows without the
    /// range check.
    pub fn dimension(&self, start: Date, end_exclusive: Date) -> Result<Vec<DimensionRow>, Error> {
        let days = self.dimension_span(start, end_exclusive)?;
        let mut rows = Vec::with_capacity(days as usize);
        for d in 0..days {
            let (key, location) = self.locate_keyed(start.add(d), "start")?;
            let labels = self
                .label_slots()
                .map(|(s, l)| self.label_for(s, l, &location, key))
                .collect();
            rows.push(DimensionRow { location, labels });
        }
        Ok(rows)
    }

    /// The same rows as [`FiscalCalendar::dimension`], as RFC 4180 CSV with a
    /// header row, `\n` line endings and a final newline.
    ///
    /// # Errors
    ///
    /// The same as [`FiscalCalendar::dimension`].
    pub fn dimension_csv(&self, start: Date, end_exclusive: Date) -> Result<String, Error> {
        let rows = self.dimension(start, end_exclusive)?;
        let mut out = String::new();
        write_list(&mut out, self.columns(), |out, name| csv_field(out, &name));
        out.push('\n');
        for row in &rows {
            write_list(&mut out, self.cells(row), |out, cell| cell.write_csv(out));
            out.push('\n');
        }
        Ok(out)
    }

    /// The fixed columns followed by the label columns.
    fn columns(&self) -> Vec<String> {
        FIXED_COLUMNS
            .iter()
            .map(|c| String::from(*c))
            .chain(self.label_columns())
            .collect()
    }

    fn cells<'a>(&'a self, row: &'a DimensionRow) -> Vec<Cell<'a>> {
        let l = &row.location;
        let index = |i: Option<u32>| i.map_or(Cell::Null, |i| Cell::Int(i64::from(i)));
        let mut cells = Vec::with_capacity(FIXED_COLUMNS.len() + row.labels.len());
        cells.extend([
            Cell::Date(l.date),
            Cell::Text(self.calendar_id()),
            Cell::Int(i64::from(l.year.year)),
            index(l.quarter.index),
            index(l.period.index),
            index(l.week.index),
            Cell::Int(i64::from(l.day_of_year)),
            Cell::Int(i64::from(l.day_of_quarter)),
            Cell::Int(i64::from(l.day_of_period)),
            Cell::Int(i64::from(l.day_of_week)),
            Cell::Date(l.year.start),
            Cell::Date(l.year.last_day()),
            Cell::Date(l.quarter.start),
            Cell::Date(l.quarter.last_day()),
            Cell::Date(l.period.start),
            Cell::Date(l.period.last_day()),
            Cell::Date(l.week.start),
            Cell::Date(l.week.last_day()),
            Cell::Bool(l.year.has_extra_week),
            Cell::Bool(l.period.has_extra_week),
            Cell::Bool(l.week.has_extra_week),
        ]);
        cells.extend(
            row.labels
                .iter()
                .map(|s| s.as_deref().map_or(Cell::Null, Cell::Text)),
        );
        cells
    }

    pub(crate) fn write_rows_json(&self, out: &mut String, rows: &[DimensionRow]) {
        let names = self.columns();
        out.push('[');
        write_list(out, rows, |out, row| {
            out.push('{');
            write_list(
                out,
                names.iter().zip(self.cells(row)),
                |out, (name, cell)| {
                    write_str(out, name);
                    out.push(':');
                    cell.write_json(out);
                },
            );
            out.push('}');
        });
        out.push(']');
    }
}

fn csv_field(out: &mut String, s: &str) {
    if s.contains([',', '"', '\n', '\r']) {
        out.push('"');
        out.push_str(&s.replace('"', "\"\""));
        out.push('"');
    } else {
        out.push_str(s);
    }
}
