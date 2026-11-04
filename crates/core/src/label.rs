use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use crate::calendar::Level;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Part {
    Lit(String),
    Yyyy,
    Yy,
    YyNext,
    Q,
    Pp,
    Ww,
}

/// A compiled label template; the source text is kept for the canonical form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Template {
    pub source: String,
    pub parts: Vec<Part>,
}

impl Template {
    /// Compiles `source` for `level`, or returns why it is invalid.
    pub fn compile(source: &str, level: Level) -> Result<Template, String> {
        let mut parts = Vec::new();
        let mut lit = String::new();
        let mut rest = source;
        while let Some(c) = rest.chars().next() {
            match c {
                '{' => {
                    let close = rest
                        .find('}')
                        .ok_or_else(|| String::from("unbalanced brace: '{' is never closed"))?;
                    let name = &rest[1..close];
                    if name.contains('{') {
                        return Err(String::from("unbalanced brace: '{' inside a placeholder"));
                    }
                    let part = match name {
                        "yyyy" => Part::Yyyy,
                        "yy" => Part::Yy,
                        "yy_next" => Part::YyNext,
                        "q" => Part::Q,
                        "pp" => Part::Pp,
                        "ww" => Part::Ww,
                        _ => return Err(format!("unknown placeholder {{{name}}}")),
                    };
                    let allowed = match part {
                        Part::Q => level == Level::Quarter,
                        Part::Pp => level == Level::Period,
                        Part::Ww => level == Level::Week,
                        _ => true,
                    };
                    if !allowed {
                        return Err(format!(
                            "placeholder {{{name}}} is not allowed in a {} template",
                            level.name()
                        ));
                    }
                    if !lit.is_empty() {
                        parts.push(Part::Lit(core::mem::take(&mut lit)));
                    }
                    parts.push(part);
                    rest = &rest[close + 1..];
                }
                '}' => return Err(String::from("unbalanced brace: '}' without '{'")),
                _ => {
                    lit.push(c);
                    rest = &rest[c.len_utf8()..];
                }
            }
        }
        if !lit.is_empty() {
            parts.push(Part::Lit(lit));
        }
        if !parts.iter().any(|p| matches!(p, Part::Yyyy | Part::Yy)) {
            return Err(String::from(
                "the template has no year placeholder: it needs {yyyy} or {yy}, and {yy_next} alone is not enough",
            ));
        }
        let index = match level {
            Level::Year => None,
            Level::Quarter => Some((Part::Q, "{q}")),
            Level::Period => Some((Part::Pp, "{pp}")),
            Level::Week => Some((Part::Ww, "{ww}")),
        };
        if let Some((part, name)) = index {
            if !parts.contains(&part) {
                return Err(format!(
                    "a {} template must contain {name} so that its labels identify the {}",
                    level.name(),
                    level.name()
                ));
            }
        }
        Ok(Template {
            source: String::from(source),
            parts,
        })
    }

    /// True when the template has `{yy}` but no `{yyyy}`.
    pub fn needs_pivot(&self) -> bool {
        !self.parts.contains(&Part::Yyyy) && self.parts.contains(&Part::Yy)
    }

    /// Renders the label; `None` when `{yyyy}` would need a year outside 1–9999.
    pub fn render(&self, year_number: i64, index: Option<u32>) -> Option<String> {
        let mut out = String::new();
        for part in &self.parts {
            match part {
                Part::Lit(s) => out.push_str(s),
                Part::Yyyy => {
                    if !(1..=9999).contains(&year_number) {
                        return None;
                    }
                    let _ = write!(out, "{year_number:04}");
                }
                Part::Yy => {
                    let _ = write!(out, "{:02}", year_number.rem_euclid(100));
                }
                Part::YyNext => {
                    let _ = write!(out, "{:02}", (year_number + 1).rem_euclid(100));
                }
                Part::Q => {
                    let _ = write!(out, "{}", index.unwrap_or(0));
                }
                Part::Pp | Part::Ww => {
                    let _ = write!(out, "{:02}", index.unwrap_or(0));
                }
            }
        }
        Some(out)
    }

    /// Every way the whole label matches this template, narrower widths first.
    pub fn captures(&self, label: &str) -> Vec<Captures> {
        let label = label.as_bytes();
        let mut found = Vec::new();
        let mut pending = alloc::vec![(0, 0, Captures::default())];
        while let Some((part, at, caps)) = pending.pop() {
            let s = &label[at..];
            let Some(first) = self.parts.get(part) else {
                if s.is_empty() {
                    found.push(caps);
                }
                continue;
            };
            let widths: &[usize] = match first {
                Part::Lit(lit) => {
                    if s.starts_with(lit.as_bytes()) {
                        pending.push((part + 1, at + lit.len(), caps));
                    }
                    continue;
                }
                Part::Yyyy => &[4],
                Part::Q => &[1, 2],
                _ => &[2],
            };
            for &n in widths.iter().rev() {
                let Some(value) = digits(s, n) else { continue };
                if matches!(first, Part::Q) && s[0] == b'0' {
                    continue;
                }
                let mut next = caps;
                let ok = match first {
                    Part::Yyyy => set(&mut next.yyyy, value),
                    Part::Yy => set(&mut next.yy, value),
                    Part::YyNext => set(&mut next.yy_next, value),
                    _ => set(&mut next.index, value),
                };
                if ok {
                    pending.push((part + 1, at + n, next));
                }
            }
        }
        found
    }
}

/// Values read from a label. A placeholder that appears twice must read the same value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Captures {
    pub yyyy: Option<u32>,
    pub yy: Option<u32>,
    pub yy_next: Option<u32>,
    pub index: Option<u32>,
}

fn set(slot: &mut Option<u32>, value: u32) -> bool {
    match *slot {
        Some(v) => v == value,
        None => {
            *slot = Some(value);
            true
        }
    }
}

fn digits(s: &[u8], n: usize) -> Option<u32> {
    if s.len() < n {
        return None;
    }
    s[..n].iter().try_fold(0u32, |acc, &c| {
        c.is_ascii_digit().then(|| acc * 10 + u32::from(c - b'0'))
    })
}
