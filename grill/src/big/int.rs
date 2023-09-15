use super::{ten, u64_to_usize};
use crate::error::NumberError;
use num::{pow, BigInt};
use num_rational::BigRational;
use std::str::FromStr;

pub fn parse_int(value: &str) -> Result<BigInt, NumberError> {
    Parser::parse(value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Head,
    Negative,
    Integer,
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
                _ => Error,
            },
            Negative => match c {
                '0'..='9' => Integer,
                _ => Error,
            },
            Integer => match c {
                '0'..='9' => Integer,
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
struct Parser<'a> {
    value: &'a str,
    state: State,
    is_negative: bool,
    integer_index: Option<usize>,
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
            E => {
                self.exponent_index = Some(i);
            }
            Error => {
                if c == '.' {
                    return Err(NumberError::NotAnInteger {
                        value: self.value.to_string(),
                    });
                }
                return Err(NumberError::UnexpectedChar {
                    value: self.value.to_string(),
                    character: c,
                    index: i,
                });
            }
            _ => {}
        }
        Ok(())
    }
    fn parse(value: &'a str) -> Result<BigInt, NumberError> {
        let value = value.trim();
        let mut parser = Parser {
            value,
            state: State::Head,
            integer_index: None,
            exponent_index: None,
            is_negative: false,
        };
        for (i, c) in value.char_indices() {
            parser.next(i, c)?;
        }
        let integer = BigInt::from_str(parser.integer()).unwrap();
        let exponent = parser
            .exponent()
            .map(i64::from_str)
            .transpose()
            .map_err(|err| NumberError::FailedToParseExponent {
                value: value.to_string(),
                source: err,
            })?;
        let mut result = BigRational::from_integer(integer);

        if let Some(exp) = exponent {
            let is_positive = exp.is_positive();
            #[cfg(not(target_pointer_width = "64"))]
            let exp = u64_to_usize(exp.unsigned_abs())?;
            #[cfg(target_pointer_width = "64")]
            let exp = u64_to_usize(exp.unsigned_abs()).unwrap();
            if is_positive {
                result *= pow(ten(), exp);
            } else {
                result /= pow(ten(), exp);
                if !result.is_integer() {
                    return Err(NumberError::NotAnInteger {
                        value: value.to_string(),
                    });
                }
            }
        }
        Ok(result.to_integer())
    }

    fn integer(&self) -> &str {
        let Some(start) = self.integer_index else { return "0" };
        let end = self.exponent_index.unwrap_or(self.value.len());
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
    fn test_parse() {
        let valid_tests = [
            ("123456", BigInt::from(123_456)),
            ("5e10", BigInt::from(50_000_000_000i64)),
        ];
        for (input, expected) in valid_tests {
            let int = parse_int(input).unwrap();
            assert_eq!(int, expected);
        }

        let invalid_tests = [(
            "12.345",
            NumberError::NotAnInteger {
                value: "12.345".to_string(),
            },
        )];

        for (input, expected) in invalid_tests {
            let err = parse_int(input);
            assert_eq!(err, Err(expected));
        }
    }
    #[test]
    fn test_state_changes() {
        let tests = [
            (Head, "-", Negative),
            (Head, "-0", Integer),
            (Head, "-0.", Error),
            (Head, "0.", Error),
            (Head, "10e", E),
            (Head, "-0e-", Exponent),
            (Head, "-0e--", Error),
            (Head, "-0e-0", Exponent),
            (Head, "-0e3", Exponent),
            (Head, "-0e3.", Error),
            (Head, "123.", Error),
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

    #[allow(clippy::unnecessary_box_returns)]
    fn b<T: 'static>(t: T) -> Box<T> {
        Box::new(t)
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

    fn assert_failed_transition(state: State, input: char) {
        assert!(
            std::panic::catch_unwind(|| state.next(input)).is_err(),
            "state: {state:?}\ninput: \'{input:?}\'\nexpected: panic\nresult: {:?}\n* \n",
            state.next(input)
        );
    }
}
