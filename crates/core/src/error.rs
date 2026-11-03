use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

use crate::calendar::{Level, Period};

/// A stable error code from the catalogue in `spec/api-v1.md`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ErrorCode {
    /// The definition is not valid JSON, is not an object, or repeats a key in
    /// some object.
    InvalidJson,
    /// A field the format does not define.
    UnknownField,
    /// A required field is absent.
    MissingField,
    /// A field has the wrong JSON type.
    InvalidType,
    /// A value is outside its allowed set or range.
    InvalidValue,
    /// `spec_version` is not 1.
    UnsupportedSpecVersion,
    /// A reserved part of the format is used.
    UnsupportedFeature,
    /// A subdivision type is not valid inside the year type.
    UnsupportedCombination,
    /// `periods` does not sum to 52.
    WeeksSumMismatch,
    /// `quarters` does not sum to the number of periods.
    QuartersSumMismatch,
    /// A week-based year has no `extra_week`.
    ExtraWeekMissing,
    /// `extra_week.period` names a period the year does not have.
    ExtraWeekOutOfRange,
    /// `start.day` is 29–31 and `short_month` is absent.
    ShortMonthPolicyMissing,
    /// `month_start_day` differs from `year.start.day`.
    SubdivisionYearMismatch,
    /// `naming.default` is not a scheme name.
    DefaultSchemeMissing,
    /// A template is malformed.
    InvalidTemplate,
    /// A two-digit year template has no `century_pivot`.
    CenturyPivotMissing,
    /// A scheme gives two fiscal years the same number, or skips a number.
    YearNumberCollision,
    /// A `majority` scheme meets a fiscal year split evenly between two calendar years.
    MajorityTie,
    /// A date argument is not a valid `YYYY-MM-DD` civil date.
    InvalidDate,
    /// A binding received a timestamp where a date was expected.
    TimestampRejected,
    /// An argument is missing, of the wrong type or out of its allowed range.
    InvalidArgument,
    /// The fiscal year involved is outside the supported range.
    OutOfRange,
    /// No period has those coordinates.
    PeriodNotFound,
    /// The requested level does not nest inside the period.
    LevelNotNested,
    /// No scheme has that name.
    SchemeNotFound,
    /// The scheme has no template for the period's level.
    TemplateMissing,
    /// No template matches the label.
    LabelNotRecognised,
    /// More than one period matches the label.
    AmbiguousLabel,
}

