use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::calendar::{FiscalCalendar, Location};
use crate::date::Date;
use crate::error::Error;
use crate::json::write_str;

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

pub(crate) enum Cell {
    Str(String),
    Int(i64),
    Bool(bool),
    Null,
}

impl FiscalCalendar {
    /// One row per day in `[start, end_exclusive)`, at most 3,660,000 days.
    pub fn dimension(&self, start: Date, end_exclusive: Date) -> Result<Vec<DimensionRow>, Error> {
        let days = self.dimension_span(start, end_exclusive)?;
        let mut rows = Vec::with_capacity(days as usize);
        for d in 0..days {
            let location = self.locate_at(start.add(d), "start")?;
            let labels = self
                .label_slots()
                .map(|(s, l)| self.label_for(s, l, &location))
                .collect();
            rows.push(DimensionRow { location, labels });
        }
        Ok(rows)
    }

    /// The same rows as [`FiscalCalendar::dimension`], as RFC 4180 CSV with a
    /// header row, `\n` line endings and a final newline.
    pub fn dimension_csv(&self, start: Date, end_exclusive: Date) -> Result<String, Error> {
        let rows = self.dimension(start, end_exclusive)?;
        let mut out = String::new();
        let header = FIXED_COLUMNS
            .iter()
            .map(|c| String::from(*c))
            .chain(self.label_columns());
        for (i, name) in header.enumerate() {
            if i > 0 {
                out.push(',');
            }
            csv_field(&mut out, &name);
        }
        out.push('\n');
        for row in &rows {
            for (i, cell) in self.cells(row).into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                match cell {
                    Cell::Str(s) => csv_field(&mut out, &s),
                    Cell::Int(n) => out.push_str(&format!("{n}")),
                    Cell::Bool(b) => out.push_str(if b { "true" } else { "false" }),
                    Cell::Null => {}
                }
            }
            out.push('\n');
        }
        Ok(out)
    }

    pub(crate) fn cells(&self, row: &DimensionRow) -> Vec<Cell> {
        let l = &row.location;
        let date = |d: Date| Cell::Str(format!("{d}"));
        let index = |i: Option<u32>| i.map_or(Cell::Null, |i| Cell::Int(i64::from(i)));
        let mut cells = alloc::vec![
            date(l.date),
            Cell::Str(String::from(self.calendar_id())),
            Cell::Int(i64::from(l.year.year)),
            index(l.quarter.index),
            index(l.period.index),
            index(l.week.index),
            Cell::Int(i64::from(l.day_of_year)),
            Cell::Int(i64::from(l.day_of_quarter)),
            Cell::Int(i64::from(l.day_of_period)),
            Cell::Int(i64::from(l.day_of_week)),
            date(l.year.start),
            date(l.year.last_day()),
            date(l.quarter.start),
            date(l.quarter.last_day()),
            date(l.period.start),
            date(l.period.last_day()),
            date(l.week.start),
            date(l.week.last_day()),
            Cell::Bool(l.year.has_extra_week),
            Cell::Bool(l.period.has_extra_week),
            Cell::Bool(l.week.has_extra_week),
        ];
        cells.extend(
            row.labels
                .iter()
                .map(|s| s.as_ref().map_or(Cell::Null, |s| Cell::Str(s.clone()))),
        );
        cells
    }

    pub(crate) fn write_rows_json(&self, out: &mut String, rows: &[DimensionRow]) {
        let names: Vec<String> = FIXED_COLUMNS
            .iter()
            .map(|c| String::from(*c))
            .chain(self.label_columns())
            .collect();
        out.push('[');
        for (r, row) in rows.iter().enumerate() {
            if r > 0 {
                out.push(',');
            }
            out.push('{');
            for (i, (name, cell)) in names.iter().zip(self.cells(row)).enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_str(out, name);
                out.push(':');
                match cell {
                    Cell::Str(s) => write_str(out, &s),
                    Cell::Int(n) => out.push_str(&format!("{n}")),
                    Cell::Bool(b) => out.push_str(if b { "true" } else { "false" }),
                    Cell::Null => out.push_str("null"),
                }
            }
            out.push('}');
        }
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
