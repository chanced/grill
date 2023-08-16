use std::str::FromStr;

use num_rational::BigRational;

lazy_static::lazy_static! {
    static ref TEN: BigInt = BigInt::from_u8(10).unwrap();
}

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
use num::{integer, pow, BigInt, FromPrimitive, One, Signed, Zero};

#[derive(Debug)]
struct ParseNumberError;

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
struct Parser<'a> {
    value: &'a str,
    state: State,
    is_negative: bool,
    integer_idx: Option<usize>,
    fraction_idx: Option<usize>,
    exponent: Option<usize>,
}

impl<'a> Parser<'a> {
    fn next(&mut self, i: usize, c: char) {
        use State::*;
        self.state = self.state.next(c);
        match self.state {
            Negative => {
                self.is_negative = true;
            }
            Integer => {
                if self.integer_idx.is_none() {
                    self.integer_idx = Some(i);
                }
            }
            Fraction => {
                if self.fraction_idx.is_none() {
                    self.fraction_idx = Some(i);
                }
            }
            E => {
                self.exponent = Some(i);
            }
            Error => panic!("error"),
            _ => {}
        }
    }
    fn parse(value: &'a str) {
        let value = value.trim();
        let mut parser = Parser {
            value,
            state: State::Head,
            integer_idx: None,
            fraction_idx: None,
            exponent: None,
            is_negative: false,
        };
        for (i, c) in value.char_indices() {
            parser.next(i, c);
        }
        let integer = BigInt::from_str(parser.integer()).unwrap();
        let fraction = parser
            .fraction()
            .map(|f| BigInt::from_str(f).unwrap())
            .unwrap_or(BigInt::zero());

        let denom = parser
            .fraction()
            .map_or(BigInt::one(), |f| pow(TEN.clone(), f.len()));

        let fraction = BigRational::new(fraction, denom);
        let mut result = fraction + integer;
        if let Some(exp) = parser.exponent().map(|e| i64::from_str(e).unwrap()) {
            if exp.is_positive() {
                result *= pow(TEN.clone(), exp as usize);
            } else {
                result /= pow(TEN.clone(), exp.unsigned_abs() as usize);
            }
        }

        println!("result: {}", result);
    }
    fn fraction(&self) -> Option<&str> {
        let start = self.fraction_idx?;
        let end = self.exponent.unwrap_or(self.value.len());
        Some(&self.value[start + 1..end])
    }

    fn integer(&self) -> &str {
        let Some(start) = self.integer_idx else { return "0" };
        let end = self
            .fraction_idx
            .or(self.exponent)
            .unwrap_or(self.value.len());
        &self.value[start..end]
    }

    fn exponent(&self) -> Option<&str> {
        let e = &self.value[self.exponent? + 1..];
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
