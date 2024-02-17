use super::{ten, u64_to_usize};
use std::str::FromStr;

use num::{pow, BigInt, BigRational, One, Zero};
use snafu::Backtrace;

use crate::error::NumberError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Head,
    Negative,
    Integer,
    Fraction,
    E,
    Exponent,
    Error,
}

impl State {
    fn next(self, c: char) -> State {
        use State::*;
        match self {
            Head => match c {
                ' ' => Head,
                '-' => Negative,
                '0'..='9' => Integer,
                '.' => Fraction,
                _ => Error,
            },
            Negative => match c {
                '0'..='9' => Integer,
                '.' => Fraction,
                _ => Error,
            },
            Integer => match c {
                '0'..='9' => Integer,
                '.' => Fraction,
                'e' | 'E' => E,
                _ => Error,
            },
            Fraction => match c {
                '0'..='9' => Fraction,
                'e' | 'E' => E,
                _ => Error,
            },
            E => match c {
                '-' | '+' | '0'..='9' => Exponent,
                _ => Error,
            },
            Exponent => match c {
                '0'..='9' => Exponent,
                _ => Error,
            },
            Error => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub(super) struct Parser<'a> {
    value: &'a str,
    state: State,
    is_negative: bool,
    integer_index: Option<usize>,
    fraction_index: Option<usize>,
    exponent_index: Option<usize>,
}

impl<'a> Parser<'a> {
    fn next(&mut self, i: usize, c: char) -> Result<(), NumberError> {
        use State::*;
        self.state = self.state.next(c);
        match self.state {
            Negative => {
                self.is_negative = true;
            }
            Integer => {
                if self.integer_index.is_none() {
                    self.integer_index = Some(i);
                }
            }
            Fraction => {
                if self.fraction_index.is_none() {
                    self.fraction_index = Some(i);
                }
            }
            E => {
                self.exponent_index = Some(i);
            }
            Error => {
                return Err(NumberError::UnexpectedChar {
                    value: self.value.to_string(),
                    character: c,
                    index: i,
                    backtrace: Backtrace::capture(),
                })
            }
            _ => {}
        }
        Ok(())
    }
    pub(super) fn parse(value: &'a str) -> Result<BigRational, NumberError> {
        let value = value.trim();
        let mut parser = Parser {
            value,
            state: State::Head,
            integer_index: None,
            fraction_index: None,
            exponent_index: None,
            is_negative: false,
        };
        for (i, c) in value.char_indices() {
            parser.next(i, c)?;
        }
        let integer = BigInt::from_str(parser.integer()).unwrap();
        let fraction = parser
            .fraction()
            .map_or(BigInt::zero(), |f| BigInt::from_str(f).unwrap());

        let denom = parser
            .fraction()
            .map_or(BigInt::one(), |f| pow(ten().clone(), f.len()));

        let fraction = BigRational::new(fraction, denom);
        let mut result = fraction + integer;
        let exponent = parser
            .exponent()
            .map(i64::from_str)
            .transpose()
            .map_err(|err| NumberError::FailedToParseExponent {
                value: value.to_string(),
                source: err,
                backtrace: Backtrace::capture(),
            })?;

        if let Some(exp) = exponent {
            let is_positive = exp.is_positive();
            #[cfg(not(target_pointer_width = "64"))]
            let exp = u64_to_usize(exp.unsigned_abs())?;

            #[cfg(target_pointer_width = "64")]
            // safety: usize is the same width as u64 on 64-bit systems
            let exp = u64_to_usize(exp.unsigned_abs()).unwrap();
            if is_positive {
                result *= pow(ten().clone(), exp);
            } else {
                result /= pow(ten().clone(), exp);
            }
        }
        Ok(result)
    }
    fn fraction(&self) -> Option<&str> {
        let start = self.fraction_index?;
        let end = self.exponent_index.unwrap_or(self.value.len());
        Some(&self.value[start + 1..end])
    }

    fn integer(&self) -> &str {
        let Some(start) = self.integer_index else {
            return "0";
        };
        let end = self
            .fraction_index
            .or(self.exponent_index)
            .unwrap_or(self.value.len());
        &self.value[start..end]
    }

    fn exponent(&self) -> Option<&str> {
        let e = &self.value[self.exponent_index? + 1..];
        if e.is_empty() {
            None
        } else {
            Some(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use State::*;

    #[test]
    fn test_state_changes() {
        let tests = [
            (Head, "-", Negative),
            (Head, "-0", Integer),
            (Head, "-0.", Fraction),
            (Head, "-0.0", Fraction),
            (Head, "-0.0e", E),
            (Head, "-0.0e-", Exponent),
            (Head, "-0.0e--", Error),
            (Head, "-0.0e-0", Exponent),
            (Head, "-0.0e3", Exponent),
            (Head, "-0.0e3.", Error),
            (Head, "123.456", Fraction),
        ];
        for (state, input, expected) in &tests {
            assert_state_change(*state, input, *expected);
        }
    }

    #[test]
    fn test_state_transitions() {
        let tests = [(Head, '-', Negative), (Head, '0', Integer)];
        for (state, input, expected) in &tests {
            assert_transition(*state, *input, *expected);
        }
    }

    fn assert_state_change(state: State, input: &str, expected: State) {
        let result = input.chars().fold(state, State::next);
        assert_eq!(
            result, expected,
            "\n\ninput:\t\t\'{input:?}\'\nexpected:\t{expected:?}\nresult:\t\t{result:?}\n\n"
        );
    }

    fn assert_transition(state: State, input: char, expected: State) {
        let result = state.next(input);
        assert_eq!(
            result, expected,
            "\n\nstate: {state:?}\ninput: \'{input:?}\'\nexpected: {expected:?}\nresult: {result:?}\n\n"
        );
    }
}
