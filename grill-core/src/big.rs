//! Big numeric data structures (re-exported from `num` crate) and parsers
//! for `BigInt` and `BigRational`

use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;

pub use num;
pub use num::{BigInt, BigRational};

use num::FromPrimitive;
use once_cell::sync::Lazy;

/// The number ten (10) as a [`BigInt`]
static TEN: Lazy<BigInt> = Lazy::new(|| BigInt::from_u8(10).unwrap());

/// Parses a string into a [`BigInt`]
pub fn parse_int(value: &str) -> Result<BigInt, ParseError> {
    int::Parser::parse(value)
}

/// Parses a string into a [`BigRational`]
pub fn parse_rational(value: &str) -> Result<BigRational, ParseError> {
    rational::Parser::parse(value)
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  ParseError                                  ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// An error occurred while parsing a [`Number`] as a [`num::BigRational`].
#[derive(Debug)]
pub enum ParseError {
    /// Failed to parse exponent of a number.
    FailedToParseExponent(FailedToParseExpontentError),

    /// Unexpected character found in a number.
    UnexpectedChar(UnexpectedCharError),

    /// The number is not an integer.
    NotAnInteger(NotAnIntegerError),

    #[cfg(not(target_pointer_width = "64"))]
    ExponentTooLarge(ExponentTooLargeError),
}

impl From<NotAnIntegerError> for ParseError {
    fn from(err: NotAnIntegerError) -> Self {
        Self::NotAnInteger(err)
    }
}
impl From<UnexpectedCharError> for ParseError {
    fn from(err: UnexpectedCharError) -> Self {
        Self::UnexpectedChar(err)
    }
}
impl From<FailedToParseExpontentError> for ParseError {
    fn from(err: FailedToParseExpontentError) -> Self {
        Self::FailedToParseExponent(err)
    }
}

#[cfg(not(target_pointer_width = "64"))]
impl From<ExponentTooLargeError> for ParseError {
    fn from(err: ExponentTooLargeError) -> Self {
        return Self::ExponentTooLarge(err);
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                            ExponentTooLargeError                             ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[cfg(not(target_pointer_width = "64"))]
#[derive(Debug)]
pub struct ExponentTooLargeError {
    value: u64,
}

#[cfg(not(target_pointer_width = "64"))]
impl Display for ExponentTooLargeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "exponent ({}) exceeds maximum value for non-64-bit architecture",
            self.value
        )
    }
}
#[cfg(not(target_pointer_width = "64"))]
impl ExponentTooLargeError {
    pub fn fail<T>(value: u64) -> Result<T, Self> {
        Err(Self { value })
    }
    pub fn value(&self) -> u64 {
        self.value
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                              NotAnIntegerError                               ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug, PartialEq)]
pub struct NotAnIntegerError {
    /// the value of the string being parsed
    value: String,
}

impl From<String> for NotAnIntegerError {
    fn from(value: String) -> Self {
        Self { value }
    }
}
impl NotAnIntegerError {
    pub fn new(value: String) -> Self {
        Self { value }
    }
    pub fn fail<T>(value: String) -> Result<T, Self> {
        Err(Self { value })
    }
    pub fn value(&self) -> &String {
        &self.value
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                             UnexpectedCharError                              ║
║                            ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug, PartialEq)]
pub struct UnexpectedCharError {
    /// the value of the string being parsed
    value: String,
    /// the character which caused the error
    character: char,
    /// the index of the character which caused the error
    index: usize,
}
impl UnexpectedCharError {
    pub fn fail<T>(value: String, character: char, index: usize) -> Result<T, Self> {
        Err(Self {
            value,
            character,
            index,
        })
    }
    pub fn value(&self) -> &String {
        &self.value
    }
    pub fn character(&self) -> char {
        self.character
    }
    pub fn index(&self) -> usize {
        self.index
    }
}
impl Display for UnexpectedCharError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to parse number \"{}\":\n\tunexpected character: '{}' at index {}",
            self.value, self.character, self.index
        )
    }
}

impl Error for UnexpectedCharError {}

#[derive(Debug)]
pub struct FailedToParseExpontentError {
    /// the value of the string being parsed
    value: String,
    /// the underlying error
    source: ParseIntError,
}
impl FailedToParseExpontentError {
    pub fn new(value: String, source: ParseIntError) -> Self {
        Self { value, source }
    }
    pub fn fail<T>(value: String, source: ParseIntError) -> Result<T, Self> {
        Err(Self { value, source })
    }
    pub fn value(&self) -> &String {
        &self.value
    }
    /// the underlying [`ParseIntError`]
    pub fn source(&self) -> &ParseIntError {
        &self.source
    }
}

