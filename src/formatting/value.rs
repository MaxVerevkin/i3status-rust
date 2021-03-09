use std::fmt;

use super::suffix::Suffix;
use super::unit::Unit;
use super::Variable;

#[derive(Debug, Clone)]
pub struct Value {
    unit: Unit,
    min_width: usize,
    icon: Option<String>,
    value: InternalValue,
}

#[derive(Debug, Clone)]
enum InternalValue {
    Text(String),
    Integer(i64),
    Float(f64),
}

//FIXME: fix confvertation of bytes (2^10 != 10^3)
//FIXME: do not use suffixes smaller than `One` for bytes
fn format_number(raw_value: f64, min_width: usize, min_suffix: &Suffix) -> String {
    let min_exp_level = match min_suffix {
        Suffix::Tera => 4,
        Suffix::Giga => 3,
        Suffix::Mega => 2,
        Suffix::Kilo => 1,
        Suffix::One => 0,
        Suffix::Milli => -1,
        Suffix::Micro => -2,
        Suffix::Nano => -3,
    };

    let exp_level = (raw_value.log10().div_euclid(3.) as i32).clamp(min_exp_level, 4);
    let value = raw_value / (10f64).powi(exp_level * 3);

    let suffix = match exp_level {
        4 => Suffix::Tera,
        3 => Suffix::Giga,
        2 => Suffix::Mega,
        1 => Suffix::Kilo,
        0 => Suffix::One,
        -1 => Suffix::Milli,
        -2 => Suffix::Micro,
        _ => Suffix::Nano,
    };

    // The length of the integer part of a number
    let digits = (value.log10().floor() + 1.0).max(1.0) as isize;
    // How many characters is left for "." and the fractional part?
    match min_width as isize - digits {
        // No characters left
        x if x <= 0 => format!("{:.0}{}", value, suffix),
        // Only one character -> print a trailing dot
        x if x == 1 => format!("{:.0}{}.", value, suffix),
        // There is space for fractional part
        rest => format!("{:.*}{}", (rest as usize) - 1, value, suffix),
    }
}

fn format_bar(value: f64, length: usize) -> String {
    let value = value.clamp(0., 1.);
    let chars_to_fill = value * length as f64;
    (0..length)
        .map(|i| {
            let printed_chars = i as f64;
            let val = (chars_to_fill - printed_chars).clamp(0., 1.) * 8.;
            match val as usize {
                0 => ' ',
                1 => '\u{258f}',
                2 => '\u{258e}',
                3 => '\u{258d}',
                4 => '\u{258c}',
                5 => '\u{258b}',
                6 => '\u{258a}',
                7 => '\u{2589}',
                _ => '\u{2588}',
            }
        })
        .collect()
}

impl Value {
    // Constuctors
    pub fn from_string(text: String) -> Self {
        Self {
            icon: None,
            min_width: 0,
            unit: Unit::None,
            value: InternalValue::Text(text),
        }
    }
    pub fn from_integer(value: i64) -> Self {
        Self {
            icon: None,
            min_width: 2,
            unit: Unit::None,
            value: InternalValue::Integer(value),
        }
    }
    pub fn from_float(value: f64) -> Self {
        Self {
            icon: None,
            min_width: 3,
            unit: Unit::None,
            value: InternalValue::Float(value),
        }
    }

    // Set options
    pub fn icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }
    //pub fn min_width(mut self, min_width: usize) -> Self {
    //self.min_width = min_width;
    //self
    //}

    // Units
    pub fn degrees(mut self) -> Self {
        self.unit = Unit::Degrees;
        self
    }
    pub fn percents(mut self) -> Self {
        self.unit = Unit::Percents;
        self
    }
    pub fn bits_per_second(mut self) -> Self {
        self.unit = Unit::BitsPerSecond;
        self
    }
    pub fn bytes_per_second(mut self) -> Self {
        self.unit = Unit::BytesPerSecond;
        self
    }
    pub fn seconds(mut self) -> Self {
        self.unit = Unit::Seconds;
        self
    }
    pub fn watts(mut self) -> Self {
        self.unit = Unit::Watts;
        self
    }
    pub fn hertz(mut self) -> Self {
        self.unit = Unit::Hertz;
        self
    }
    pub fn bytes(mut self) -> Self {
        self.unit = Unit::Bytes;
        self
    }

    //TODO impl Display
    pub fn format(&self, var: &Variable) -> String {
        let min_width = var.min_width.unwrap_or(self.min_width);
        let pad_with = var.pad_with.unwrap_or(' ');
        let unit = var.unit.as_ref().unwrap_or(&self.unit);

        // Draw the bar instead of usual formatting if `bar_max_value` is set
        // (olny for integers and floats)
        if let Some(bar_max_value) = var.bar_max_value {
            match self.value {
                InternalValue::Integer(i) => {
                    return format_bar(i as f64 / bar_max_value, min_width)
                }
                InternalValue::Float(f) => return format_bar(f / bar_max_value, min_width),
                _ => (),
            }
        }

        let value = match self.value {
            InternalValue::Text(ref text) => {
                let mut text = text.clone();
                let text_len = text.len();
                if text_len < min_width {
                    for _ in text_len..min_width {
                        text.push(pad_with);
                    }
                }
                if let Some(max_width) = var.max_width {
                    text.truncate(max_width);
                }
                text
            }
            InternalValue::Integer(value) => {
                //TODO better way to do it?
                let value = if self.unit == Unit::BytesPerSecond && *unit == Unit::BitsPerSecond {
                    value * 8
                } else if self.unit == Unit::BitsPerSecond && *unit == Unit::BytesPerSecond {
                    value / 8
                } else {
                    value
                };

                let text = value.to_string();
                let mut retval = String::new();
                let text_len = text.len();
                if text_len < min_width {
                    for _ in text_len..min_width {
                        retval.push(pad_with);
                    }
                }
                retval.push_str(&text);
                retval
            }
            InternalValue::Float(value) => {
                //TODO better way to do it?
                let value = if self.unit == Unit::BytesPerSecond && *unit == Unit::BitsPerSecond {
                    value * 8.
                } else if self.unit == Unit::BitsPerSecond && *unit == Unit::BytesPerSecond {
                    value / 8.
                } else {
                    value
                };

                format_number(
                    value,
                    min_width,
                    var.min_suffix.as_ref().unwrap_or(&Suffix::Nano),
                )
            }
        };
        if let Some(ref icon) = self.icon {
            format!("{}{}{}", icon, value, unit)
        } else {
            format!("{}{}", value, unit)
        }
    }
}