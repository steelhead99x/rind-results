// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0
//! Python's number formatting and `json.dumps`, byte for byte, so the
//! artifacts the Python tools published regenerate unchanged.

use serde_json::{Map, Value};
use std::cmp::Ordering;

/// `repr(float)`: the shortest digits that round-trip, positional from 1e-4
/// up to 1e16 and scientific (`1e-05`, `1.5e+16`) outside that.
pub fn repr(x: f64) -> String {
    if x.is_nan() {
        return "NaN".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "Infinity" } else { "-Infinity" }.into();
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0.0" } else { "0.0" }.into();
    }
    let s = format!("{:e}", x.abs());
    let (mant, exp) = s.split_once('e').expect("{:e} has an exponent");
    let exp: i32 = exp.parse().expect("exponent");
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let sign = if x < 0.0 { "-" } else { "" };
    let n = digits.len() as i32;
    let decpt = exp + 1;
    let body = if (-4..16).contains(&exp) {
        if decpt <= 0 {
            format!("0.{}{}", "0".repeat((-decpt) as usize), digits)
        } else if decpt >= n {
            format!("{}{}.0", digits, "0".repeat((decpt - n) as usize))
        } else {
            format!(
                "{}.{}",
                &digits[..decpt as usize],
                &digits[decpt as usize..]
            )
        }
    } else {
        let rest = &digits[1..];
        let e = format!("{}{:02}", if exp < 0 { '-' } else { '+' }, exp.abs());
        if rest.is_empty() {
            format!("{}e{}", &digits[..1], e)
        } else {
            format!("{}.{}e{}", &digits[..1], rest, e)
        }
    };
    format!("{sign}{body}")
}

/// `f"{x:.{prec}f}"`.
pub fn fixed(x: f64, prec: usize) -> String {
    format!("{x:.prec$}")
}

/// `f"{x:,.{prec}f}"`.
pub fn fixed_commas(x: f64, prec: usize) -> String {
    commas(&fixed(x, prec))
}

/// `f"{n:,}"` for an integer.
pub fn int_commas(n: i64) -> String {
    commas(&n.to_string())
}

fn commas(s: &str) -> String {
    let (sign, s) = s.strip_prefix('-').map_or(("", s), |r| ("-", r));
    let (int, frac) = s.split_once('.').map_or((s, None), |(a, b)| (a, Some(b)));
    let mut out = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (int.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    match frac {
        Some(f) => format!("{sign}{out}.{f}"),
        None => format!("{sign}{out}"),
    }
}

/// A JSON number as Python holds it: `int` or `float`.
#[derive(Clone, Copy, Debug)]
pub enum Num {
    Int(i64),
    Float(f64),
}

impl Num {
    pub fn of(v: &Value) -> Option<Num> {
        match v {
            Value::Number(n) if n.is_f64() => n.as_f64().map(Num::Float),
            Value::Number(n) => n.as_i64().map(Num::Int),
            Value::Bool(b) => Some(Num::Int(*b as i64)),
            _ => None,
        }
    }
    pub fn f(self) -> f64 {
        match self {
            Num::Int(i) => i as f64,
            Num::Float(x) => x,
        }
    }
    pub fn value(self) -> Value {
        match self {
            Num::Int(i) => Value::from(i),
            Num::Float(x) => Value::from(x),
        }
    }
    pub fn truthy(self) -> bool {
        self.f() != 0.0
    }
}

fn cmp(a: &Num, b: &Num) -> Ordering {
    match (a, b) {
        (Num::Int(x), Num::Int(y)) => x.cmp(y),
        _ => a.f().partial_cmp(&b.f()).unwrap_or(Ordering::Equal),
    }
}

/// `statistics.median`: the middle element itself for an odd count, else the
/// mean of the two middle ones (always a float).
pub fn median(xs: &[Num]) -> Num {
    let mut v = xs.to_vec();
    v.sort_by(cmp);
    let n = v.len();
    assert!(n > 0, "median of nothing");
    if n % 2 == 1 {
        v[n / 2]
    } else {
        Num::Float((v[n / 2 - 1].f() + v[n / 2].f()) / 2.0)
    }
}

/// `min` and `max`: the first smallest or largest element.
pub fn min(xs: &[Num]) -> Num {
    let mut best = xs[0];
    for x in &xs[1..] {
        if cmp(x, &best) == Ordering::Less {
            best = *x;
        }
    }
    best
}

pub fn max(xs: &[Num]) -> Num {
    let mut best = xs[0];
    for x in &xs[1..] {
        if cmp(x, &best) == Ordering::Greater {
            best = *x;
        }
    }
    best
}

/// `json.dumps(v, indent=indent)`, or `json.dumps(v)` (`", "` and `": "`
/// separators on one line) when `indent` is `None`; `ensure_ascii` on.
pub fn dumps(v: &Value, indent: Option<usize>) -> String {
    let mut out = String::new();
    write(&mut out, v, indent, 0);
    out
}

fn write(out: &mut String, v: &Value, indent: Option<usize>, level: usize) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => {
            if n.is_f64() {
                out.push_str(&repr(n.as_f64().unwrap()));
            } else {
                out.push_str(&n.to_string());
            }
        }
        Value::String(s) => string(out, s),
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, x) in a.iter().enumerate() {
                sep(out, i, indent, level + 1);
                write(out, x, indent, level + 1);
            }
            close(out, indent, level);
            out.push(']');
        }
        Value::Object(m) => object(out, m, indent, level),
    }
}

fn object(out: &mut String, m: &Map<String, Value>, indent: Option<usize>, level: usize) {
    if m.is_empty() {
        out.push_str("{}");
        return;
    }
    out.push('{');
    for (i, (k, x)) in m.iter().enumerate() {
        sep(out, i, indent, level + 1);
        string(out, k);
        out.push_str(": ");
        write(out, x, indent, level + 1);
    }
    close(out, indent, level);
    out.push('}');
}

fn sep(out: &mut String, i: usize, indent: Option<usize>, level: usize) {
    match indent {
        Some(n) => {
            if i > 0 {
                out.push(',');
            }
            out.push('\n');
            out.push_str(&" ".repeat(n * level));
        }
        None => {
            if i > 0 {
                out.push_str(", ");
            }
        }
    }
}

fn close(out: &mut String, indent: Option<usize>, level: usize) {
    if let Some(n) = indent {
        out.push('\n');
        out.push_str(&" ".repeat(n * level));
    }
}

fn string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 || (c as u32) > 0x7e => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    out.push_str(&format!("\\u{:04x}", u));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floats() {
        for (x, s) in [
            (1e-5, "1e-05"),
            (0.0001, "0.0001"),
            (1.5e16, "1.5e+16"),
            (1e16, "1e+16"),
            (1234567890123456.0, "1234567890123456.0"),
            (17.0, "17.0"),
            (0.1, "0.1"),
            (70.841, "70.841"),
            (-2.5, "-2.5"),
            (239999431.6, "239999431.6"),
            (1.0 / 3.0, "0.3333333333333333"),
        ] {
            assert_eq!(repr(x), s);
        }
    }

    #[test]
    fn rounding_matches_python() {
        // Python: f"{0.125:.2f}" == "0.12", f"{2.5:.0f}" == "2", f"{0.375:.2f}" == "0.38"
        assert_eq!(fixed(0.125, 2), "0.12");
        assert_eq!(fixed(2.5, 0), "2");
        assert_eq!(fixed(0.375, 2), "0.38");
        assert_eq!(fixed_commas(1234567.5, 0), "1,234,568");
        assert_eq!(int_commas(-1234), "-1,234");
    }
}