impl Error for FailedToParseExpontentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl Display for FailedToParseExpontentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "failed to parse exponent of number \"{}\":\n\t{}",
            self.value, self.source
        )
    }
}

/// Attempts to convert a `u64` to `usize`
///
/// # Errors
/// Returns `value` if the architure is less than 64-bit and the value is too large
#[inline]
pub(crate) fn u64_to_usize(value: u64) -> Result<usize, u64> {
    value.try_into().map_err(|_| value)
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                     int                                      ║
║                                    ¯¯¯¯¯                                     ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
mod int {
    use crate::big::{NotAnIntegerError, UnexpectedCharError};

    use super::{u64_to_usize, TEN};
    use super::{FailedToParseExpontentError, ParseError};
    use num::{pow, BigInt};
    use num_rational::BigRational;
    use std::str::FromStr;

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
        fn next(self, c: char) -> Self {
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
    pub(super) struct Parser<'a> {
        value: &'a str,
        state: State,
        is_negative: bool,
        integer_index: Option<usize>,
        exponent_index: Option<usize>,
    }

    impl<'a> Parser<'a> {
        fn next(&mut self, i: usize, c: char) -> Result<(), ParseError> {
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
                        NotAnIntegerError::fail(self.value.to_string())?;
                    }
                    UnexpectedCharError::fail(self.value.to_string(), c, i)?;
                }
                _ => {}
            }
            Ok(())
        }
        pub(super) fn parse(value: &'a str) -> Result<BigInt, ParseError> {
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
                .map_err(|err| FailedToParseExpontentError::new(value.to_string(), err))?;
            let mut result = BigRational::from_integer(integer);

            if let Some(exp) = exponent {
                let is_positive = exp.is_positive();
                #[cfg(not(target_pointer_width = "64"))]
                let exp = u64_to_usize(exp.unsigned_abs())?;
                #[cfg(target_pointer_width = "64")]
                let exp = u64_to_usize(exp.unsigned_abs()).unwrap();
                if is_positive {
                    result *= pow(TEN.clone(), exp);
                } else {
                    result /= pow(TEN.clone(), exp);
                    if !result.is_integer() {
                        NotAnIntegerError::fail(value.to_string())?;
                    }
                }
            }
            Ok(result.to_integer())
        }

        fn integer(&self) -> &str {
            let Some(start) = self.integer_index else {
                return "0";
            };
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

    /*
      ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
      ╔══════════════════════════════════════════════════════════════════════════════╗
    ║                                                                              ║
    ║                                    tests                                     ║
    ║                                   ¯¯¯¯¯¯¯                                    ║
      ╚══════════════════════════════════════════════════════════════════════════════╝
      ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
      */
    #[cfg(test)]
    mod tests {
        use crate::big::parse_int;

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

            // let _invalid_tests = [("12.345", NotAnIntegerError::new("12.345".into()).into())];

            // for (input, expected) in invalid_tests {
            //     let err = parse_int(input);
            //     assert_eq!(err, Err(expected));
            // }
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
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   rational                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
mod rational {
    use crate::big::UnexpectedCharError;

    use super::{u64_to_usize, FailedToParseExpontentError, ParseError, TEN};
    use std::str::FromStr;

    use num::{pow, BigInt, BigRational, One, Zero};

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
        fn next(&mut self, i: usize, c: char) -> Result<(), ParseError> {
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
                    UnexpectedCharError::fail(self.value.to_string(), c, i)?;
                }
                _ => {}
            }
            Ok(())
        }
        pub(super) fn parse(value: &'a str) -> Result<BigRational, ParseError> {
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
                .map_or(BigInt::one(), |f| pow(TEN.clone(), f.len()));

            let fraction = BigRational::new(fraction, denom);
            let mut result = fraction + integer;
            let exponent = parser
                .exponent()
                .map(i64::from_str)
                .transpose()
                .map_err(|err| FailedToParseExpontentError::new(value.to_string(), err))?;

            if let Some(exp) = exponent {
                let is_positive = exp.is_positive();
                #[cfg(not(target_pointer_width = "64"))]
                let exp = u64_to_usize(exp.unsigned_abs())?;

                #[cfg(target_pointer_width = "64")]
                // safety: usize is the same width as u64 on 64-bit systems
                let exp = u64_to_usize(exp.unsigned_abs()).unwrap();
                if is_positive {
                    result *= pow(TEN.clone(), exp);
                } else {
                    result /= pow(TEN.clone(), exp);
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

    /*
      ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
      ╔══════════════════════════════════════════════════════════════════════════════╗
    ║                                                                              ║
    ║                                    tests                                     ║
    ║                                   ¯¯¯¯¯¯¯                                    ║
      ╚══════════════════════════════════════════════════════════════════════════════╝
      ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
      */
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
}