impl ErrorCode {
    /// The upper-snake-case code, such as `"WEEKS_SUM_MISMATCH"`.
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorCode::InvalidJson => "INVALID_JSON",
            ErrorCode::UnknownField => "UNKNOWN_FIELD",
            ErrorCode::MissingField => "MISSING_FIELD",
            ErrorCode::InvalidType => "INVALID_TYPE",
            ErrorCode::InvalidValue => "INVALID_VALUE",
            ErrorCode::UnsupportedSpecVersion => "UNSUPPORTED_SPEC_VERSION",
            ErrorCode::UnsupportedFeature => "UNSUPPORTED_FEATURE",
            ErrorCode::UnsupportedCombination => "UNSUPPORTED_COMBINATION",
            ErrorCode::WeeksSumMismatch => "WEEKS_SUM_MISMATCH",
            ErrorCode::QuartersSumMismatch => "QUARTERS_SUM_MISMATCH",
            ErrorCode::ExtraWeekMissing => "EXTRA_WEEK_MISSING",
            ErrorCode::ExtraWeekOutOfRange => "EXTRA_WEEK_OUT_OF_RANGE",
            ErrorCode::ShortMonthPolicyMissing => "SHORT_MONTH_POLICY_MISSING",
            ErrorCode::SubdivisionYearMismatch => "SUBDIVISION_YEAR_MISMATCH",
            ErrorCode::DefaultSchemeMissing => "DEFAULT_SCHEME_MISSING",
            ErrorCode::InvalidTemplate => "INVALID_TEMPLATE",
            ErrorCode::CenturyPivotMissing => "CENTURY_PIVOT_MISSING",
            ErrorCode::YearNumberCollision => "YEAR_NUMBER_COLLISION",
            ErrorCode::MajorityTie => "MAJORITY_TIE",
            ErrorCode::InvalidDate => "INVALID_DATE",
            ErrorCode::TimestampRejected => "TIMESTAMP_REJECTED",
            ErrorCode::InvalidArgument => "INVALID_ARGUMENT",
            ErrorCode::OutOfRange => "OUT_OF_RANGE",
            ErrorCode::PeriodNotFound => "PERIOD_NOT_FOUND",
            ErrorCode::LevelNotNested => "LEVEL_NOT_NESTED",
            ErrorCode::SchemeNotFound => "SCHEME_NOT_FOUND",
            ErrorCode::TemplateMissing => "TEMPLATE_MISSING",
            ErrorCode::LabelNotRecognised => "LABEL_NOT_RECOGNISED",
            ErrorCode::AmbiguousLabel => "AMBIGUOUS_LABEL",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One period a label could denote, reported by [`ErrorCode::AmbiguousLabel`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    /// The period, with its year under the default scheme.
    pub period: Period,
    /// The scheme whose template matched.
    pub scheme: String,
    /// The level of the template that matched.
    pub level: Level,
}

/// Structured details attached to some errors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Details {
    /// The periods an ambiguous label could denote, ordered by start date,
    /// then scheme name, then level.
    Candidates(Vec<Candidate>),
    /// Up to five affected years of a numbering error. For
    /// [`ErrorCode::YearNumberCollision`] these are the year numbers given
    /// twice or skipped, including the scheme's `offset`. For
    /// [`ErrorCode::MajorityTie`] they are the calendar years in which a tied
    /// fiscal year starts, without the offset.
    Years(Vec<i64>),
}

/// A structured error: a stable code, the JSON path it concerns, and text for people.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    /// The stable code.
    pub code: ErrorCode,
    /// Dotted JSON path with `[i]` indexes, or the argument name; empty for the whole input.
    pub path: String,
    /// What went wrong. Do not match on the wording.
    pub message: String,
    /// How to fix it, when there is a useful suggestion.
    pub hint: Option<String>,
    /// Structured details, for the codes that carry them.
    pub details: Option<Details>,
}

impl Error {
    pub(crate) fn new(
        code: ErrorCode,
        path: impl Into<String>,
        message: impl Into<String>,
    ) -> Error {
        Error {
            code,
            path: path.into(),
            message: message.into(),
            hint: None,
            details: None,
        }
    }

    pub(crate) fn hint(mut self, hint: impl ToString) -> Error {
        self.hint = Some(hint.to_string());
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "{}: {}", self.code, self.message)
        } else {
            write!(f, "{} at {}: {}", self.code, self.path, self.message)
        }
    }
}

impl core::error::Error for Error {}

/// Every error found while validating a definition, sorted by path and then code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Errors(Vec<Error>);

impl Errors {
    pub(crate) fn sorted(mut errors: Vec<Error>) -> Errors {
        errors.sort_by(|a, b| {
            a.path
                .cmp(&b.path)
                .then_with(|| a.code.as_str().cmp(b.code.as_str()))
        });
        Errors(errors)
    }

    /// The errors as a slice.
    pub fn as_slice(&self) -> &[Error] {
        &self.0
    }

    /// The errors as an owned vector.
    pub fn into_vec(self) -> Vec<Error> {
        self.0
    }
}

impl From<Error> for Errors {
    fn from(e: Error) -> Errors {
        Errors(alloc::vec![e])
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, e) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str("; ")?;
            }
            write!(f, "{e}")?;
        }
        Ok(())
    }
}

impl core::error::Error for Errors {}
