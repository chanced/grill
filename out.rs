#![feature(prelude_import)]
//! # grill-core
//!
#![deny(clippy::all, clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::implicit_hasher, clippy::wildcard_imports)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod big {
    //! Big numeric data structures (re-exported from `num` crate) and parsers
    //! for `BigInt` and `BigRational`
    use std::num::ParseIntError;
    pub use num;
    pub use num::{BigInt, BigRational};
    use num::FromPrimitive;
    use once_cell::sync::Lazy;
    use snafu::{Backtrace, Snafu};
    /// The number ten (10) as a [`BigInt`]
    #[must_use]
    pub fn ten() -> &'static BigInt {
        static TEN: Lazy<BigInt> = Lazy::new(|| BigInt::from_u8(10).unwrap());
        &TEN
    }
    /// Parses a string into a [`BigInt`]
    pub fn parse_int(value: &str) -> Result<BigInt, ParseError> {
        int::Parser::parse(value)
    }
    /// Parses a string into a [`BigRational`]
    pub fn parse_rational(value: &str) -> Result<BigRational, ParseError> {
        rational::Parser::parse(value)
    }
    /// An error occurred while parsing a [`Number`] as a [`num::BigRational`].
    #[snafu(visibility(pub))]
    pub enum ParseError {
        /// Failed to parse exponent of a number.
        #[snafu(display("failed to parse exponent of number \"{value}\":\n\t{source}"))]
        FailedToParseExponent {
            /// the value of the string being parsed
            value: String,
            /// the underlying error
            source: ParseIntError,
            /// backtrace
            backtrace: Backtrace,
        },
        /// Unexpected character found in a number.
        # [snafu (display ("failed to parse number \"{value}\":\n\tunexpected character: '{character}' at index {index}"))]
        UnexpectedChar {
            /// the value of the string being parsed
            value: String,
            /// the character which caused the error
            character: char,
            /// the index of the character which caused the error
            index: usize,
            /// backtrace
            backtrace: Backtrace,
        },
        /// The number is not an integer.
        #[snafu(display("failed to parse number \"{value}\":\n\tnot an integer"))]
        NotAnInteger {
            /// value of string being parsed
            value: String,
            /// backtrace
            backtrace: Backtrace,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ParseError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ParseError::FailedToParseExponent {
                    value: __self_0,
                    source: __self_1,
                    backtrace: __self_2,
                } => ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "FailedToParseExponent",
                    "value",
                    __self_0,
                    "source",
                    __self_1,
                    "backtrace",
                    &__self_2,
                ),
                ParseError::UnexpectedChar {
                    value: __self_0,
                    character: __self_1,
                    index: __self_2,
                    backtrace: __self_3,
                } => ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "UnexpectedChar",
                    "value",
                    __self_0,
                    "character",
                    __self_1,
                    "index",
                    __self_2,
                    "backtrace",
                    &__self_3,
                ),
                ParseError::NotAnInteger {
                    value: __self_0,
                    backtrace: __self_1,
                } => ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "NotAnInteger",
                    "value",
                    __self_0,
                    "backtrace",
                    &__self_1,
                ),
            }
        }
    }
    ///SNAFU context selector for the `ParseError::FailedToParseExponent` variant
    pub struct FailedToParseExponentSnafu<__T0> {
        #[allow(missing_docs)]
        pub value: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for FailedToParseExponentSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "FailedToParseExponentSnafu",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for FailedToParseExponentSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for FailedToParseExponentSnafu<__T0> {
        #[inline]
        fn clone(&self) -> FailedToParseExponentSnafu<__T0> {
            FailedToParseExponentSnafu {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl<__T0> ::snafu::IntoError<ParseError> for FailedToParseExponentSnafu<__T0>
    where
        ParseError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<String>,
    {
        type Source = ParseIntError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> ParseError {
            let error: ParseIntError = (|v| v)(error);
            ParseError::FailedToParseExponent {
                backtrace: {
                    use ::snafu::AsErrorSource;
                    let error = error.as_error_source();
                    ::snafu::GenerateImplicitData::generate_with_source(error)
                },
                source: error,
                value: ::core::convert::Into::into(self.value),
            }
        }
    }
    ///SNAFU context selector for the `ParseError::UnexpectedChar` variant
    pub struct UnexpectedCharSnafu<__T0, __T1, __T2> {
        #[allow(missing_docs)]
        pub value: __T0,
        #[allow(missing_docs)]
        pub character: __T1,
        #[allow(missing_docs)]
        pub index: __T2,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug, __T2: ::core::fmt::Debug>
        ::core::fmt::Debug for UnexpectedCharSnafu<__T0, __T1, __T2>
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "UnexpectedCharSnafu",
                "value",
                &self.value,
                "character",
                &self.character,
                "index",
                &&self.index,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy, __T2: ::core::marker::Copy>
        ::core::marker::Copy for UnexpectedCharSnafu<__T0, __T1, __T2>
    {
    }
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone, __T2: ::core::clone::Clone>
        ::core::clone::Clone for UnexpectedCharSnafu<__T0, __T1, __T2>
    {
        #[inline]
        fn clone(&self) -> UnexpectedCharSnafu<__T0, __T1, __T2> {
            UnexpectedCharSnafu {
                value: ::core::clone::Clone::clone(&self.value),
                character: ::core::clone::Clone::clone(&self.character),
                index: ::core::clone::Clone::clone(&self.index),
            }
        }
    }
    impl<__T0, __T1, __T2> UnexpectedCharSnafu<__T0, __T1, __T2> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        pub fn build(self) -> ParseError
        where
            __T0: ::core::convert::Into<String>,
            __T1: ::core::convert::Into<char>,
            __T2: ::core::convert::Into<usize>,
        {
            ParseError::UnexpectedChar {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                value: ::core::convert::Into::into(self.value),
                character: ::core::convert::Into::into(self.character),
                index: ::core::convert::Into::into(self.index),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        pub fn fail<__T>(self) -> ::core::result::Result<__T, ParseError>
        where
            __T0: ::core::convert::Into<String>,
            __T1: ::core::convert::Into<char>,
            __T2: ::core::convert::Into<usize>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0, __T1, __T2> ::snafu::IntoError<ParseError> for UnexpectedCharSnafu<__T0, __T1, __T2>
    where
        ParseError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<String>,
        __T1: ::core::convert::Into<char>,
        __T2: ::core::convert::Into<usize>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> ParseError {
            ParseError::UnexpectedChar {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                value: ::core::convert::Into::into(self.value),
                character: ::core::convert::Into::into(self.character),
                index: ::core::convert::Into::into(self.index),
            }
        }
    }
    ///SNAFU context selector for the `ParseError::NotAnInteger` variant
    pub struct NotAnIntegerSnafu<__T0> {
        #[allow(missing_docs)]
        pub value: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for NotAnIntegerSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "NotAnIntegerSnafu",
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for NotAnIntegerSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for NotAnIntegerSnafu<__T0> {
        #[inline]
        fn clone(&self) -> NotAnIntegerSnafu<__T0> {
            NotAnIntegerSnafu {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl<__T0> NotAnIntegerSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        pub fn build(self) -> ParseError
        where
            __T0: ::core::convert::Into<String>,
        {
            ParseError::NotAnInteger {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                value: ::core::convert::Into::into(self.value),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        pub fn fail<__T>(self) -> ::core::result::Result<__T, ParseError>
        where
            __T0: ::core::convert::Into<String>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<ParseError> for NotAnIntegerSnafu<__T0>
    where
        ParseError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<String>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> ParseError {
            ParseError::NotAnInteger {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                value: ::core::convert::Into::into(self.value),
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for ParseError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                ParseError::FailedToParseExponent {
                    ref backtrace,
                    ref source,
                    ref value,
                } => __snafu_display_formatter.write_fmt(format_args!(
                    "failed to parse exponent of number \"{1}\":\n\t{0}",
                    source, value
                )),
                ParseError::UnexpectedChar {
                    ref backtrace,
                    ref character,
                    ref index,
                    ref value,
                } => __snafu_display_formatter.write_fmt(format_args!(
                    "failed to parse number \"{2}\":\n\tunexpected character: \'{0}\' at index {1}",
                    character, index, value
                )),
                ParseError::NotAnInteger {
                    ref backtrace,
                    ref value,
                } => __snafu_display_formatter.write_fmt(format_args!(
                    "failed to parse number \"{0}\":\n\tnot an integer",
                    value
                )),
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for ParseError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                ParseError::FailedToParseExponent { .. } => "ParseError :: FailedToParseExponent",
                ParseError::UnexpectedChar { .. } => "ParseError :: UnexpectedChar",
                ParseError::NotAnInteger { .. } => "ParseError :: NotAnInteger",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                ParseError::FailedToParseExponent { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                ParseError::UnexpectedChar { .. } => ::core::option::Option::None,
                ParseError::NotAnInteger { .. } => ::core::option::Option::None,
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                ParseError::FailedToParseExponent { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                ParseError::UnexpectedChar { .. } => ::core::option::Option::None,
                ParseError::NotAnInteger { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for ParseError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                ParseError::FailedToParseExponent { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                ParseError::UnexpectedChar { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                ParseError::NotAnInteger { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
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
    mod int {
        use super::ParseError;
        use super::{ten, u64_to_usize};
        use num::{pow, BigInt};
        use num_rational::BigRational;
        use snafu::Backtrace;
        use std::str::FromStr;
        enum State {
            Head,
            Negative,
            Integer,
            E,
            Exponent,
            Error,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for State {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        State::Head => "Head",
                        State::Negative => "Negative",
                        State::Integer => "Integer",
                        State::E => "E",
                        State::Exponent => "Exponent",
                        State::Error => "Error",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for State {
            #[inline]
            fn clone(&self) -> State {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for State {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for State {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for State {
            #[inline]
            fn eq(&self, other: &State) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for State {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
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
                    Error => ::core::panicking::panic("internal error: entered unreachable code"),
                }
            }
        }
        pub(super) struct Parser<'a> {
            value: &'a str,
            state: State,
            is_negative: bool,
            integer_index: Option<usize>,
            exponent_index: Option<usize>,
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for Parser<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Parser",
                    "value",
                    &self.value,
                    "state",
                    &self.state,
                    "is_negative",
                    &self.is_negative,
                    "integer_index",
                    &self.integer_index,
                    "exponent_index",
                    &&self.exponent_index,
                )
            }
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
                            return Err(ParseError::NotAnInteger {
                                value: self.value.to_string(),
                                backtrace: Backtrace::capture(),
                            });
                        }
                        return Err(ParseError::UnexpectedChar {
                            value: self.value.to_string(),
                            character: c,
                            index: i,
                            backtrace: Backtrace::capture(),
                        });
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
                    .map_err(|err| ParseError::FailedToParseExponent {
                        value: value.to_string(),
                        source: err,
                        backtrace: Backtrace::capture(),
                    })?;
                let mut result = BigRational::from_integer(integer);
                if let Some(exp) = exponent {
                    let is_positive = exp.is_positive();
                    #[cfg(target_pointer_width = "64")]
                    let exp = u64_to_usize(exp.unsigned_abs()).unwrap();
                    if is_positive {
                        result *= pow(ten().clone(), exp);
                    } else {
                        result /= pow(ten().clone(), exp);
                        if !result.is_integer() {
                            return Err(ParseError::NotAnInteger {
                                value: value.to_string(),
                                backtrace: Backtrace::capture(),
                            });
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
    }
    mod rational {
        use super::{ten, u64_to_usize, ParseError};
        use std::str::FromStr;
        use num::{pow, BigInt, BigRational, One, Zero};
        use snafu::Backtrace;
        enum State {
            Head,
            Negative,
            Integer,
            Fraction,
            E,
            Exponent,
            Error,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for State {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        State::Head => "Head",
                        State::Negative => "Negative",
                        State::Integer => "Integer",
                        State::Fraction => "Fraction",
                        State::E => "E",
                        State::Exponent => "Exponent",
                        State::Error => "Error",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for State {
            #[inline]
            fn clone(&self) -> State {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for State {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for State {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for State {
            #[inline]
            fn eq(&self, other: &State) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for State {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
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
                    Error => ::core::panicking::panic("internal error: entered unreachable code"),
                }
            }
        }
        pub(super) struct Parser<'a> {
            value: &'a str,
            state: State,
            is_negative: bool,
            integer_index: Option<usize>,
            fraction_index: Option<usize>,
            exponent_index: Option<usize>,
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for Parser<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "value",
                    "state",
                    "is_negative",
                    "integer_index",
                    "fraction_index",
                    "exponent_index",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.value,
                    &self.state,
                    &self.is_negative,
                    &self.integer_index,
                    &self.fraction_index,
                    &&self.exponent_index,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "Parser", names, values)
            }
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
                        return Err(ParseError::UnexpectedChar {
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
                    .map_or(BigInt::one(), |f| pow(ten().clone(), f.len()));
                let fraction = BigRational::new(fraction, denom);
                let mut result = fraction + integer;
                let exponent = parser
                    .exponent()
                    .map(i64::from_str)
                    .transpose()
                    .map_err(|err| ParseError::FailedToParseExponent {
                        value: value.to_string(),
                        source: err,
                        backtrace: Backtrace::capture(),
                    })?;
                if let Some(exp) = exponent {
                    let is_positive = exp.is_positive();
                    #[cfg(target_pointer_width = "64")]
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
    }
}
pub mod iter {
    //! Various [`Iterator`]s.
    /// TODO: Implement this
    pub struct Ancestors<'i, S, K> {
        _schema: S,
        _key: K,
        _marker: std::marker::PhantomData<&'i ()>,
    }
}
pub mod lang {
    //! Traits and resources for integrating a schema language.
    //!
    //! [`Interrogator`](crate::Interrogator) relies upon implementations of
    //! [`Language`] to compile and evaluate schemas. This `mod` contains the traits
    //! to satisfy that contract.
    //!
    //! ## What's provided
    //! An [`Interrogator`](crate::Interrogator) contains a number of data
    //! structures to facilitate implementing [`language`]:
    //! - [`Schemas`] is a [`SlotMap`](`slotmap::SlotMap`)-backed graph of schemas.
    //! - [`Sources`] is a repository of [`Arc<Value>`](`serde_json::Value`) indexed
    //!   by [`AbsoluteUri`].
    //! - [`Values`] is a cache of [`Arc<Value>`](`serde_json::Value`) indexed by
    //!   [`Value`].
    //! - [`Numbers`] is a cache of [`Arc<BigRational>`](num::BigRational) that will
    //!   also parse [`serde_json::Number`]s.
    //!
    //! ## Compiling a schema
    pub mod cache {
        //! Cache stores for numbers and JSON values.
        use std::{
            collections::HashMap,
            hash::{BuildHasher, Hasher},
            sync::Arc,
        };
        use num::BigRational;
        use once_cell::sync::Lazy;
        use serde_json::{Number, Value};
        use crate::big::{self, parse_rational};
        fn boolean(value: bool) -> Arc<Value> {
            static TRUE: Lazy<Arc<Value>> = Lazy::new(|| Arc::new(Value::Bool(true)));
            static FALSE: Lazy<Arc<Value>> = Lazy::new(|| Arc::new(Value::Bool(false)));
            if value {
                TRUE.clone()
            } else {
                FALSE.clone()
            }
        }
        fn null() -> Arc<Value> {
            static NULL: Lazy<Arc<Value>> = Lazy::new(|| Arc::new(Value::Null));
            NULL.clone()
        }
        type Map<K, V> = HashMap<K, V, LenHasher>;
        /// A cache store of [`Value`]s.
        pub struct Values {
            strings: Vec<Arc<Value>>,
            numbers: Vec<Arc<Value>>,
            objects: Map<usize, Vec<Arc<Value>>>,
            arrays: Map<usize, Vec<Arc<Value>>>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Values {
            #[inline]
            fn clone(&self) -> Values {
                Values {
                    strings: ::core::clone::Clone::clone(&self.strings),
                    numbers: ::core::clone::Clone::clone(&self.numbers),
                    objects: ::core::clone::Clone::clone(&self.objects),
                    arrays: ::core::clone::Clone::clone(&self.arrays),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Values {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Values",
                    "strings",
                    &self.strings,
                    "numbers",
                    &self.numbers,
                    "objects",
                    &self.objects,
                    "arrays",
                    &&self.arrays,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Values {
            #[inline]
            fn default() -> Values {
                Values {
                    strings: ::core::default::Default::default(),
                    numbers: ::core::default::Default::default(),
                    objects: ::core::default::Default::default(),
                    arrays: ::core::default::Default::default(),
                }
            }
        }
        impl Values {
            /// Creates a new [`Values`] cache.
            pub fn new() -> Self {
                Self {
                    strings: Vec::new(),
                    numbers: Vec::new(),
                    objects: Map::default(),
                    arrays: Map::default(),
                }
            }
            /// Returns an `Arc<Value>` representation of `value`, either by returning
            /// an existing cached instance or inserts and returns a new instance.
            #[must_use]
            pub fn get_or_insert(&mut self, value: &Value) -> Arc<Value> {
                match value {
                    Value::Number(_) => self.resolve_number(value),
                    Value::String(_) => self.resolve_string(value),
                    Value::Array(_) => self.resolve_array(value),
                    Value::Object(_) => self.resolve_object(value),
                    Value::Bool(value) => boolean(*value),
                    Value::Null => null(),
                }
            }
            fn resolve_object(&mut self, value: &Value) -> Arc<Value> {
                let object = value.as_object().unwrap();
                let len = object.len();
                let objects = self.objects.entry(len).or_default();
                if let Some(object) = objects.iter().find(|o| o.as_object().unwrap() == object) {
                    return object.clone();
                }
                let value = Arc::new(value.clone());
                objects.push(value.clone());
                value
            }
            fn resolve_array(&mut self, value: &Value) -> Arc<Value> {
                let array = value.as_array().unwrap();
                let len = array.len();
                let arrays = self.arrays.entry(len).or_default();
                if let Some(object) = arrays.iter().find(|o| o.as_array().unwrap() == array) {
                    return object.clone();
                }
                let value = Arc::new(value.clone());
                arrays.push(value.clone());
                value
            }
            fn resolve_string(&mut self, value: &Value) -> Arc<Value> {
                let string = value.as_str().unwrap();
                #[allow(clippy::map_unwrap_or)]
                self.strings
                    .binary_search_by_key(&string, |v| v.as_str().unwrap())
                    .map(|index| self.strings[index].clone())
                    .unwrap_or_else(|index| {
                        self.strings.insert(index, Arc::new(value.clone()));
                        self.strings[index].clone()
                    })
            }
            fn resolve_number(&mut self, value: &Value) -> Arc<Value> {
                let number = value.as_number().unwrap();
                let number = number.as_str();
                #[allow(clippy::map_unwrap_or)]
                self.numbers
                    .binary_search_by_key(&number, |v| {
                        let number = v.as_number().unwrap();
                        number.as_str()
                    })
                    .map(|index| self.numbers[index].clone())
                    .unwrap_or_else(|index| {
                        self.numbers.insert(index, Arc::new(value.clone()));
                        self.numbers[index].clone()
                    })
            }
        }
        /// A cache of numbers parsed as `BigRational`.
        pub struct Numbers {
            rationals: HashMap<String, Arc<BigRational>>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Numbers {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Numbers",
                    "rationals",
                    &&self.rationals,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Numbers {
            #[inline]
            fn default() -> Numbers {
                Numbers {
                    rationals: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Numbers {
            #[inline]
            fn clone(&self) -> Numbers {
                Numbers {
                    rationals: ::core::clone::Clone::clone(&self.rationals),
                }
            }
        }
        impl Numbers {
            /// Creates a new [`Numbers`] cache
            pub fn new() -> Self {
                Self {
                    rationals: HashMap::new(),
                }
            }
            /// Either returns an [`Arc`] to a previously parsed [`BigRational`]
            /// or parses and
            /// returns a reference to the [`BigRational`].
            ///
            /// # Errors
            /// Returns [`big::ParseError`] if the number fails to parse
            pub fn get_or_insert_arc(
                &mut self,
                number: &Number,
            ) -> Result<Arc<BigRational>, big::ParseError> {
                if let Some(existing) = self.rationals.get(number.as_str()) {
                    return Ok(existing.clone());
                }
                let num = Arc::new(parse_rational(number.as_str())?);
                self.rationals.insert(number.to_string(), num.clone());
                Ok(num)
            }
            /// Either returns a reference to a previously parsed [`BigRational`] or parses and
            /// returns a reference to the [`BigRational`].
            ///
            /// # Errors
            /// Returns [`big::ParseError`] if the number fails to parse
            pub fn get_or_insert_ref(
                &mut self,
                number: &Number,
            ) -> Result<&BigRational, big::ParseError> {
                if self.rationals.contains_key(number.as_str()) {
                    return Ok(self.rationals.get(number.as_str()).unwrap().as_ref());
                }
                let n = parse_rational(number.as_str())?;
                self.rationals.insert(number.to_string(), Arc::new(n));
                Ok(self.rationals.get(number.as_str()).unwrap().as_ref())
            }
            /// Returns an [`Arc`] to the [`BigRational`] associated with `value` if it
            /// exists.
            #[must_use]
            pub fn get_arc(&self, number: &Number) -> Option<Arc<BigRational>> {
                self.rationals.get(number.as_str()).cloned()
            }
            /// Returns a reference to the [`BigRational`] associated with `value` if it exists.
            #[must_use]
            pub fn get_ref(&self, number: &Number) -> Option<&BigRational> {
                self.rationals.get(number.as_str()).map(AsRef::as_ref)
            }
            /// Creates an empty [`Numbers`] with at least the specified capacity.
            #[must_use]
            pub fn with_capacity(capacity: usize) -> Numbers {
                Self {
                    rationals: HashMap::with_capacity(capacity),
                }
            }
        }
        struct LenHasher(u64);
        #[automatically_derived]
        impl ::core::clone::Clone for LenHasher {
            #[inline]
            fn clone(&self) -> LenHasher {
                LenHasher(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for LenHasher {
            #[inline]
            fn default() -> LenHasher {
                LenHasher(::core::default::Default::default())
            }
        }
        impl Hasher for LenHasher {
            fn write(&mut self, _bytes: &[u8]) {
                ::core::panicking::panic("internal error: entered unreachable code");
            }
            fn write_usize(&mut self, i: usize) {
                self.0 = i as u64;
            }
            #[inline]
            fn write_u64(&mut self, id: u64) {
                self.0 = id;
            }
            #[inline]
            fn finish(&self) -> u64 {
                self.0
            }
        }
        impl BuildHasher for LenHasher {
            type Hasher = Self;
            fn build_hasher(&self) -> Self::Hasher {
                Self(self.0)
            }
        }
    }
    pub mod schema {
        //! Schema definitions and data structures.
        use grill_uri::AbsoluteUri;
        use slotmap::{new_key_type, Key, SlotMap};
        use snafu::{ensure, Snafu};
        use std::collections::HashMap;
        use super::source::Sources;
        /// Default key type used as a unique identifier for a schema.
        #[repr(transparent)]
        pub struct DefaultKey(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for DefaultKey {}
        #[automatically_derived]
        impl ::core::clone::Clone for DefaultKey {
            #[inline]
            fn clone(&self) -> DefaultKey {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DefaultKey {
            #[inline]
            fn default() -> DefaultKey {
                DefaultKey(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for DefaultKey {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DefaultKey {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DefaultKey {
            #[inline]
            fn eq(&self, other: &DefaultKey) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for DefaultKey {
            #[inline]
            fn cmp(&self, other: &DefaultKey) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for DefaultKey {
            #[inline]
            fn partial_cmp(
                &self,
                other: &DefaultKey,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for DefaultKey {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DefaultKey {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "DefaultKey", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for DefaultKey {
            fn from(k: ::slotmap::KeyData) -> Self {
                DefaultKey(k)
            }
        }
        unsafe impl ::slotmap::Key for DefaultKey {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        /// A trait which indicates that a schema is capable of being embedded in
        /// another schema.
        pub trait EmbeddedIn<K> {
            /// Returns the key of the schema that this schema is embedded in, if any.
            fn embedded_in(&self) -> Option<K>;
        }
        /// A trait which indicates that a schema is capable of having subschemas.
        pub trait Embedded<K> {
            /// Returns a slice of subschema keys for this schema.
            fn embedded(&self) -> &[K];
        }
        /// A trait satisfied by a type that represents a borrowed (but ownable) schema.
        ///
        /// This trait is satisfied by [`Language`](crate::lang::Language)
        /// implementations. See your desired language's documentation for more
        /// information.
        pub trait Schema<'i, K> {
            /// Returns the key of the schema.
            fn key(&self) -> K;
        }
        /// A trait satisfied by a type that represents a compiled schema.
        ///
        /// This trait is satisfied by [`Language`](crate::lang::Language)
        /// implementations. See your desired language's documentation for more
        /// information.
        pub trait CompiledSchema<K>: Clone + PartialEq {
            /// The borrowed schema representation.
            type Schema<'i>: Schema<'i, K>;
            /// Sets the key of the schema.
            fn set_key(&mut self, key: K);
            /// Returns the borrowed [`Self::Schema`] representation.
            fn as_schema<'i>(&self, sources: &Sources) -> Self::Schema<'i>;
        }
        /// A collection of schemas indexed by [`AbsoluteUri`]s.
        pub struct Schemas<S, K: Key> {
            schemas: SlotMap<K, S>,
            keys: HashMap<AbsoluteUri, K>,
        }
        #[automatically_derived]
        impl<S: ::core::fmt::Debug, K: ::core::fmt::Debug + Key> ::core::fmt::Debug for Schemas<S, K> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Schemas",
                    "schemas",
                    &self.schemas,
                    "keys",
                    &&self.keys,
                )
            }
        }
        #[automatically_derived]
        impl<S: ::core::clone::Clone, K: ::core::clone::Clone + Key> ::core::clone::Clone
            for Schemas<S, K>
        {
            #[inline]
            fn clone(&self) -> Schemas<S, K> {
                Schemas {
                    schemas: ::core::clone::Clone::clone(&self.schemas),
                    keys: ::core::clone::Clone::clone(&self.keys),
                }
            }
        }
        impl<S, K: Key> Default for Schemas<S, K> {
            fn default() -> Self {
                Self::new()
            }
        }
        impl<S, K: Key> Schemas<S, K> {
            /// Creates a new schema graph.
            pub fn new() -> Self {
                Self {
                    schemas: SlotMap::with_key(),
                    keys: HashMap::new(),
                }
            }
        }
        impl<S, K> Schemas<S, K>
        where
            S: CompiledSchema<K>,
            K: Key,
        {
            /// Inserts `schema` into the graph and returns its key.
            pub fn insert(&mut self, schema: S) -> K {
                let key = self.schemas.insert(schema);
                self.schemas.get_mut(key).unwrap().set_key(key);
                key
            }
            /// Assigns an `AbsoluteUri` to a schema key.
            ///
            /// # Errors
            /// Returns [`DuplicateLinkError`] if a schema is already linked to the
            /// given `uri`.
            pub fn assign(
                &mut self,
                uri: AbsoluteUri,
                key: K,
            ) -> Result<(), DuplicateLinkError<K>> {
                match self.keys.get(&uri).copied() {
                    Some(existing) => {
                        if !(existing == key) {
                            return DuplicateLinkSnafu { existing, uri }
                                .fail()
                                .map_err(::core::convert::Into::into);
                        }
                    }
                    None => self.insert_uri(uri, key),
                }
                Ok(())
            }
            fn insert_uri(&mut self, uri: AbsoluteUri, key: K) {
                self.keys.insert(uri, key);
            }
            /// Returns [`Self::C::Schema`](CompiledSchema::Schema) for the supplied
            /// [`AbsoluteUri`], if it exists.
            pub fn get_by_uri(&self, uri: &AbsoluteUri) -> Option<&S> {
                self.keys.get(uri).copied().and_then(|k| self.get_by_key(k))
            }
            /// Returns a reference to compiled schema ([`Self::C`](`CompiledSchema`))
            /// with the supplied `key` (``)
            pub fn get_by_key(&self, key: K) -> Option<&S> {
                self.schemas.get(key)
            }
            /// Returns a mutable reference to the schema ([`C`](`CompiledSchema`)) with
            /// the given key.
            pub fn get_mut(&mut self, key: K) -> Option<&mut S> {
                self.schemas.get_mut(key)
            }
        }
        /// A duplicate [`CompiledSchema`] already exists at the given `uri`.
        pub struct DuplicateLinkError<K> {
            /// The URI that the schema is already linked to.
            pub uri: AbsoluteUri,
            /// The key of the existing schema.
            pub existing: K,
        }
        #[automatically_derived]
        impl<K: ::core::fmt::Debug> ::core::fmt::Debug for DuplicateLinkError<K> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DuplicateLinkError",
                    "uri",
                    &self.uri,
                    "existing",
                    &&self.existing,
                )
            }
        }
        #[allow(single_use_lifetimes)]
        impl<K> ::snafu::Error for DuplicateLinkError<K>
        where
            Self: ::core::fmt::Debug + ::core::fmt::Display,
        {
            fn description(&self) -> &str {
                match *self {
                    Self { .. } => "DuplicateLinkError",
                }
            }
            fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
                use ::snafu::AsErrorSource;
                match *self {
                    Self { .. } => ::core::option::Option::None,
                }
            }
            fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
                use ::snafu::AsErrorSource;
                match *self {
                    Self { .. } => ::core::option::Option::None,
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl<K> ::snafu::ErrorCompat for DuplicateLinkError<K> {
            fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
                match *self {
                    Self { .. } => ::core::option::Option::None,
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl<K> ::core::fmt::Display for DuplicateLinkError<K> {
            fn fmt(
                &self,
                __snafu_display_formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                #[allow(unused_variables)]
                match *self {
                    Self {
                        ref existing,
                        ref uri,
                    } => __snafu_display_formatter.write_fmt(format_args!(
                        "A duplicate [`CompiledSchema`] already exists at the given `uri`."
                    )),
                }
            }
        }
        ///SNAFU context selector for the `DuplicateLinkError` error
        struct DuplicateLinkSnafu<__T0, __T1> {
            #[allow(missing_docs)]
            uri: __T0,
            #[allow(missing_docs)]
            existing: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
            for DuplicateLinkSnafu<__T0, __T1>
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DuplicateLinkSnafu",
                    "uri",
                    &self.uri,
                    "existing",
                    &&self.existing,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
            for DuplicateLinkSnafu<__T0, __T1>
        {
        }
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
            for DuplicateLinkSnafu<__T0, __T1>
        {
            #[inline]
            fn clone(&self) -> DuplicateLinkSnafu<__T0, __T1> {
                DuplicateLinkSnafu {
                    uri: ::core::clone::Clone::clone(&self.uri),
                    existing: ::core::clone::Clone::clone(&self.existing),
                }
            }
        }
        impl<__T0, __T1> DuplicateLinkSnafu<__T0, __T1> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            fn build<K>(self) -> DuplicateLinkError<K>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
                __T1: ::core::convert::Into<K>,
            {
                DuplicateLinkError {
                    uri: ::core::convert::Into::into(self.uri),
                    existing: ::core::convert::Into::into(self.existing),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            fn fail<K, __T>(self) -> ::core::result::Result<__T, DuplicateLinkError<K>>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
                __T1: ::core::convert::Into<K>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<K, __T0, __T1> ::snafu::IntoError<DuplicateLinkError<K>> for DuplicateLinkSnafu<__T0, __T1>
        where
            DuplicateLinkError<K>: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
            __T1: ::core::convert::Into<K>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> DuplicateLinkError<K> {
                DuplicateLinkError {
                    uri: ::core::convert::Into::into(self.uri),
                    existing: ::core::convert::Into::into(self.existing),
                }
            }
        }
    }
    pub mod source {
        //! Source repository for JSON Schema documents.
        use std::{
            borrow::Cow,
            collections::{HashMap, VecDeque},
            sync::Arc,
        };
        use grill_uri::AbsoluteUri;
        use jsonptr::Pointer;
        use serde_json::Value;
        use slotmap::{new_key_type, SecondaryMap, SlotMap};
        use snafu::{Backtrace, ResultExt, Snafu};
        /// Key to root documents within [`Sources`]
        #[repr(transparent)]
        pub struct DocumentKey(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for DocumentKey {}
        #[automatically_derived]
        impl ::core::clone::Clone for DocumentKey {
            #[inline]
            fn clone(&self) -> DocumentKey {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DocumentKey {
            #[inline]
            fn default() -> DocumentKey {
                DocumentKey(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for DocumentKey {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DocumentKey {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DocumentKey {
            #[inline]
            fn eq(&self, other: &DocumentKey) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for DocumentKey {
            #[inline]
            fn cmp(&self, other: &DocumentKey) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for DocumentKey {
            #[inline]
            fn partial_cmp(
                &self,
                other: &DocumentKey,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for DocumentKey {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DocumentKey {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "DocumentKey", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for DocumentKey {
            fn from(k: ::slotmap::KeyData) -> Self {
                DocumentKey(k)
            }
        }
        unsafe impl ::slotmap::Key for DocumentKey {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        /// Link to a position within a source document, associated to a
        /// specific URI.
        #[repr(transparent)]
        pub struct SourceKey(::slotmap::KeyData);
        #[automatically_derived]
        impl ::core::marker::Copy for SourceKey {}
        #[automatically_derived]
        impl ::core::clone::Clone for SourceKey {
            #[inline]
            fn clone(&self) -> SourceKey {
                let _: ::core::clone::AssertParamIsClone<::slotmap::KeyData>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for SourceKey {
            #[inline]
            fn default() -> SourceKey {
                SourceKey(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for SourceKey {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::slotmap::KeyData>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SourceKey {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SourceKey {
            #[inline]
            fn eq(&self, other: &SourceKey) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Ord for SourceKey {
            #[inline]
            fn cmp(&self, other: &SourceKey) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for SourceKey {
            #[inline]
            fn partial_cmp(
                &self,
                other: &SourceKey,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for SourceKey {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SourceKey {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "SourceKey", &&self.0)
            }
        }
        impl ::slotmap::__impl::From<::slotmap::KeyData> for SourceKey {
            fn from(k: ::slotmap::KeyData) -> Self {
                SourceKey(k)
            }
        }
        unsafe impl ::slotmap::Key for SourceKey {
            fn data(&self) -> ::slotmap::KeyData {
                self.0
            }
        }
        /// A document within the source repository.
        pub struct Document<'i> {
            key: DocumentKey,
            uri: Cow<'i, AbsoluteUri>,
            value: Arc<Value>,
            links: Cow<'i, [Link]>,
        }
        impl Document<'_> {
            /// The key of the document within the source repository.
            pub fn key(&self) -> DocumentKey {
                self.key
            }
            /// The URI of the document.
            pub fn uri(&self) -> &AbsoluteUri {
                &self.uri
            }
            /// The value of the document.
            pub fn value(&self) -> Arc<Value> {
                self.value.clone()
            }
            /// The links within the document.
            pub fn links(&self) -> &[Link] {
                &self.links
            }
            /// Consumes `self` and returns an owned, 'static variant.
            pub fn into_owned(self) -> Document<'static> {
                Document {
                    key: self.key,
                    uri: Cow::Owned(self.uri.into_owned()),
                    value: self.value,
                    links: Cow::Owned(self.links.into_owned()),
                }
            }
        }
        /// A reference to a location within a source
        pub struct Source<'i> {
            key: SourceKey,
            link: Cow<'i, Link>,
            /// The value of the source
            document: Arc<Value>,
        }
        #[automatically_derived]
        impl<'i> ::core::fmt::Debug for Source<'i> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Source",
                    "key",
                    &self.key,
                    "link",
                    &self.link,
                    "document",
                    &&self.document,
                )
            }
        }
        #[automatically_derived]
        impl<'i> ::core::clone::Clone for Source<'i> {
            #[inline]
            fn clone(&self) -> Source<'i> {
                Source {
                    key: ::core::clone::Clone::clone(&self.key),
                    link: ::core::clone::Clone::clone(&self.link),
                    document: ::core::clone::Clone::clone(&self.document),
                }
            }
        }
        #[automatically_derived]
        impl<'i> ::core::marker::StructuralPartialEq for Source<'i> {}
        #[automatically_derived]
        impl<'i> ::core::cmp::PartialEq for Source<'i> {
            #[inline]
            fn eq(&self, other: &Source<'i>) -> bool {
                self.key == other.key && self.link == other.link && self.document == other.document
            }
        }
        #[automatically_derived]
        impl<'i> ::core::cmp::Eq for Source<'i> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<SourceKey>;
                let _: ::core::cmp::AssertParamIsEq<Cow<'i, Link>>;
                let _: ::core::cmp::AssertParamIsEq<Arc<Value>>;
            }
        }
        impl<'i> Source<'i> {
            fn new(key: SourceKey, link: &'i Link, document: Arc<Value>) -> Self {
                Self {
                    key,
                    link: Cow::Borrowed(link),
                    document,
                }
            }
            /// The [`SourceKey`] of the source
            pub fn key(&self) -> SourceKey {
                self.key
            }
            /// The [`DocumentKey`] of the root document
            pub fn document_key(&self) -> DocumentKey {
                self.link.key
            }
            /// The `AbsoluteUri` of the root document.
            pub fn uri(&self) -> &AbsoluteUri {
                &self.link.uri
            }
            /// The path of the source, as a JSON [`Pointer`], within the root
            /// document.
            pub fn path(&self) -> &Pointer {
                &self.link.path
            }
            /// Returns the `LinkKey` of the source.
            pub fn link_key(&self) -> SourceKey {
                self.key
            }
            /// Returns a reference to the `Link` of this source.
            pub fn link(&self) -> &Link {
                &self.link
            }
            /// The root document of the source as an `Arc<Value>`. Use
            /// [`document_ref`](Self::document_ref) for a reference.
            pub fn document(&self) -> Arc<Value> {
                self.document.clone()
            }
            /// The root document of the source.
            pub fn document_ref(&self) -> &Value {
                &self.document
            }
            /// Resolves source the path within the document, returning the
            /// [`Value`] at the location.
            pub fn resolve(&self) -> &Value {
                self.link.path.resolve(&self.document).unwrap()
            }
            /// Consumes this `Source`` and returns an owned, `'static` variant.
            pub fn into_owned(self) -> Source<'static> {
                Source {
                    key: self.key,
                    link: Cow::Owned(self.link.into_owned()),
                    document: self.document,
                }
            }
        }
        /// A reference to a [`&Value`](`Value`) within [`Sources`]
        pub struct Link {
            /// The URI of the source
            pub uri: AbsoluteUri,
            /// The key of the root document within the [`Sources`] store
            pub key: DocumentKey,
            /// The path within the document
            pub path: Pointer,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Link {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Link",
                    "uri",
                    &self.uri,
                    "key",
                    &self.key,
                    "path",
                    &&self.path,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Link {
            #[inline]
            fn clone(&self) -> Link {
                Link {
                    uri: ::core::clone::Clone::clone(&self.uri),
                    key: ::core::clone::Clone::clone(&self.key),
                    path: ::core::clone::Clone::clone(&self.path),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Link {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Link {
            #[inline]
            fn eq(&self, other: &Link) -> bool {
                self.uri == other.uri && self.key == other.key && self.path == other.path
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Link {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<AbsoluteUri>;
                let _: ::core::cmp::AssertParamIsEq<DocumentKey>;
                let _: ::core::cmp::AssertParamIsEq<Pointer>;
            }
        }
        impl Link {
            /// Instantiates a new `Link`
            fn new(uri: AbsoluteUri, doc_key: DocumentKey, path: Pointer) -> Self {
                Self {
                    uri,
                    key: doc_key,
                    path,
                }
            }
        }
        /// A repository of [`Value`]s indexed by [`AbsoluteUri`]s with interior
        /// indexing of paths by JSON [`Pointer`]s.
        ///
        /// # Example
        /// ```rust
        /// # use std::sync::Arc;
        /// # use grill_uri::AbsoluteUri;
        /// # use jsonptr::Pointer;
        /// # use serde_json::json;
        ///
        /// let base_uri = AbsoluteUri::parse("https://example.com").unwrap();
        ///
        /// let path = Pointer::new(["foo", "bar"]);
        /// let uri = base_uri.with_fragment(Some(&path)).unwrap());
        /// let document = json!({"foo": { "bar": "baz" }});
        ///
        /// let mut sources = Sources::new();
        /// let key = sources.insert(base_uri, Arc::new(document)).unwrap();
        ///
        /// let uri = AbsoluteUri::parse("https://example.com/#/foo/bar").unwrap();
        /// let source = sources.get(&uri).unwrap();
        /// assert_eq!(source.resolve(), &json!("baz"));
        /// ```
        pub struct Sources {
            docs: SlotMap<DocumentKey, Arc<Value>>,
            links: SlotMap<SourceKey, Link>,
            index: HashMap<AbsoluteUri, SourceKey>,
            doc_links: SecondaryMap<DocumentKey, Vec<SourceKey>>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Sources {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Sources",
                    "docs",
                    &self.docs,
                    "links",
                    &self.links,
                    "index",
                    &self.index,
                    "doc_links",
                    &&self.doc_links,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Sources {
            #[inline]
            fn default() -> Sources {
                Sources {
                    docs: ::core::default::Default::default(),
                    links: ::core::default::Default::default(),
                    index: ::core::default::Default::default(),
                    doc_links: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Sources {
            #[inline]
            fn clone(&self) -> Sources {
                Sources {
                    docs: ::core::clone::Clone::clone(&self.docs),
                    links: ::core::clone::Clone::clone(&self.links),
                    index: ::core::clone::Clone::clone(&self.index),
                    doc_links: ::core::clone::Clone::clone(&self.doc_links),
                }
            }
        }
        impl Sources {
            /// Instantiates a new `Sources`
            pub fn new() -> Self {
                Self {
                    docs: SlotMap::with_key(),
                    links: SlotMap::with_key(),
                    index: HashMap::new(),
                    doc_links: SecondaryMap::new(),
                }
            }
            /// Inserts new [`Link`] into the store.
            ///
            /// # Example
            /// ```rust
            /// # use std::sync::Arc;
            /// # use grill_uri::AbsoluteUri;
            /// # use jsonptr::Pointer;
            /// # use serde_json::json;
            /// use grill_core::lang::source::{Sources, Link};
            ///
            /// let document = json!({"foo": { "bar": "baz" }});
            /// let base_uri = AbsoluteUri::must_parse("https://example.com");
            /// let path = Pointer::new(["foo", "bar"]);
            /// let uri = base_uri.with_fragment(Some(&path)).unwrap();
            ///
            /// let mut sources = Sources::new();
            /// // Insert the root document at the base uri
            /// let key = sources.insert(base_uri, Arc::new(document)).unwrap();
            ///
            /// // creates a Link from the uri `https://another.example` to the
            /// // value at `/foo/bar` within the document indexed at `"https://example.com"`.
            /// sources.link(uri, Link { key, path }).unwrap();
            ///
            /// let uri = AbsoluteUri::must_parse("https://another.example");
            /// let source = sources.get(&uri).unwrap();
            /// assert_eq!(source.resolve(), &json!("baz"));
            /// ```
            ///
            /// # Errors
            /// Returns [`LinkError`] if:
            /// - The URI is already linked to a different source.
            /// - The JSON pointer of the link cannot be resolved within the source.
            pub fn link(&mut self, link: Link) -> Result<SourceKey, LinkError> {
                match self.index.get(&link.uri) {
                    None => self.insert_link(link),
                    Some(&existing) => self.handle_duplicate_link(existing, link),
                }
            }
            /// Retrieves a [`Source`] from the store by [`AbsoluteUri`], if a [`Link`]
            /// exists.
            pub fn get<'s>(&'s self, uri: &AbsoluteUri) -> Option<Source<'s>> {
                self.index.get(uri).copied().map(|key| {
                    let link = self.links.get(key).unwrap();
                    Source::new(key, link, self.docs[link.key].clone())
                })
            }
            /// Retrieves the root document [`Value`] by [`SrcKey`].
            pub fn get_document(&self, key: DocumentKey) -> Option<Arc<Value>> {
                self.docs.get(key).cloned()
            }
            /// Retrieves the associated [`Link`] by [`AbsoluteUri`], if it eists.
            pub fn get_link(&self, uri: &AbsoluteUri) -> Option<&Link> {
                self.index
                    .get(uri)
                    .copied()
                    .map(|k| self.links.get(k).unwrap())
            }
            /// Inserts a new source document for the given **absolute** (meaning it
            /// must not contain a fragment) [`AbsoluteUri`] into the repository and
            /// creates applicable [`Link`]s.
            ///
            /// In the event a source is already indexed at the URI, the document must
            /// be the same as the existing document otherwise an error is returned.
            ///
            /// Upon insertion, a [`Link`] is created for the URI as well as
            /// recursively for the entire document.
            ///
            /// ## Example
            /// ```rust
            /// # use std::sync::Arc;
            /// # use grill_uri::AbsoluteUri;
            /// # use serde_json::json;
            ///
            /// let document = Arc::new(json!({"foo": {"bar": "baz"}}));
            /// let uri = AbsoluteUri::must_parse("https://example.com");
            ///
            /// let mut sources = Sources::new();
            /// let key = sources.insert(uri.clone(), document.clone()).unwrap();
            ///
            /// assert_eq!(&sources.get_document(key), &document);
            /// assert_eq!(sources.get(&uri).unwrap().resolve(), &document);
            ///
            /// let uri = AbsoluteUri::must_parse("https://example.com#/foo");
            /// assert_eq!(sources.get(&uri).unwrap().resolve(), "baz");
            ///
            /// ```
            ///
            /// ## Errors
            /// Returns [`InsertError`] if:
            /// - If the URI contains a JSON pointer fragment (e.g.
            ///   `https://example.com#/foo/bar`)
            /// - If the URI is already indexed to a different value
            pub fn insert(
                &mut self,
                absolute_uri: AbsoluteUri,
                document: Arc<Value>,
            ) -> Result<DocumentKey, InsertError> {
                if absolute_uri.has_non_empty_fragment() {
                    return InsertError::fail_not_absolute(absolute_uri, document);
                }
                if self.index.contains_key(&absolute_uri) {
                    return self.check_existing(absolute_uri, document);
                }
                let key = self.docs.insert(document.clone());
                for link in build_links(key, &absolute_uri, &document) {
                    self.insert_link_skip_check(link);
                }
                Ok(key)
            }
            fn check_existing(
                &self,
                uri: AbsoluteUri,
                value: Arc<Value>,
            ) -> Result<DocumentKey, InsertError> {
                let existing_key = self.index.get(&uri).copied().unwrap();
                let existing = self.links.get(existing_key).unwrap();
                let existing_value = &self.docs[existing.key];
                if value.as_ref() != existing_value.as_ref() {
                    InsertError::source_conflict(
                        uri,
                        value,
                        existing.clone(),
                        existing_value.clone(),
                    )
                } else {
                    Ok(existing.key)
                }
            }
            fn insert_link(&mut self, link: Link) -> Result<SourceKey, LinkError> {
                let src = self.docs.get(link.key).unwrap();
                let _ = link
                    .path
                    .resolve(src)
                    .context(ResolutionFailedSnafu)
                    .with_context(|_| LinkSnafu { link: link.clone() })?;
                Ok(self.insert_link_skip_check(link))
            }
            fn insert_link_skip_check(&mut self, link: Link) -> SourceKey {
                let uri = link.uri.clone();
                let doc_key = link.key;
                let src_key = self.links.insert(link);
                self.doc_links
                    .entry(doc_key)
                    .unwrap()
                    .or_default()
                    .push(src_key);
                self.index.insert(uri, src_key);
                src_key
            }
            fn handle_duplicate_link(
                &self,
                existing_key: SourceKey,
                link: Link,
            ) -> Result<SourceKey, LinkError> {
                let existing = self.links.get(existing_key).unwrap();
                if &link != existing {
                    LinkError::fail_confict(link, existing.clone())
                } else {
                    Ok(existing_key)
                }
            }
        }
        /// An error occurred while inserting a source document.
        ///
        /// See [`InsertErrorCause`] for potential causes.
        #[snafu(display("failed to insert source \"{uri}\""))]
        pub struct InsertError {
            /// The [`AbsoluteUri`] attempting to be inserted
            pub uri: AbsoluteUri,
            /// The [`Arc<Value>`](`Value`) attempting to be inserted
            pub document: Arc<Value>,
            /// The cause ([`InsertErrorCause`]) of the error
            #[snafu(source)]
            pub cause: InsertErrorCause,
            /// Backtrace of the error
            pub backtrace: Backtrace,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InsertError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "InsertError",
                    "uri",
                    &self.uri,
                    "document",
                    &self.document,
                    "cause",
                    &self.cause,
                    "backtrace",
                    &&self.backtrace,
                )
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::Error for InsertError
        where
            Self: ::core::fmt::Debug + ::core::fmt::Display,
        {
            fn description(&self) -> &str {
                match *self {
                    Self { .. } => "InsertError",
                }
            }
            fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
                use ::snafu::AsErrorSource;
                match *self {
                    Self { ref cause, .. } => ::core::option::Option::Some(cause.as_error_source()),
                }
            }
            fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
                use ::snafu::AsErrorSource;
                match *self {
                    Self { ref cause, .. } => ::core::option::Option::Some(cause.as_error_source()),
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::ErrorCompat for InsertError {
            fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
                match *self {
                    Self { ref backtrace, .. } => ::snafu::AsBacktrace::as_backtrace(backtrace),
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::core::fmt::Display for InsertError {
            fn fmt(
                &self,
                __snafu_display_formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                #[allow(unused_variables)]
                match *self {
                    Self {
                        ref backtrace,
                        ref cause,
                        ref document,
                        ref uri,
                    } => __snafu_display_formatter
                        .write_fmt(format_args!("failed to insert source \"{0}\"", uri)),
                }
            }
        }
        ///SNAFU context selector for the `InsertError` error
        struct InsertSnafu<__T0, __T1> {
            #[allow(missing_docs)]
            uri: __T0,
            #[allow(missing_docs)]
            document: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
            for InsertSnafu<__T0, __T1>
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "InsertSnafu",
                    "uri",
                    &self.uri,
                    "document",
                    &&self.document,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
            for InsertSnafu<__T0, __T1>
        {
        }
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
            for InsertSnafu<__T0, __T1>
        {
            #[inline]
            fn clone(&self) -> InsertSnafu<__T0, __T1> {
                InsertSnafu {
                    uri: ::core::clone::Clone::clone(&self.uri),
                    document: ::core::clone::Clone::clone(&self.document),
                }
            }
        }
        impl<__T0, __T1> ::snafu::IntoError<InsertError> for InsertSnafu<__T0, __T1>
        where
            InsertError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
            __T1: ::core::convert::Into<Arc<Value>>,
        {
            type Source = InsertErrorCause;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> InsertError {
                let error: InsertErrorCause = (|v| v)(error);
                InsertError {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    cause: error,
                    uri: ::core::convert::Into::into(self.uri),
                    document: ::core::convert::Into::into(self.document),
                }
            }
        }
        impl InsertError {
            /// Returns a `Result<T, Self>::Err(Self)` with an [`InsertErrorCause`]
            /// of [`NotAbsolute`](InsertErrorCause::NotAbsolute).
            pub fn fail_not_absolute<T>(uri: AbsoluteUri, document: Arc<Value>) -> Result<T, Self> {
                NotAbsoluteSnafu
                    .fail()
                    .with_context(|_| InsertSnafu { uri, document })
            }
            /// Returns an `Result<T, Self>::Err(Self)` with an [`InertErrorCause`]
            /// of [`SourceConflict`](InsertErrorCause::SourceConflict).
            pub fn source_conflict<T>(
                uri: AbsoluteUri,
                document: Arc<Value>,
                existing_link: Link,
                existing_value: Arc<Value>,
            ) -> Result<T, Self> {
                SourceConflictSnafu {
                    existing_link,
                    existing_value,
                }
                .fail()
                .with_context(|_| InsertSnafu { uri, document })
            }
        }
        /// Cause of an [`InsertError`].
        pub enum InsertErrorCause {
            /// The [`AbsoluteUri`] provided contained a fragment. Only root documents can be inserted.
            #[snafu(display("source URI must be absolute but contains a fragment",))]
            NotAbsolute,
            /// A source document was attempted to be inserted at an [`AbsoluteUri`]
            /// that is already indexed to a different source document.
            #[snafu(display("URI is indexed to a different source"))]
            SourceConflict {
                /// The existing [`Link`] associated with the URI
                existing_link: Link,
                /// The existing [`Value`] associated with the URI
                existing_value: Arc<Value>,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for InsertErrorCause {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    InsertErrorCause::NotAbsolute => {
                        ::core::fmt::Formatter::write_str(f, "NotAbsolute")
                    }
                    InsertErrorCause::SourceConflict {
                        existing_link: __self_0,
                        existing_value: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "SourceConflict",
                        "existing_link",
                        __self_0,
                        "existing_value",
                        &__self_1,
                    ),
                }
            }
        }
        ///SNAFU context selector for the `InsertErrorCause::NotAbsolute` variant
        struct NotAbsoluteSnafu;
        #[automatically_derived]
        impl ::core::fmt::Debug for NotAbsoluteSnafu {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "NotAbsoluteSnafu")
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for NotAbsoluteSnafu {}
        #[automatically_derived]
        impl ::core::clone::Clone for NotAbsoluteSnafu {
            #[inline]
            fn clone(&self) -> NotAbsoluteSnafu {
                *self
            }
        }
        impl NotAbsoluteSnafu {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            fn build(self) -> InsertErrorCause {
                InsertErrorCause::NotAbsolute {}
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            fn fail<__T>(self) -> ::core::result::Result<__T, InsertErrorCause> {
                ::core::result::Result::Err(self.build())
            }
        }
        impl ::snafu::IntoError<InsertErrorCause> for NotAbsoluteSnafu
        where
            InsertErrorCause: ::snafu::Error + ::snafu::ErrorCompat,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> InsertErrorCause {
                InsertErrorCause::NotAbsolute {}
            }
        }
        ///SNAFU context selector for the `InsertErrorCause::SourceConflict` variant
        struct SourceConflictSnafu<__T0, __T1> {
            #[allow(missing_docs)]
            existing_link: __T0,
            #[allow(missing_docs)]
            existing_value: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
            for SourceConflictSnafu<__T0, __T1>
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SourceConflictSnafu",
                    "existing_link",
                    &self.existing_link,
                    "existing_value",
                    &&self.existing_value,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
            for SourceConflictSnafu<__T0, __T1>
        {
        }
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
            for SourceConflictSnafu<__T0, __T1>
        {
            #[inline]
            fn clone(&self) -> SourceConflictSnafu<__T0, __T1> {
                SourceConflictSnafu {
                    existing_link: ::core::clone::Clone::clone(&self.existing_link),
                    existing_value: ::core::clone::Clone::clone(&self.existing_value),
                }
            }
        }
        impl<__T0, __T1> SourceConflictSnafu<__T0, __T1> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            fn build(self) -> InsertErrorCause
            where
                __T0: ::core::convert::Into<Link>,
                __T1: ::core::convert::Into<Arc<Value>>,
            {
                InsertErrorCause::SourceConflict {
                    existing_link: ::core::convert::Into::into(self.existing_link),
                    existing_value: ::core::convert::Into::into(self.existing_value),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            fn fail<__T>(self) -> ::core::result::Result<__T, InsertErrorCause>
            where
                __T0: ::core::convert::Into<Link>,
                __T1: ::core::convert::Into<Arc<Value>>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1> ::snafu::IntoError<InsertErrorCause> for SourceConflictSnafu<__T0, __T1>
        where
            InsertErrorCause: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<Link>,
            __T1: ::core::convert::Into<Arc<Value>>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> InsertErrorCause {
                InsertErrorCause::SourceConflict {
                    existing_link: ::core::convert::Into::into(self.existing_link),
                    existing_value: ::core::convert::Into::into(self.existing_value),
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::core::fmt::Display for InsertErrorCause {
            fn fmt(
                &self,
                __snafu_display_formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                #[allow(unused_variables)]
                match *self {
                    InsertErrorCause::NotAbsolute {} => __snafu_display_formatter.write_fmt(
                        format_args!("source URI must be absolute but contains a fragment"),
                    ),
                    InsertErrorCause::SourceConflict {
                        ref existing_link,
                        ref existing_value,
                    } => __snafu_display_formatter
                        .write_fmt(format_args!("URI is indexed to a different source")),
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::Error for InsertErrorCause
        where
            Self: ::core::fmt::Debug + ::core::fmt::Display,
        {
            fn description(&self) -> &str {
                match *self {
                    InsertErrorCause::NotAbsolute { .. } => "InsertErrorCause :: NotAbsolute",
                    InsertErrorCause::SourceConflict { .. } => "InsertErrorCause :: SourceConflict",
                }
            }
            fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
                use ::snafu::AsErrorSource;
                match *self {
                    InsertErrorCause::NotAbsolute { .. } => ::core::option::Option::None,
                    InsertErrorCause::SourceConflict { .. } => ::core::option::Option::None,
                }
            }
            fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
                use ::snafu::AsErrorSource;
                match *self {
                    InsertErrorCause::NotAbsolute { .. } => ::core::option::Option::None,
                    InsertErrorCause::SourceConflict { .. } => ::core::option::Option::None,
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::ErrorCompat for InsertErrorCause {
            fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
                match *self {
                    InsertErrorCause::NotAbsolute { .. } => ::core::option::Option::None,
                    InsertErrorCause::SourceConflict { .. } => ::core::option::Option::None,
                }
            }
        }
        /// An error occurred while inserting a [`Link`].
        ///
        /// See [`LinkErrorCause`] for potential causes of this error.
        # [snafu (display ("failed to link source \"{}\"" , link . uri))]
        pub struct LinkError {
            /// The [`Link`] attempting to be inserted
            pub link: Link,
            /// The [`cause`](LinkErrorCause) of the error
            #[snafu(source)]
            pub cause: LinkErrorCause,
            /// Backtrace of the error
            pub backtrace: Backtrace,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LinkError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "LinkError",
                    "link",
                    &self.link,
                    "cause",
                    &self.cause,
                    "backtrace",
                    &&self.backtrace,
                )
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::Error for LinkError
        where
            Self: ::core::fmt::Debug + ::core::fmt::Display,
        {
            fn description(&self) -> &str {
                match *self {
                    Self { .. } => "LinkError",
                }
            }
            fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
                use ::snafu::AsErrorSource;
                match *self {
                    Self { ref cause, .. } => ::core::option::Option::Some(cause.as_error_source()),
                }
            }
            fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
                use ::snafu::AsErrorSource;
                match *self {
                    Self { ref cause, .. } => ::core::option::Option::Some(cause.as_error_source()),
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::ErrorCompat for LinkError {
            fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
                match *self {
                    Self { ref backtrace, .. } => ::snafu::AsBacktrace::as_backtrace(backtrace),
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::core::fmt::Display for LinkError {
            fn fmt(
                &self,
                __snafu_display_formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                #[allow(unused_variables)]
                match *self {
                    Self {
                        ref backtrace,
                        ref cause,
                        ref link,
                    } => __snafu_display_formatter
                        .write_fmt(format_args!("failed to link source \"{0}\"", link.uri)),
                }
            }
        }
        ///SNAFU context selector for the `LinkError` error
        struct LinkSnafu<__T0> {
            #[allow(missing_docs)]
            link: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for LinkSnafu<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "LinkSnafu",
                    "link",
                    &&self.link,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for LinkSnafu<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for LinkSnafu<__T0> {
            #[inline]
            fn clone(&self) -> LinkSnafu<__T0> {
                LinkSnafu {
                    link: ::core::clone::Clone::clone(&self.link),
                }
            }
        }
        impl<__T0> ::snafu::IntoError<LinkError> for LinkSnafu<__T0>
        where
            LinkError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<Link>,
        {
            type Source = LinkErrorCause;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> LinkError {
                let error: LinkErrorCause = (|v| v)(error);
                LinkError {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    cause: error,
                    link: ::core::convert::Into::into(self.link),
                }
            }
        }
        impl LinkError {
            /// Returns an `Result<T, Self>::Err(Self)` with a [`LinkErrorCause`]
            /// of [`Conflict`](LinkErrorCause::Conflict).
            pub fn fail_confict<T>(link: Link, existing: Link) -> Result<T, Self> {
                ConflictSnafu { existing }
                    .fail()
                    .with_context(|_| LinkSnafu { link })
            }
        }
        /// Underlying cause of a [`LinkError`].
        #[snafu(visibility(pub(super)))]
        pub enum LinkErrorCause {
            /// The JSON pointer of the [`Link`] could not be resolved within the source.
            #[snafu(display("failed to resolve JSON pointer of sourced document: {source}"))]
            ResolutionFailed {
                /// Error encountered by while attempting to resolve the json pointer
                source: jsonptr::Error,
            },
            /// The URI is already linked to a different source.
            #[snafu(display("URI is already linked to a different source."))]
            Conflict {
                /// The existing [`Link`] associated with the URI
                existing: Link,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LinkErrorCause {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    LinkErrorCause::ResolutionFailed { source: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "ResolutionFailed",
                            "source",
                            &__self_0,
                        )
                    }
                    LinkErrorCause::Conflict { existing: __self_0 } => {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f, "Conflict", "existing", &__self_0,
                        )
                    }
                }
            }
        }
        ///SNAFU context selector for the `LinkErrorCause::ResolutionFailed` variant
        pub(super) struct ResolutionFailedSnafu;
        #[automatically_derived]
        impl ::core::fmt::Debug for ResolutionFailedSnafu {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "ResolutionFailedSnafu")
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ResolutionFailedSnafu {}
        #[automatically_derived]
        impl ::core::clone::Clone for ResolutionFailedSnafu {
            #[inline]
            fn clone(&self) -> ResolutionFailedSnafu {
                *self
            }
        }
        impl ::snafu::IntoError<LinkErrorCause> for ResolutionFailedSnafu
        where
            LinkErrorCause: ::snafu::Error + ::snafu::ErrorCompat,
        {
            type Source = jsonptr::Error;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> LinkErrorCause {
                let error: jsonptr::Error = (|v| v)(error);
                LinkErrorCause::ResolutionFailed { source: error }
            }
        }
        ///SNAFU context selector for the `LinkErrorCause::Conflict` variant
        pub(super) struct ConflictSnafu<__T0> {
            #[allow(missing_docs)]
            pub(super) existing: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for ConflictSnafu<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ConflictSnafu",
                    "existing",
                    &&self.existing,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for ConflictSnafu<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for ConflictSnafu<__T0> {
            #[inline]
            fn clone(&self) -> ConflictSnafu<__T0> {
                ConflictSnafu {
                    existing: ::core::clone::Clone::clone(&self.existing),
                }
            }
        }
        impl<__T0> ConflictSnafu<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub(super) fn build(self) -> LinkErrorCause
            where
                __T0: ::core::convert::Into<Link>,
            {
                LinkErrorCause::Conflict {
                    existing: ::core::convert::Into::into(self.existing),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub(super) fn fail<__T>(self) -> ::core::result::Result<__T, LinkErrorCause>
            where
                __T0: ::core::convert::Into<Link>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<LinkErrorCause> for ConflictSnafu<__T0>
        where
            LinkErrorCause: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<Link>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> LinkErrorCause {
                LinkErrorCause::Conflict {
                    existing: ::core::convert::Into::into(self.existing),
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::core::fmt::Display for LinkErrorCause {
            fn fmt(
                &self,
                __snafu_display_formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                #[allow(unused_variables)]
                match *self {
                    LinkErrorCause::ResolutionFailed { ref source } => __snafu_display_formatter
                        .write_fmt(format_args!(
                            "failed to resolve JSON pointer of sourced document: {0}",
                            source
                        )),
                    LinkErrorCause::Conflict { ref existing } => __snafu_display_formatter
                        .write_fmt(format_args!("URI is already linked to a different source.")),
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::Error for LinkErrorCause
        where
            Self: ::core::fmt::Debug + ::core::fmt::Display,
        {
            fn description(&self) -> &str {
                match *self {
                    LinkErrorCause::ResolutionFailed { .. } => "LinkErrorCause :: ResolutionFailed",
                    LinkErrorCause::Conflict { .. } => "LinkErrorCause :: Conflict",
                }
            }
            fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
                use ::snafu::AsErrorSource;
                match *self {
                    LinkErrorCause::ResolutionFailed { ref source, .. } => {
                        ::core::option::Option::Some(source.as_error_source())
                    }
                    LinkErrorCause::Conflict { .. } => ::core::option::Option::None,
                }
            }
            fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
                use ::snafu::AsErrorSource;
                match *self {
                    LinkErrorCause::ResolutionFailed { ref source, .. } => {
                        ::core::option::Option::Some(source.as_error_source())
                    }
                    LinkErrorCause::Conflict { .. } => ::core::option::Option::None,
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::ErrorCompat for LinkErrorCause {
            fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
                match *self {
                    LinkErrorCause::ResolutionFailed { .. } => ::core::option::Option::None,
                    LinkErrorCause::Conflict { .. } => ::core::option::Option::None,
                }
            }
        }
        /// A source was not found at the given URI.
        #[snafu(display("source not found: {}", uri))]
        pub struct NotFoundError {
            /// The URI that was not found.
            pub uri: AbsoluteUri,
            /// The backtrace.
            pub backtrace: Backtrace,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NotFoundError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "NotFoundError",
                    "uri",
                    &self.uri,
                    "backtrace",
                    &&self.backtrace,
                )
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::Error for NotFoundError
        where
            Self: ::core::fmt::Debug + ::core::fmt::Display,
        {
            fn description(&self) -> &str {
                match *self {
                    Self { .. } => "NotFoundError",
                }
            }
            fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
                use ::snafu::AsErrorSource;
                match *self {
                    Self { .. } => ::core::option::Option::None,
                }
            }
            fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
                use ::snafu::AsErrorSource;
                match *self {
                    Self { .. } => ::core::option::Option::None,
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::snafu::ErrorCompat for NotFoundError {
            fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
                match *self {
                    Self { ref backtrace, .. } => ::snafu::AsBacktrace::as_backtrace(backtrace),
                }
            }
        }
        #[allow(single_use_lifetimes)]
        impl ::core::fmt::Display for NotFoundError {
            fn fmt(
                &self,
                __snafu_display_formatter: &mut ::core::fmt::Formatter,
            ) -> ::core::fmt::Result {
                #[allow(unused_variables)]
                match *self {
                    Self {
                        ref backtrace,
                        ref uri,
                    } => __snafu_display_formatter
                        .write_fmt(format_args!("source not found: {0}", uri)),
                }
            }
        }
        ///SNAFU context selector for the `NotFoundError` error
        struct NotFoundSnafu<__T0> {
            #[allow(missing_docs)]
            uri: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for NotFoundSnafu<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "NotFoundSnafu",
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for NotFoundSnafu<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for NotFoundSnafu<__T0> {
            #[inline]
            fn clone(&self) -> NotFoundSnafu<__T0> {
                NotFoundSnafu {
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0> NotFoundSnafu<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            fn build(self) -> NotFoundError
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                NotFoundError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            fn fail<__T>(self) -> ::core::result::Result<__T, NotFoundError>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<NotFoundError> for NotFoundSnafu<__T0>
        where
            NotFoundError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> NotFoundError {
                NotFoundError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
        impl NotFoundError {
            /// Returns a new `Result<T, Self>::Err(Self)` with the given URI.
            pub fn new(uri: AbsoluteUri) -> Self {
                NotFoundSnafu { uri }.build()
            }
        }
        fn build_links<'i>(
            doc_key: DocumentKey,
            base_uri: &'i AbsoluteUri,
            document: &'i Value,
        ) -> BuildLinks<'i> {
            BuildLinks::new(doc_key, base_uri, document)
        }
        struct BuildLinks<'i> {
            doc_key: DocumentKey,
            base_uri: &'i AbsoluteUri,
            path_finder: FindPaths<'i>,
        }
        impl<'i> BuildLinks<'i> {
            fn new(doc_key: DocumentKey, base_uri: &'i AbsoluteUri, document: &'i Value) -> Self {
                let path_finder = FindPaths::new(document);
                Self {
                    doc_key,
                    base_uri,
                    path_finder,
                }
            }
        }
        impl Iterator for BuildLinks<'_> {
            type Item = Link;
            fn next(&mut self) -> Option<Self::Item> {
                let path = self.path_finder.next()?;
                let uri = self.base_uri.with_fragment(Some(&path)).unwrap();
                Some(Link::new(uri, self.doc_key, path))
            }
        }
        struct FindPaths<'v> {
            queue: VecDeque<(Pointer, &'v Value)>,
        }
        impl<'v> FindPaths<'v> {
            fn new(value: &'v Value) -> Self {
                let mut queue = VecDeque::new();
                queue.push_back((Pointer::default(), value));
                Self { queue }
            }
        }
        impl<'v> Iterator for FindPaths<'v> {
            type Item = Pointer;
            fn next(&mut self) -> Option<Self::Item> {
                let (path, value) = self.queue.pop_front()?;
                match value {
                    Value::Object(map) => {
                        for (key, value) in map.iter().rev() {
                            let mut ptr = path.clone();
                            ptr.push_back(key.into());
                            self.queue.push_back((ptr, value));
                        }
                    }
                    Value::Array(array) => {
                        for (i, value) in array.iter().enumerate().rev() {
                            let mut ptr = path.clone();
                            ptr.push_back(i.into());
                            self.queue.push_back((ptr, value));
                        }
                    }
                    _ => {}
                }
                Some(path)
            }
        }
    }
    pub use {
        cache::{Numbers, Values},
        schema::Schemas,
        source::Sources,
    };
    use crate::Resolve;
    use grill_uri::AbsoluteUri;
    use serde_json::Value;
    use slotmap::Key;
    use std::fmt::Debug;
    /// A trait which defines how to compile and evaluate a schema against a
    /// [`Value`].
    ///
    /// See the [`mod`](crate::lang) for more information.
    pub trait Language<K>: Sized + Clone + Debug + Send
    where
        K: 'static + Key,
    {
        /// The [`CompiledSchema`](schema::CompiledSchema) of this language.
        type CompiledSchema: schema::CompiledSchema<K>;
        /// The error type possibly returned from [`compile`](Language::compile).
        type CompileError;
        /// The result type returned from [`evaluate`](Language::evaluate).
        type EvaluateResult<'v>;
        /// Context type supplied to `evaluate`.
        ///
        /// For example, `grill-json-schema` uses an `enum` to represent the desired
        /// format of the output.
        type Context;
        /// The error type that can be returned when initializing the language.
        type InitError;
        /// Initializes the language with the given [`Init`] request.
        fn init(&mut self, init: Init<'_, Self::CompiledSchema, K>) -> Result<(), Self::InitError>;
        /// Compiles a schema for the given [`Compile`] request and returns the key,
        /// if successful.
        ///
        /// This method is `async` to allow for languages that need to fetch schemas
        /// during compilation.
        ///
        /// # Errors
        /// Returns [`Self::CompileError`] if the schema could not be compiled.
        fn compile<'i, R: Resolve + Send + Sync>(
            &'i mut self,
            compile: Compile<'i, Self::CompiledSchema, R, K>,
        ) -> impl ::core::future::Future<Output = Result<K, Self::CompileError>> + Send;
        /// Compiles all schemas for the given [`CompileAll`] request and returns the
        /// keys, if successful.
        fn compile_all<'i, R: Resolve + Send + Sync>(
            &'i mut self,
            compile_all: CompileAll<'i, Self::CompiledSchema, R, K>,
        ) -> impl ::core::future::Future<Output = Result<Vec<K>, Self::CompileError>> + Send;
        /// Evaluates a schema for the given [`Evaluate`] request.
        fn evaluate<'i, 'v>(
            &'i self,
            eval: Evaluate<'i, 'v, Self::CompiledSchema, Self::Context, K>,
        ) -> Self::EvaluateResult<'v>;
    }
    /// Request to initialize a language.
    pub struct Init<'i, S, K: Key> {
        /// Schema graph
        pub schemas: &'i mut Schemas<S, K>,
        /// Source repository
        pub sources: &'i mut Sources,
        /// Number cache
        pub numbers: &'i mut Numbers,
        /// Values cache
        pub values: &'i mut Values,
    }
    /// Request to compile a schema.
    pub struct Compile<'i, S, R, K: Key> {
        /// The URI of the schema to compile
        pub uri: AbsoluteUri,
        /// Schema graph
        pub schemas: &'i mut Schemas<S, K>,
        /// Source repository
        pub sources: &'i mut Sources,
        /// Number cache
        pub numbers: &'i mut Numbers,
        /// Values cache
        pub values: &'i mut Values,
        /// Implementation of [`Resolve`]
        pub resolve: &'i R,
    }
    /// Request to compile a schema.
    pub struct CompileAll<'i, S, R, K: Key> {
        /// The URI of the schema to compile
        pub uris: Vec<AbsoluteUri>,
        /// Schema graph
        pub schemas: &'i mut Schemas<S, K>,
        /// Source repository
        pub sources: &'i mut Sources,
        /// Number cache
        pub numbers: &'i mut Numbers,
        /// Values cache
        pub values: &'i mut Values,
        /// Implementation of [`Resolve`]
        pub resolve: &'i R,
    }
    /// Request to evaluate a schema.
    pub struct Evaluate<'i, 'v, S, X, K: Key> {
        /// Evaluation context `S::Context`
        pub context: X,
        /// The key of the schema to evaluate
        pub key: K,
        /// The value to evaluate
        pub value: &'v Value,
        /// Schema graph
        pub schemas: &'i Schemas<S, K>,
        /// Source repository
        pub sources: &'i Sources,
        /// Read-only access to global (to the `Interrogator`) cache of [`Numbers`]     
        pub numbers_cache: &'i Numbers,
        /// Read-only access to global (to the `Interrogator`) cache of [`Values`]
        pub values_cache: &'i Values,
        /// [`Numbers`] local to this evaluation
        pub numbers: &'i mut Numbers,
        /// [`Values`] local to this evaluation
        pub values: &'i mut Values,
    }
}
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};
pub use lang::{schema::DefaultKey, Language};
pub use slotmap::{new_key_type, Key};
use grill_uri::AbsoluteUri;
use lang::{
    source::{self, NotFoundError},
    Compile, Evaluate, Numbers, Schemas, Sources, Values,
};
use serde_json::Value;
/// A trait for resolving and deserializing a [`Value`] at a given [`AbsoluteUri`].
pub trait Resolve: Send {
    /// The error type that can be returned when resolving a [`Value`].
    type Error;
    /// Resolves and deserializes a [`Value`] at the supplied [`AbsoluteUri`].
    ///
    /// # Errors
    /// Returns [`Self::Error`] if an error occurs during resolution.
    fn resolve(
        &self,
        uri: &AbsoluteUri,
    ) -> impl ::core::future::Future<Output = Result<Arc<Value>, Self::Error>> + Send;
}
#[doc("path = ", std::collections::HashMap)]
const I: () = ();
impl Resolve for () {
    type Error = source::NotFoundError;
    async fn resolve(&self, uri: &AbsoluteUri) -> Result<Arc<Value>, Self::Error> {
        Err(source::NotFoundError::new(uri.clone()))
    }
}
/// Type alias for `()` which implements [`Resolve`] by always returning
/// [`NotFoundError`], thus relying entirely on documents added as sources
/// to the [`Interrogator`].
pub type Internal = ();
/// Evaluates the integrity of data through a schema language.
pub struct Interrogator<L: Language<K>, K: 'static + Key = DefaultKey> {
    lang: L,
    schemas: Schemas<L::CompiledSchema, K>,
    sources: Sources,
    values: Values,
    numbers: Numbers,
}
impl<L: Language<K>, K: Key> Interrogator<L, K> {
    fn init(mut self) -> Result<Self, L::InitError> {
        self.lang.init(lang::Init {
            schemas: &mut self.schemas,
            sources: &mut self.sources,
            numbers: &mut self.numbers,
            values: &mut self.values,
        })?;
        Ok(self)
    }
    /// Creates a new `Interrogator`.
    pub fn new(lang: L) -> Result<Self, L::InitError> {
        Self {
            lang,
            schemas: Schemas::new(),
            sources: Sources::new(),
            values: Values::new(),
            numbers: Numbers::new(),
        }
        .init()
    }
    /// Compiles a schema for the given [`Compile`] request and returns the key,
    /// if successful.
    ///
    /// This method is `async` to allow for languages that need to fetch schemas
    /// during compilation.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    pub async fn compile<R>(
        &mut self,
        schema_uri: AbsoluteUri,
        resolve: &R,
    ) -> Result<K, L::CompileError>
    where
        R: Resolve + Sync,
    {
        let mut sources = self.sources.clone();
        let mut schemas = self.schemas.clone();
        let c = Compile {
            uri: schema_uri,
            schemas: &mut schemas,
            sources: &mut sources,
            numbers: &mut self.numbers,
            values: &mut self.values,
            resolve,
        };
        let key = self.lang.compile(c).await?;
        self.schemas = schemas;
        self.sources = sources;
        Ok(key)
    }
    /// Evaluates a schema for the given [`Evaluate`] request.
    pub fn evaluate<'i, 'v>(
        &'i self,
        schema: K,
        context: L::Context,
        value: &'v Value,
    ) -> L::EvaluateResult<'v> {
        let mut numbers = Numbers::new();
        let mut values = Values::new();
        self.lang.evaluate(Evaluate {
            context,
            key: schema,
            value,
            schemas: &self.schemas,
            sources: &self.sources,
            numbers_cache: &self.numbers,
            values_cache: &self.values,
            numbers: &mut numbers,
            values: &mut values,
        })
    }
}
