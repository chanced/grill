pub mod error {
    //! Logical errors which can occur while interacting this library.
    //!
    //!
    //!
    use grill_uri::error::Error;
    use jsonptr::Pointer;
    #[doc(no_inline)]
    pub use jsonptr::{Error as ResolvePointerError, MalformedPointerError};
    use snafu::Backtrace;
    use snafu::Snafu;
    use std::collections::HashMap;
    use std::error;
    use crate::Key;
    use crate::{schema::Anchor, uri::AbsoluteUri, uri::Uri};
    use serde_json::Value;
    use std::{
        error::Error as StdError, fmt::{self, Debug, Display},
        num::ParseIntError, ops::Deref, string::FromUtf8Error,
    };
    pub trait CompileError: From<
            CyclicDependencyError,
        > + From<UnknownAnchorError> + error::Error + Send + Sync + 'static {}
    /// An issue with an anchor keyword (e.g. `$anchor`, `$dynamicAnchor`,
    /// `$recursiveAnchor`) occurred
    #[snafu(visibility(pub), context(suffix(Ctx)), module)]
    pub enum AnchorError {
        /// An anchor keyword which does not allow for empty values (e.g. `$anchor`,
        /// `$dynamicAnchor`) was found with an empty string.
        #[snafu(display("{keyword} must be a non-empty string"))]
        Empty { keyword: &'static str, backtrace: Backtrace },
        /// An anchor keyword which does not allow for non-empty values (e.g.
        /// `$recursiveAnchor`) was found with a value.
        #[snafu(display("{keyword} must be an empty string; found {value}"))]
        ValueNotAllowed {
            /// The [`Keyword`] of the anchor.
            keyword: &'static str,
            /// The value of the anchor.
            value: Box<Value>,
            backtrace: Backtrace,
        },
        /// `$anchor` and `$dynamicAnchor` must start with either a letter
        /// (`([A-Za-z])`) or an underscore (`_`).
        #[snafu(
            display(
                "{keyword} must start with either a letter (([A-Za-z])) or an underscore (_); found {value} for {char}"
            )
        )]
        InvalidLeadingCharacter {
            /// The value of the anchor.
            value: String,
            /// The [`Keyword`] of the anchor.
            keyword: &'static str,
            /// The character which caused the error.
            char: char,
            backtrace: Backtrace,
        },
        /// An anchor keyword contained an invalid character.
        ///
        /// `$anchor` and `$dynamicAnchor` may only contain letters (`([A-Za-z])`),
        /// digits (`[0-9]`), hyphens (`'-'`), underscores (`'_'`), and periods
        /// (`'.'`).
        #[snafu(
            display(
                "{keyword} may only contain letters (([A-Za-z])), digits ([0-9]), hyphens ('-'), underscores ('_'), and periods ('.'); found {value} for {char}"
            )
        )]
        InvalidChar {
            /// The value of the anchor.
            value: String,
            /// The [`Keyword`] of the anchor.
            keyword: &'static str,
            /// The character which caused the error.
            char: char,
            backtrace: Backtrace,
        },
        /// The anchor value was not of the expected type.
        #[snafu(display("invalid anchor: {}", source))]
        InvalidType { source: InvalidTypeError, backtrace: Backtrace },
        #[snafu(transparent, context(false))]
        Duplicate { #[snafu(backtrace)] source: DuplicateAnchorError },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AnchorError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AnchorError::Empty { keyword: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Empty",
                        "keyword",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                AnchorError::ValueNotAllowed {
                    keyword: __self_0,
                    value: __self_1,
                    backtrace: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ValueNotAllowed",
                        "keyword",
                        __self_0,
                        "value",
                        __self_1,
                        "backtrace",
                        &__self_2,
                    )
                }
                AnchorError::InvalidLeadingCharacter {
                    value: __self_0,
                    keyword: __self_1,
                    char: __self_2,
                    backtrace: __self_3,
                } => {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "InvalidLeadingCharacter",
                        "value",
                        __self_0,
                        "keyword",
                        __self_1,
                        "char",
                        __self_2,
                        "backtrace",
                        &__self_3,
                    )
                }
                AnchorError::InvalidChar {
                    value: __self_0,
                    keyword: __self_1,
                    char: __self_2,
                    backtrace: __self_3,
                } => {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "InvalidChar",
                        "value",
                        __self_0,
                        "keyword",
                        __self_1,
                        "char",
                        __self_2,
                        "backtrace",
                        &__self_3,
                    )
                }
                AnchorError::InvalidType { source: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "InvalidType",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                AnchorError::Duplicate { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Duplicate",
                        "source",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub mod anchor_error {
        use super::*;
        ///SNAFU context selector for the `AnchorError::Empty` variant
        pub struct EmptyCtx<__T0> {
            #[allow(missing_docs)]
            pub keyword: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for EmptyCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "EmptyCtx",
                    "keyword",
                    &&self.keyword,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for EmptyCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for EmptyCtx<__T0> {
            #[inline]
            fn clone(&self) -> EmptyCtx<__T0> {
                EmptyCtx {
                    keyword: ::core::clone::Clone::clone(&self.keyword),
                }
            }
        }
        impl<__T0> EmptyCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> AnchorError
            where
                __T0: ::core::convert::Into<&'static str>,
            {
                AnchorError::Empty {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    keyword: ::core::convert::Into::into(self.keyword),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, AnchorError>
            where
                __T0: ::core::convert::Into<&'static str>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<AnchorError> for EmptyCtx<__T0>
        where
            AnchorError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<&'static str>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> AnchorError {
                AnchorError::Empty {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    keyword: ::core::convert::Into::into(self.keyword),
                }
            }
        }
        ///SNAFU context selector for the `AnchorError::ValueNotAllowed` variant
        pub struct ValueNotAllowedCtx<__T0, __T1> {
            #[allow(missing_docs)]
            pub keyword: __T0,
            #[allow(missing_docs)]
            pub value: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
        for ValueNotAllowedCtx<__T0, __T1> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ValueNotAllowedCtx",
                    "keyword",
                    &self.keyword,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
        for ValueNotAllowedCtx<__T0, __T1> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
        for ValueNotAllowedCtx<__T0, __T1> {
            #[inline]
            fn clone(&self) -> ValueNotAllowedCtx<__T0, __T1> {
                ValueNotAllowedCtx {
                    keyword: ::core::clone::Clone::clone(&self.keyword),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        impl<__T0, __T1> ValueNotAllowedCtx<__T0, __T1> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> AnchorError
            where
                __T0: ::core::convert::Into<&'static str>,
                __T1: ::core::convert::Into<Box<Value>>,
            {
                AnchorError::ValueNotAllowed {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    keyword: ::core::convert::Into::into(self.keyword),
                    value: ::core::convert::Into::into(self.value),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, AnchorError>
            where
                __T0: ::core::convert::Into<&'static str>,
                __T1: ::core::convert::Into<Box<Value>>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1> ::snafu::IntoError<AnchorError>
        for ValueNotAllowedCtx<__T0, __T1>
        where
            AnchorError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<&'static str>,
            __T1: ::core::convert::Into<Box<Value>>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> AnchorError {
                AnchorError::ValueNotAllowed {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    keyword: ::core::convert::Into::into(self.keyword),
                    value: ::core::convert::Into::into(self.value),
                }
            }
        }
        ///SNAFU context selector for the `AnchorError::InvalidLeadingCharacter` variant
        pub struct InvalidLeadingCharacterCtx<__T0, __T1, __T2> {
            #[allow(missing_docs)]
            pub value: __T0,
            #[allow(missing_docs)]
            pub keyword: __T1,
            #[allow(missing_docs)]
            pub char: __T2,
        }
        #[automatically_derived]
        impl<
            __T0: ::core::fmt::Debug,
            __T1: ::core::fmt::Debug,
            __T2: ::core::fmt::Debug,
        > ::core::fmt::Debug for InvalidLeadingCharacterCtx<__T0, __T1, __T2> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "InvalidLeadingCharacterCtx",
                    "value",
                    &self.value,
                    "keyword",
                    &self.keyword,
                    "char",
                    &&self.char,
                )
            }
        }
        #[automatically_derived]
        impl<
            __T0: ::core::marker::Copy,
            __T1: ::core::marker::Copy,
            __T2: ::core::marker::Copy,
        > ::core::marker::Copy for InvalidLeadingCharacterCtx<__T0, __T1, __T2> {}
        #[automatically_derived]
        impl<
            __T0: ::core::clone::Clone,
            __T1: ::core::clone::Clone,
            __T2: ::core::clone::Clone,
        > ::core::clone::Clone for InvalidLeadingCharacterCtx<__T0, __T1, __T2> {
            #[inline]
            fn clone(&self) -> InvalidLeadingCharacterCtx<__T0, __T1, __T2> {
                InvalidLeadingCharacterCtx {
                    value: ::core::clone::Clone::clone(&self.value),
                    keyword: ::core::clone::Clone::clone(&self.keyword),
                    char: ::core::clone::Clone::clone(&self.char),
                }
            }
        }
        impl<__T0, __T1, __T2> InvalidLeadingCharacterCtx<__T0, __T1, __T2> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> AnchorError
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<&'static str>,
                __T2: ::core::convert::Into<char>,
            {
                AnchorError::InvalidLeadingCharacter {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                    keyword: ::core::convert::Into::into(self.keyword),
                    char: ::core::convert::Into::into(self.char),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, AnchorError>
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<&'static str>,
                __T2: ::core::convert::Into<char>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1, __T2> ::snafu::IntoError<AnchorError>
        for InvalidLeadingCharacterCtx<__T0, __T1, __T2>
        where
            AnchorError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<String>,
            __T1: ::core::convert::Into<&'static str>,
            __T2: ::core::convert::Into<char>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> AnchorError {
                AnchorError::InvalidLeadingCharacter {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                    keyword: ::core::convert::Into::into(self.keyword),
                    char: ::core::convert::Into::into(self.char),
                }
            }
        }
        ///SNAFU context selector for the `AnchorError::InvalidChar` variant
        pub struct InvalidCharCtx<__T0, __T1, __T2> {
            #[allow(missing_docs)]
            pub value: __T0,
            #[allow(missing_docs)]
            pub keyword: __T1,
            #[allow(missing_docs)]
            pub char: __T2,
        }
        #[automatically_derived]
        impl<
            __T0: ::core::fmt::Debug,
            __T1: ::core::fmt::Debug,
            __T2: ::core::fmt::Debug,
        > ::core::fmt::Debug for InvalidCharCtx<__T0, __T1, __T2> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "InvalidCharCtx",
                    "value",
                    &self.value,
                    "keyword",
                    &self.keyword,
                    "char",
                    &&self.char,
                )
            }
        }
        #[automatically_derived]
        impl<
            __T0: ::core::marker::Copy,
            __T1: ::core::marker::Copy,
            __T2: ::core::marker::Copy,
        > ::core::marker::Copy for InvalidCharCtx<__T0, __T1, __T2> {}
        #[automatically_derived]
        impl<
            __T0: ::core::clone::Clone,
            __T1: ::core::clone::Clone,
            __T2: ::core::clone::Clone,
        > ::core::clone::Clone for InvalidCharCtx<__T0, __T1, __T2> {
            #[inline]
            fn clone(&self) -> InvalidCharCtx<__T0, __T1, __T2> {
                InvalidCharCtx {
                    value: ::core::clone::Clone::clone(&self.value),
                    keyword: ::core::clone::Clone::clone(&self.keyword),
                    char: ::core::clone::Clone::clone(&self.char),
                }
            }
        }
        impl<__T0, __T1, __T2> InvalidCharCtx<__T0, __T1, __T2> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> AnchorError
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<&'static str>,
                __T2: ::core::convert::Into<char>,
            {
                AnchorError::InvalidChar {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                    keyword: ::core::convert::Into::into(self.keyword),
                    char: ::core::convert::Into::into(self.char),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, AnchorError>
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<&'static str>,
                __T2: ::core::convert::Into<char>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1, __T2> ::snafu::IntoError<AnchorError>
        for InvalidCharCtx<__T0, __T1, __T2>
        where
            AnchorError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<String>,
            __T1: ::core::convert::Into<&'static str>,
            __T2: ::core::convert::Into<char>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> AnchorError {
                AnchorError::InvalidChar {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                    keyword: ::core::convert::Into::into(self.keyword),
                    char: ::core::convert::Into::into(self.char),
                }
            }
        }
        ///SNAFU context selector for the `AnchorError::InvalidType` variant
        pub struct InvalidTypeCtx;
        #[automatically_derived]
        impl ::core::fmt::Debug for InvalidTypeCtx {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "InvalidTypeCtx")
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for InvalidTypeCtx {}
        #[automatically_derived]
        impl ::core::clone::Clone for InvalidTypeCtx {
            #[inline]
            fn clone(&self) -> InvalidTypeCtx {
                *self
            }
        }
        impl ::snafu::IntoError<AnchorError> for InvalidTypeCtx
        where
            AnchorError: ::snafu::Error + ::snafu::ErrorCompat,
        {
            type Source = InvalidTypeError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> AnchorError {
                let error: InvalidTypeError = (|v| v)(error);
                AnchorError::InvalidType {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                }
            }
        }
        impl ::core::convert::From<DuplicateAnchorError> for AnchorError {
            #[track_caller]
            fn from(error: DuplicateAnchorError) -> Self {
                let error: DuplicateAnchorError = (|v| v)(error);
                AnchorError::Duplicate {
                    source: error,
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for AnchorError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                AnchorError::Empty { ref backtrace, ref keyword } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!("{0} must be a non-empty string", keyword),
                        )
                }
                AnchorError::ValueNotAllowed {
                    ref backtrace,
                    ref keyword,
                    ref value,
                } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "{0} must be an empty string; found {1}", keyword, value
                            ),
                        )
                }
                AnchorError::InvalidLeadingCharacter {
                    ref backtrace,
                    ref char,
                    ref keyword,
                    ref value,
                } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "{1} must start with either a letter (([A-Za-z])) or an underscore (_); found {2} for {0}",
                                char, keyword, value
                            ),
                        )
                }
                AnchorError::InvalidChar {
                    ref backtrace,
                    ref char,
                    ref keyword,
                    ref value,
                } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "{1} may only contain letters (([A-Za-z])), digits ([0-9]), hyphens (\'-\'), underscores (\'_\'), and periods (\'.\'); found {2} for {0}",
                                char, keyword, value
                            ),
                        )
                }
                AnchorError::InvalidType { ref backtrace, ref source } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("invalid anchor: {0}", source))
                }
                AnchorError::Duplicate { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for AnchorError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                AnchorError::Empty { .. } => "AnchorError :: Empty",
                AnchorError::ValueNotAllowed { .. } => "AnchorError :: ValueNotAllowed",
                AnchorError::InvalidLeadingCharacter { .. } => {
                    "AnchorError :: InvalidLeadingCharacter"
                }
                AnchorError::InvalidChar { .. } => "AnchorError :: InvalidChar",
                AnchorError::InvalidType { .. } => "AnchorError :: InvalidType",
                AnchorError::Duplicate { .. } => "AnchorError :: Duplicate",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                AnchorError::Empty { .. } => ::core::option::Option::None,
                AnchorError::ValueNotAllowed { .. } => ::core::option::Option::None,
                AnchorError::InvalidLeadingCharacter { .. } => {
                    ::core::option::Option::None
                }
                AnchorError::InvalidChar { .. } => ::core::option::Option::None,
                AnchorError::InvalidType { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                AnchorError::Duplicate { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                AnchorError::Empty { .. } => ::core::option::Option::None,
                AnchorError::ValueNotAllowed { .. } => ::core::option::Option::None,
                AnchorError::InvalidLeadingCharacter { .. } => {
                    ::core::option::Option::None
                }
                AnchorError::InvalidChar { .. } => ::core::option::Option::None,
                AnchorError::InvalidType { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                AnchorError::Duplicate { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for AnchorError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                AnchorError::Empty { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                AnchorError::ValueNotAllowed { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                AnchorError::InvalidLeadingCharacter { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                AnchorError::InvalidChar { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                AnchorError::InvalidType { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                AnchorError::Duplicate { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
            }
        }
    }
    #[snafu(module, display("duplicate anchor found: \"{}\"", existing.name))]
    pub struct DuplicateAnchorError {
        pub existing: Anchor,
        pub duplicate: Anchor,
        pub backtrace: Backtrace,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DuplicateAnchorError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "DuplicateAnchorError",
                "existing",
                &self.existing,
                "duplicate",
                &self.duplicate,
                "backtrace",
                &&self.backtrace,
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for DuplicateAnchorError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "DuplicateAnchorError",
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
    impl ::snafu::ErrorCompat for DuplicateAnchorError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for DuplicateAnchorError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self { ref backtrace, ref duplicate, ref existing } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "duplicate anchor found: \"{0}\"", existing.name
                            ),
                        )
                }
            }
        }
    }
    mod duplicate_anchor_error {
        use super::*;
        ///SNAFU context selector for the `DuplicateAnchorError` error
        pub(super) struct DuplicateAnchorSnafu<__T0, __T1> {
            #[allow(missing_docs)]
            pub(super) existing: __T0,
            #[allow(missing_docs)]
            pub(super) duplicate: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
        for DuplicateAnchorSnafu<__T0, __T1> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DuplicateAnchorSnafu",
                    "existing",
                    &self.existing,
                    "duplicate",
                    &&self.duplicate,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
        for DuplicateAnchorSnafu<__T0, __T1> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
        for DuplicateAnchorSnafu<__T0, __T1> {
            #[inline]
            fn clone(&self) -> DuplicateAnchorSnafu<__T0, __T1> {
                DuplicateAnchorSnafu {
                    existing: ::core::clone::Clone::clone(&self.existing),
                    duplicate: ::core::clone::Clone::clone(&self.duplicate),
                }
            }
        }
        impl<__T0, __T1> DuplicateAnchorSnafu<__T0, __T1> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub(super) fn build(self) -> DuplicateAnchorError
            where
                __T0: ::core::convert::Into<Anchor>,
                __T1: ::core::convert::Into<Anchor>,
            {
                DuplicateAnchorError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    existing: ::core::convert::Into::into(self.existing),
                    duplicate: ::core::convert::Into::into(self.duplicate),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub(super) fn fail<__T>(
                self,
            ) -> ::core::result::Result<__T, DuplicateAnchorError>
            where
                __T0: ::core::convert::Into<Anchor>,
                __T1: ::core::convert::Into<Anchor>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1> ::snafu::IntoError<DuplicateAnchorError>
        for DuplicateAnchorSnafu<__T0, __T1>
        where
            DuplicateAnchorError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<Anchor>,
            __T1: ::core::convert::Into<Anchor>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> DuplicateAnchorError {
                DuplicateAnchorError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    existing: ::core::convert::Into::into(self.existing),
                    duplicate: ::core::convert::Into::into(self.duplicate),
                }
            }
        }
    }
    /// An error occurred while attempting to add a new a schema source.
    #[snafu(visibility(pub), context(suffix(Ctx)), module)]
    pub enum SourceError {
        /// An error occurred while attempting to deserialize a source.
        #[snafu(display("failed to deserialize source \"{uri}\":\n\t{source}"))]
        DeserializationFailed {
            /// The [`AbsoluteUri`] of the source.
            uri: AbsoluteUri,
            /// The underlying [`DeserializeError`].
            source: DeserializeError,
            backtrace: Backtrace,
        },
        /// Resolution of a source failed
        #[snafu(transparent)]
        ResolutionFailed { #[snafu(backtrace)] source: ResolveErrors },
        /// The source was not valid UTF-8.
        #[snafu(display("source is not valid UTF-8: {source}"))]
        InvalidUtf8 { source: FromUtf8Error, uri: AbsoluteUri, backtrace: Backtrace },
        /// The source's URI was not able to be parsed
        #[snafu(display("failed to parse source URI: {source}"))]
        UriFailedToParse { #[snafu(backtrace)] source: Error },
        /// The source URI contains afragment which is not allowed.
        #[snafu(display("source URIs may not contain fragments, found \"{uri}\""))]
        UnexpectedUriFragment { uri: AbsoluteUri, backtrace: Backtrace },
        /// A JSON Pointer failed to parse or resolve.
        #[snafu(display("failed to parse json pointer: {source}"))]
        PointerFailedToParse { source: MalformedPointerError, backtrace: Backtrace },
        /// A JSON Pointer failed to resolve.
        #[snafu(display("failed to resolve json pointer: {source}"))]
        PointerFailedToResolve { source: ResolvePointerError, backtrace: Backtrace },
        /// A conflict occurred (i.e. a source was linked from multiple locations).
        #[snafu(
            display(
                "source address {:?} @ {:?} already assigned to {:?}",
                uri,
                new_path,
                existing_path
            )
        )]
        SchemaConflict {
            uri: AbsoluteUri,
            /// The existing schema location.
            existing_path: Pointer,
            /// The new schema location.
            new_path: Pointer,
            backtrace: Backtrace,
        },
        SourceConflict { uri: AbsoluteUri, backtrace: snafu::Backtrace },
        /// Failed to resolve a path
        #[snafu(display("failed to resolve link path: {source}"))]
        PathNotFound { source: jsonptr::Error, backtrace: Backtrace },
        /// Failed to resolve a URI
        #[snafu(display("source not found: \"{uri}\""))]
        SourceNotFound { uri: AbsoluteUri, backtrace: Backtrace },
        /// An unknown anchor (non-pointer fragment of a URI) was encountered
        #[snafu(display("unknown anchor: \"{anchor}\" in URI \"{uri}\""))]
        UnknownAnchor {
            /// The anchor which was not found.
            anchor: String,
            /// The URI of the keyword which referenced the anchor.
            uri: AbsoluteUri,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SourceError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                SourceError::DeserializationFailed {
                    uri: __self_0,
                    source: __self_1,
                    backtrace: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "DeserializationFailed",
                        "uri",
                        __self_0,
                        "source",
                        __self_1,
                        "backtrace",
                        &__self_2,
                    )
                }
                SourceError::ResolutionFailed { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "ResolutionFailed",
                        "source",
                        &__self_0,
                    )
                }
                SourceError::InvalidUtf8 {
                    source: __self_0,
                    uri: __self_1,
                    backtrace: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "InvalidUtf8",
                        "source",
                        __self_0,
                        "uri",
                        __self_1,
                        "backtrace",
                        &__self_2,
                    )
                }
                SourceError::UriFailedToParse { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UriFailedToParse",
                        "source",
                        &__self_0,
                    )
                }
                SourceError::UnexpectedUriFragment {
                    uri: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "UnexpectedUriFragment",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                SourceError::PointerFailedToParse {
                    source: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "PointerFailedToParse",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                SourceError::PointerFailedToResolve {
                    source: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "PointerFailedToResolve",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                SourceError::SchemaConflict {
                    uri: __self_0,
                    existing_path: __self_1,
                    new_path: __self_2,
                    backtrace: __self_3,
                } => {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "SchemaConflict",
                        "uri",
                        __self_0,
                        "existing_path",
                        __self_1,
                        "new_path",
                        __self_2,
                        "backtrace",
                        &__self_3,
                    )
                }
                SourceError::SourceConflict { uri: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "SourceConflict",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                SourceError::PathNotFound { source: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "PathNotFound",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                SourceError::SourceNotFound { uri: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "SourceNotFound",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                SourceError::UnknownAnchor { anchor: __self_0, uri: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "UnknownAnchor",
                        "anchor",
                        __self_0,
                        "uri",
                        &__self_1,
                    )
                }
            }
        }
    }
    pub mod source_error {
        use super::*;
        ///SNAFU context selector for the `SourceError::DeserializationFailed` variant
        pub struct DeserializationFailedCtx<__T0> {
            #[allow(missing_docs)]
            pub uri: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug
        for DeserializationFailedCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "DeserializationFailedCtx",
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy
        for DeserializationFailedCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone
        for DeserializationFailedCtx<__T0> {
            #[inline]
            fn clone(&self) -> DeserializationFailedCtx<__T0> {
                DeserializationFailedCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0> ::snafu::IntoError<SourceError> for DeserializationFailedCtx<__T0>
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = DeserializeError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                let error: DeserializeError = (|v| v)(error);
                SourceError::DeserializationFailed {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
        impl ::core::convert::From<ResolveErrors> for SourceError {
            #[track_caller]
            fn from(error: ResolveErrors) -> Self {
                let error: ResolveErrors = (|v| v)(error);
                SourceError::ResolutionFailed {
                    source: error,
                }
            }
        }
        ///SNAFU context selector for the `SourceError::InvalidUtf8` variant
        pub struct InvalidUtf8Ctx<__T0> {
            #[allow(missing_docs)]
            pub uri: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for InvalidUtf8Ctx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "InvalidUtf8Ctx",
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for InvalidUtf8Ctx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for InvalidUtf8Ctx<__T0> {
            #[inline]
            fn clone(&self) -> InvalidUtf8Ctx<__T0> {
                InvalidUtf8Ctx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0> ::snafu::IntoError<SourceError> for InvalidUtf8Ctx<__T0>
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = FromUtf8Error;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                let error: FromUtf8Error = (|v| v)(error);
                SourceError::InvalidUtf8 {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
        ///SNAFU context selector for the `SourceError::UriFailedToParse` variant
        pub struct UriFailedToParseCtx;
        #[automatically_derived]
        impl ::core::fmt::Debug for UriFailedToParseCtx {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "UriFailedToParseCtx")
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for UriFailedToParseCtx {}
        #[automatically_derived]
        impl ::core::clone::Clone for UriFailedToParseCtx {
            #[inline]
            fn clone(&self) -> UriFailedToParseCtx {
                *self
            }
        }
        impl ::snafu::IntoError<SourceError> for UriFailedToParseCtx
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
        {
            type Source = Error;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                let error: Error = (|v| v)(error);
                SourceError::UriFailedToParse {
                    source: error,
                }
            }
        }
        ///SNAFU context selector for the `SourceError::UnexpectedUriFragment` variant
        pub struct UnexpectedUriFragmentCtx<__T0> {
            #[allow(missing_docs)]
            pub uri: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug
        for UnexpectedUriFragmentCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "UnexpectedUriFragmentCtx",
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy
        for UnexpectedUriFragmentCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone
        for UnexpectedUriFragmentCtx<__T0> {
            #[inline]
            fn clone(&self) -> UnexpectedUriFragmentCtx<__T0> {
                UnexpectedUriFragmentCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0> UnexpectedUriFragmentCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> SourceError
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                SourceError::UnexpectedUriFragment {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, SourceError>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<SourceError> for UnexpectedUriFragmentCtx<__T0>
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                SourceError::UnexpectedUriFragment {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
        ///SNAFU context selector for the `SourceError::PointerFailedToParse` variant
        pub struct PointerFailedToParseCtx;
        #[automatically_derived]
        impl ::core::fmt::Debug for PointerFailedToParseCtx {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "PointerFailedToParseCtx")
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for PointerFailedToParseCtx {}
        #[automatically_derived]
        impl ::core::clone::Clone for PointerFailedToParseCtx {
            #[inline]
            fn clone(&self) -> PointerFailedToParseCtx {
                *self
            }
        }
        impl ::snafu::IntoError<SourceError> for PointerFailedToParseCtx
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
        {
            type Source = MalformedPointerError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                let error: MalformedPointerError = (|v| v)(error);
                SourceError::PointerFailedToParse {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                }
            }
        }
        ///SNAFU context selector for the `SourceError::PointerFailedToResolve` variant
        pub struct PointerFailedToResolveCtx;
        #[automatically_derived]
        impl ::core::fmt::Debug for PointerFailedToResolveCtx {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "PointerFailedToResolveCtx")
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for PointerFailedToResolveCtx {}
        #[automatically_derived]
        impl ::core::clone::Clone for PointerFailedToResolveCtx {
            #[inline]
            fn clone(&self) -> PointerFailedToResolveCtx {
                *self
            }
        }
        impl ::snafu::IntoError<SourceError> for PointerFailedToResolveCtx
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
        {
            type Source = ResolvePointerError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                let error: ResolvePointerError = (|v| v)(error);
                SourceError::PointerFailedToResolve {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                }
            }
        }
        ///SNAFU context selector for the `SourceError::SchemaConflict` variant
        pub struct SchemaConflictCtx<__T0, __T1, __T2> {
            #[allow(missing_docs)]
            pub uri: __T0,
            #[allow(missing_docs)]
            pub existing_path: __T1,
            #[allow(missing_docs)]
            pub new_path: __T2,
        }
        #[automatically_derived]
        impl<
            __T0: ::core::fmt::Debug,
            __T1: ::core::fmt::Debug,
            __T2: ::core::fmt::Debug,
        > ::core::fmt::Debug for SchemaConflictCtx<__T0, __T1, __T2> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "SchemaConflictCtx",
                    "uri",
                    &self.uri,
                    "existing_path",
                    &self.existing_path,
                    "new_path",
                    &&self.new_path,
                )
            }
        }
        #[automatically_derived]
        impl<
            __T0: ::core::marker::Copy,
            __T1: ::core::marker::Copy,
            __T2: ::core::marker::Copy,
        > ::core::marker::Copy for SchemaConflictCtx<__T0, __T1, __T2> {}
        #[automatically_derived]
        impl<
            __T0: ::core::clone::Clone,
            __T1: ::core::clone::Clone,
            __T2: ::core::clone::Clone,
        > ::core::clone::Clone for SchemaConflictCtx<__T0, __T1, __T2> {
            #[inline]
            fn clone(&self) -> SchemaConflictCtx<__T0, __T1, __T2> {
                SchemaConflictCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                    existing_path: ::core::clone::Clone::clone(&self.existing_path),
                    new_path: ::core::clone::Clone::clone(&self.new_path),
                }
            }
        }
        impl<__T0, __T1, __T2> SchemaConflictCtx<__T0, __T1, __T2> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> SourceError
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
                __T1: ::core::convert::Into<Pointer>,
                __T2: ::core::convert::Into<Pointer>,
            {
                SourceError::SchemaConflict {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                    existing_path: ::core::convert::Into::into(self.existing_path),
                    new_path: ::core::convert::Into::into(self.new_path),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, SourceError>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
                __T1: ::core::convert::Into<Pointer>,
                __T2: ::core::convert::Into<Pointer>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1, __T2> ::snafu::IntoError<SourceError>
        for SchemaConflictCtx<__T0, __T1, __T2>
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
            __T1: ::core::convert::Into<Pointer>,
            __T2: ::core::convert::Into<Pointer>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                SourceError::SchemaConflict {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                    existing_path: ::core::convert::Into::into(self.existing_path),
                    new_path: ::core::convert::Into::into(self.new_path),
                }
            }
        }
        ///SNAFU context selector for the `SourceError::SourceConflict` variant
        pub struct SourceConflictCtx<__T0> {
            #[allow(missing_docs)]
            pub uri: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for SourceConflictCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SourceConflictCtx",
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy
        for SourceConflictCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone
        for SourceConflictCtx<__T0> {
            #[inline]
            fn clone(&self) -> SourceConflictCtx<__T0> {
                SourceConflictCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0> SourceConflictCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> SourceError
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                SourceError::SourceConflict {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, SourceError>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<SourceError> for SourceConflictCtx<__T0>
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                SourceError::SourceConflict {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
        ///SNAFU context selector for the `SourceError::PathNotFound` variant
        pub struct PathNotFoundCtx;
        #[automatically_derived]
        impl ::core::fmt::Debug for PathNotFoundCtx {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "PathNotFoundCtx")
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for PathNotFoundCtx {}
        #[automatically_derived]
        impl ::core::clone::Clone for PathNotFoundCtx {
            #[inline]
            fn clone(&self) -> PathNotFoundCtx {
                *self
            }
        }
        impl ::snafu::IntoError<SourceError> for PathNotFoundCtx
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
        {
            type Source = jsonptr::Error;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                let error: jsonptr::Error = (|v| v)(error);
                SourceError::PathNotFound {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                }
            }
        }
        ///SNAFU context selector for the `SourceError::SourceNotFound` variant
        pub struct SourceNotFoundCtx<__T0> {
            #[allow(missing_docs)]
            pub uri: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for SourceNotFoundCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SourceNotFoundCtx",
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy
        for SourceNotFoundCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone
        for SourceNotFoundCtx<__T0> {
            #[inline]
            fn clone(&self) -> SourceNotFoundCtx<__T0> {
                SourceNotFoundCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0> SourceNotFoundCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> SourceError
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                SourceError::SourceNotFound {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, SourceError>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<SourceError> for SourceNotFoundCtx<__T0>
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                SourceError::SourceNotFound {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
        ///SNAFU context selector for the `SourceError::UnknownAnchor` variant
        pub struct UnknownAnchorCtx<__T0, __T1> {
            #[allow(missing_docs)]
            pub anchor: __T0,
            #[allow(missing_docs)]
            pub uri: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
        for UnknownAnchorCtx<__T0, __T1> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UnknownAnchorCtx",
                    "anchor",
                    &self.anchor,
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
        for UnknownAnchorCtx<__T0, __T1> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
        for UnknownAnchorCtx<__T0, __T1> {
            #[inline]
            fn clone(&self) -> UnknownAnchorCtx<__T0, __T1> {
                UnknownAnchorCtx {
                    anchor: ::core::clone::Clone::clone(&self.anchor),
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0, __T1> UnknownAnchorCtx<__T0, __T1> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> SourceError
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<AbsoluteUri>,
            {
                SourceError::UnknownAnchor {
                    anchor: ::core::convert::Into::into(self.anchor),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, SourceError>
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<AbsoluteUri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1> ::snafu::IntoError<SourceError> for UnknownAnchorCtx<__T0, __T1>
        where
            SourceError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<String>,
            __T1: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> SourceError {
                SourceError::UnknownAnchor {
                    anchor: ::core::convert::Into::into(self.anchor),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for SourceError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                SourceError::DeserializationFailed {
                    ref backtrace,
                    ref source,
                    ref uri,
                } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "failed to deserialize source \"{1}\":\n\t{0}", source, uri
                            ),
                        )
                }
                SourceError::ResolutionFailed { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                SourceError::InvalidUtf8 { ref backtrace, ref source, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!("source is not valid UTF-8: {0}", source),
                        )
                }
                SourceError::UriFailedToParse { ref source } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!("failed to parse source URI: {0}", source),
                        )
                }
                SourceError::UnexpectedUriFragment { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "source URIs may not contain fragments, found \"{0}\"", uri
                            ),
                        )
                }
                SourceError::PointerFailedToParse { ref backtrace, ref source } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!("failed to parse json pointer: {0}", source),
                        )
                }
                SourceError::PointerFailedToResolve { ref backtrace, ref source } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!("failed to resolve json pointer: {0}", source),
                        )
                }
                SourceError::SchemaConflict {
                    ref backtrace,
                    ref existing_path,
                    ref new_path,
                    ref uri,
                } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "source address {0:?} @ {1:?} already assigned to {2:?}",
                                uri, new_path, existing_path
                            ),
                        )
                }
                SourceError::SourceConflict { ref backtrace, ref uri } => {
                    __snafu_display_formatter.write_fmt(format_args!("SourceConflict"))
                }
                SourceError::PathNotFound { ref backtrace, ref source } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!("failed to resolve link path: {0}", source),
                        )
                }
                SourceError::SourceNotFound { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("source not found: \"{0}\"", uri))
                }
                SourceError::UnknownAnchor { ref anchor, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "unknown anchor: \"{0}\" in URI \"{1}\"", anchor, uri
                            ),
                        )
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for SourceError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                SourceError::DeserializationFailed { .. } => {
                    "SourceError :: DeserializationFailed"
                }
                SourceError::ResolutionFailed { .. } => "SourceError :: ResolutionFailed",
                SourceError::InvalidUtf8 { .. } => "SourceError :: InvalidUtf8",
                SourceError::UriFailedToParse { .. } => "SourceError :: UriFailedToParse",
                SourceError::UnexpectedUriFragment { .. } => {
                    "SourceError :: UnexpectedUriFragment"
                }
                SourceError::PointerFailedToParse { .. } => {
                    "SourceError :: PointerFailedToParse"
                }
                SourceError::PointerFailedToResolve { .. } => {
                    "SourceError :: PointerFailedToResolve"
                }
                SourceError::SchemaConflict { .. } => "SourceError :: SchemaConflict",
                SourceError::SourceConflict { .. } => "SourceError :: SourceConflict",
                SourceError::PathNotFound { .. } => "SourceError :: PathNotFound",
                SourceError::SourceNotFound { .. } => "SourceError :: SourceNotFound",
                SourceError::UnknownAnchor { .. } => "SourceError :: UnknownAnchor",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                SourceError::DeserializationFailed { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::ResolutionFailed { ref source, .. } => {
                    source.as_error_source().source()
                }
                SourceError::InvalidUtf8 { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::UriFailedToParse { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::UnexpectedUriFragment { .. } => ::core::option::Option::None,
                SourceError::PointerFailedToParse { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::PointerFailedToResolve { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::SchemaConflict { .. } => ::core::option::Option::None,
                SourceError::SourceConflict { .. } => ::core::option::Option::None,
                SourceError::PathNotFound { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::SourceNotFound { .. } => ::core::option::Option::None,
                SourceError::UnknownAnchor { .. } => ::core::option::Option::None,
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                SourceError::DeserializationFailed { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::ResolutionFailed { ref source, .. } => {
                    source.as_error_source().source()
                }
                SourceError::InvalidUtf8 { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::UriFailedToParse { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::UnexpectedUriFragment { .. } => ::core::option::Option::None,
                SourceError::PointerFailedToParse { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::PointerFailedToResolve { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::SchemaConflict { .. } => ::core::option::Option::None,
                SourceError::SourceConflict { .. } => ::core::option::Option::None,
                SourceError::PathNotFound { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SourceError::SourceNotFound { .. } => ::core::option::Option::None,
                SourceError::UnknownAnchor { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for SourceError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                SourceError::DeserializationFailed { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SourceError::ResolutionFailed { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
                SourceError::InvalidUtf8 { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SourceError::UriFailedToParse { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
                SourceError::UnexpectedUriFragment { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SourceError::PointerFailedToParse { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SourceError::PointerFailedToResolve { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SourceError::SchemaConflict { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SourceError::SourceConflict { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SourceError::PathNotFound { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SourceError::SourceNotFound { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SourceError::UnknownAnchor { .. } => ::core::option::Option::None,
            }
        }
    }
    impl From<jsonptr::MalformedPointerError> for SourceError {
        fn from(err: jsonptr::MalformedPointerError) -> Self {
            Self::PointerFailedToParseOrResolve(err.into())
        }
    }
    impl From<ResolveError> for SourceError {
        fn from(value: ResolveError) -> Self {
            Self::ResolutionFailed(ResolveErrors {
                sources: <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([value])),
                backtrace: Backtrace::capture(),
            })
        }
    }
    /// An error occurred parsing or resolving a JSON [`Pointer`].
    pub enum PointerError {
        #[snafu(transparent)]
        /// The JSON [`Pointer`] was malformed.
        ParsingFailed { source: MalformedPointerError, backtrace: Backtrace },
        #[snafu(transparent)]
        /// The JSON [`Pointer`] could not be resolved.
        ResolutionFailed { source: ResolvePointerError, backtrace: Backtrace },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PointerError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PointerError::ParsingFailed { source: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ParsingFailed",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                PointerError::ResolutionFailed {
                    source: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "ResolutionFailed",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
            }
        }
    }
    impl ::core::convert::From<MalformedPointerError> for PointerError {
        #[track_caller]
        fn from(error: MalformedPointerError) -> Self {
            let error: MalformedPointerError = (|v| v)(error);
            PointerError::ParsingFailed {
                backtrace: {
                    use ::snafu::AsErrorSource;
                    let error = error.as_error_source();
                    ::snafu::GenerateImplicitData::generate_with_source(error)
                },
                source: error,
            }
        }
    }
    impl ::core::convert::From<ResolvePointerError> for PointerError {
        #[track_caller]
        fn from(error: ResolvePointerError) -> Self {
            let error: ResolvePointerError = (|v| v)(error);
            PointerError::ResolutionFailed {
                backtrace: {
                    use ::snafu::AsErrorSource;
                    let error = error.as_error_source();
                    ::snafu::GenerateImplicitData::generate_with_source(error)
                },
                source: error,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for PointerError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                PointerError::ParsingFailed { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                PointerError::ResolutionFailed { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for PointerError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                PointerError::ParsingFailed { .. } => "PointerError :: ParsingFailed",
                PointerError::ResolutionFailed { .. } => {
                    "PointerError :: ResolutionFailed"
                }
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                PointerError::ParsingFailed { ref source, .. } => {
                    source.as_error_source().source()
                }
                PointerError::ResolutionFailed { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                PointerError::ParsingFailed { ref source, .. } => {
                    source.as_error_source().source()
                }
                PointerError::ResolutionFailed { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for PointerError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                PointerError::ParsingFailed { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                PointerError::ResolutionFailed { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
    /// Possible errors that may occur while creating a
    /// [`Dialects`](crate::dialect::Dialects)
    pub enum DialectsError {
        /// No dialects were provided.
        #[snafu(display("no dialects were provided"))]
        Empty { backtrace: Backtrace },
        /// An error occurred creating a [`Dialect`].
        #[snafu(transparent)]
        Dialect { #[snafu(backtrace)] source: DialectError },
        /// Multiple [`Dialect`]s with the same [`AbsoluteUri`] id were provided.
        #[snafu(display("duplicate dialect id provided: {uri}"))]
        Duplicate { uri: AbsoluteUri, backtrace: Backtrace },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DialectsError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DialectsError::Empty { backtrace: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Empty",
                        "backtrace",
                        &__self_0,
                    )
                }
                DialectsError::Dialect { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Dialect",
                        "source",
                        &__self_0,
                    )
                }
                DialectsError::Duplicate { uri: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Duplicate",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
            }
        }
    }
    ///SNAFU context selector for the `DialectsError::Empty` variant
    struct EmptySnafu;
    #[automatically_derived]
    impl ::core::fmt::Debug for EmptySnafu {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "EmptySnafu")
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EmptySnafu {}
    #[automatically_derived]
    impl ::core::clone::Clone for EmptySnafu {
        #[inline]
        fn clone(&self) -> EmptySnafu {
            *self
        }
    }
    impl EmptySnafu {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> DialectsError {
            DialectsError::Empty {
                backtrace: ::snafu::GenerateImplicitData::generate(),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, DialectsError> {
            ::core::result::Result::Err(self.build())
        }
    }
    impl ::snafu::IntoError<DialectsError> for EmptySnafu
    where
        DialectsError: ::snafu::Error + ::snafu::ErrorCompat,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> DialectsError {
            DialectsError::Empty {
                backtrace: ::snafu::GenerateImplicitData::generate(),
            }
        }
    }
    impl ::core::convert::From<DialectError> for DialectsError {
        #[track_caller]
        fn from(error: DialectError) -> Self {
            let error: DialectError = (|v| v)(error);
            DialectsError::Dialect {
                source: error,
            }
        }
    }
    ///SNAFU context selector for the `DialectsError::Duplicate` variant
    struct DuplicateSnafu<__T0> {
        #[allow(missing_docs)]
        uri: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for DuplicateSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "DuplicateSnafu",
                "uri",
                &&self.uri,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for DuplicateSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for DuplicateSnafu<__T0> {
        #[inline]
        fn clone(&self) -> DuplicateSnafu<__T0> {
            DuplicateSnafu {
                uri: ::core::clone::Clone::clone(&self.uri),
            }
        }
    }
    impl<__T0> DuplicateSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> DialectsError
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            DialectsError::Duplicate {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, DialectsError>
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<DialectsError> for DuplicateSnafu<__T0>
    where
        DialectsError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<AbsoluteUri>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> DialectsError {
            DialectsError::Duplicate {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for DialectsError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                DialectsError::Empty { ref backtrace } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("no dialects were provided"))
                }
                DialectsError::Dialect { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                DialectsError::Duplicate { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!("duplicate dialect id provided: {0}", uri),
                        )
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for DialectsError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                DialectsError::Empty { .. } => "DialectsError :: Empty",
                DialectsError::Dialect { .. } => "DialectsError :: Dialect",
                DialectsError::Duplicate { .. } => "DialectsError :: Duplicate",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                DialectsError::Empty { .. } => ::core::option::Option::None,
                DialectsError::Dialect { ref source, .. } => {
                    source.as_error_source().source()
                }
                DialectsError::Duplicate { .. } => ::core::option::Option::None,
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                DialectsError::Empty { .. } => ::core::option::Option::None,
                DialectsError::Dialect { ref source, .. } => {
                    source.as_error_source().source()
                }
                DialectsError::Duplicate { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for DialectsError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                DialectsError::Empty { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                DialectsError::Dialect { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
                DialectsError::Duplicate { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
    /// Possible errors that may occur while creating a
    /// [`Dialect`](crate::dialect::Dialect)
    pub enum DialectError {
        /// The default [`Dialect`] was not found.
        #[snafu(display("default dialect not found: {uri}"))]
        DefaultNotFound { uri: AbsoluteUri, backtrace: Backtrace },
        /// A [`Dialect`] ID contained a non-empty fragment.
        #[snafu(display("dialect ids may not contain fragments; found: \"{uri}\""))]
        FragmentedId { uri: AbsoluteUri, backtrace: Backtrace },
        /// `Dialect` was constructed but a metaschema with the `Dialect`'s `id` was
        /// not present.
        #[snafu(
            display(
                "primary metaschema with id \"{uri}\" not found within the supplied metaschemas"
            )
        )]
        PrimaryMetaschemaNotFound { uri: AbsoluteUri, backtrace: Backtrace },
        /// Exactly one [`Keyword`](crate::keyword::Keyword) must implement
        /// implement [`is_pertinent_to`](`crate::keyword::Keyword::is_pertinent_to`) but none were provided.
        #[snafu(
            display(
                "exactly one `Keyword` must implemenet the `is_pertinent_to` method; none were found"
            )
        )]
        IsPertinentToNotImplemented { uri: AbsoluteUri, backtrace: Backtrace },
        /// Exactly one [`Keyword`](crate::keyword::Keyword) must implement
        /// implement [`dialect`](`crate::keyword::Keyword::dialect`) but none were provided.
        #[snafu(
            display(
                "at least one `Keyword` must implement the `dialect` method; none were found"
            )
        )]
        DialectNotImplemented { uri: AbsoluteUri, backtrace: Backtrace },
        /// At least one [`Keyword`](crate::keyword::Keyword) must implement
        /// implement [`identify`](`crate::keyword::Keyword::identify`) but none were provided.
        #[snafu(
            display(
                "at least one `Keyword` must implement the `identify` method; none were found"
            )
        )]
        IdentifyNotImplemented { uri: AbsoluteUri, backtrace: Backtrace },
        /// An [`AbsoluteUri`] failed to parse.
        #[snafu(transparent)]
        UriPFailedToParse { #[snafu(backtrace)] source: Error },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DialectError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DialectError::DefaultNotFound { uri: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "DefaultNotFound",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                DialectError::FragmentedId { uri: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "FragmentedId",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                DialectError::PrimaryMetaschemaNotFound {
                    uri: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "PrimaryMetaschemaNotFound",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                DialectError::IsPertinentToNotImplemented {
                    uri: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "IsPertinentToNotImplemented",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                DialectError::DialectNotImplemented {
                    uri: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "DialectNotImplemented",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                DialectError::IdentifyNotImplemented {
                    uri: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "IdentifyNotImplemented",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                DialectError::UriPFailedToParse { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UriPFailedToParse",
                        "source",
                        &__self_0,
                    )
                }
            }
        }
    }
    ///SNAFU context selector for the `DialectError::DefaultNotFound` variant
    struct DefaultNotFoundSnafu<__T0> {
        #[allow(missing_docs)]
        uri: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for DefaultNotFoundSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "DefaultNotFoundSnafu",
                "uri",
                &&self.uri,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy
    for DefaultNotFoundSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone
    for DefaultNotFoundSnafu<__T0> {
        #[inline]
        fn clone(&self) -> DefaultNotFoundSnafu<__T0> {
            DefaultNotFoundSnafu {
                uri: ::core::clone::Clone::clone(&self.uri),
            }
        }
    }
    impl<__T0> DefaultNotFoundSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> DialectError
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            DialectError::DefaultNotFound {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, DialectError>
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<DialectError> for DefaultNotFoundSnafu<__T0>
    where
        DialectError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<AbsoluteUri>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> DialectError {
            DialectError::DefaultNotFound {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
    }
    ///SNAFU context selector for the `DialectError::FragmentedId` variant
    struct FragmentedIdSnafu<__T0> {
        #[allow(missing_docs)]
        uri: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for FragmentedIdSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "FragmentedIdSnafu",
                "uri",
                &&self.uri,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for FragmentedIdSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for FragmentedIdSnafu<__T0> {
        #[inline]
        fn clone(&self) -> FragmentedIdSnafu<__T0> {
            FragmentedIdSnafu {
                uri: ::core::clone::Clone::clone(&self.uri),
            }
        }
    }
    impl<__T0> FragmentedIdSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> DialectError
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            DialectError::FragmentedId {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, DialectError>
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<DialectError> for FragmentedIdSnafu<__T0>
    where
        DialectError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<AbsoluteUri>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> DialectError {
            DialectError::FragmentedId {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
    }
    ///SNAFU context selector for the `DialectError::PrimaryMetaschemaNotFound` variant
    struct PrimaryMetaschemaNotFoundSnafu<__T0> {
        #[allow(missing_docs)]
        uri: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug
    for PrimaryMetaschemaNotFoundSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "PrimaryMetaschemaNotFoundSnafu",
                "uri",
                &&self.uri,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy
    for PrimaryMetaschemaNotFoundSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone
    for PrimaryMetaschemaNotFoundSnafu<__T0> {
        #[inline]
        fn clone(&self) -> PrimaryMetaschemaNotFoundSnafu<__T0> {
            PrimaryMetaschemaNotFoundSnafu {
                uri: ::core::clone::Clone::clone(&self.uri),
            }
        }
    }
    impl<__T0> PrimaryMetaschemaNotFoundSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> DialectError
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            DialectError::PrimaryMetaschemaNotFound {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, DialectError>
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<DialectError> for PrimaryMetaschemaNotFoundSnafu<__T0>
    where
        DialectError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<AbsoluteUri>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> DialectError {
            DialectError::PrimaryMetaschemaNotFound {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
    }
    ///SNAFU context selector for the `DialectError::IsPertinentToNotImplemented` variant
    struct IsPertinentToNotImplementedSnafu<__T0> {
        #[allow(missing_docs)]
        uri: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug
    for IsPertinentToNotImplementedSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "IsPertinentToNotImplementedSnafu",
                "uri",
                &&self.uri,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy
    for IsPertinentToNotImplementedSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone
    for IsPertinentToNotImplementedSnafu<__T0> {
        #[inline]
        fn clone(&self) -> IsPertinentToNotImplementedSnafu<__T0> {
            IsPertinentToNotImplementedSnafu {
                uri: ::core::clone::Clone::clone(&self.uri),
            }
        }
    }
    impl<__T0> IsPertinentToNotImplementedSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> DialectError
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            DialectError::IsPertinentToNotImplemented {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, DialectError>
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<DialectError>
    for IsPertinentToNotImplementedSnafu<__T0>
    where
        DialectError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<AbsoluteUri>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> DialectError {
            DialectError::IsPertinentToNotImplemented {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
    }
    ///SNAFU context selector for the `DialectError::DialectNotImplemented` variant
    struct DialectNotImplementedSnafu<__T0> {
        #[allow(missing_docs)]
        uri: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug
    for DialectNotImplementedSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "DialectNotImplementedSnafu",
                "uri",
                &&self.uri,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy
    for DialectNotImplementedSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone
    for DialectNotImplementedSnafu<__T0> {
        #[inline]
        fn clone(&self) -> DialectNotImplementedSnafu<__T0> {
            DialectNotImplementedSnafu {
                uri: ::core::clone::Clone::clone(&self.uri),
            }
        }
    }
    impl<__T0> DialectNotImplementedSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> DialectError
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            DialectError::DialectNotImplemented {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, DialectError>
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<DialectError> for DialectNotImplementedSnafu<__T0>
    where
        DialectError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<AbsoluteUri>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> DialectError {
            DialectError::DialectNotImplemented {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
    }
    ///SNAFU context selector for the `DialectError::IdentifyNotImplemented` variant
    struct IdentifyNotImplementedSnafu<__T0> {
        #[allow(missing_docs)]
        uri: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug
    for IdentifyNotImplementedSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "IdentifyNotImplementedSnafu",
                "uri",
                &&self.uri,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy
    for IdentifyNotImplementedSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone
    for IdentifyNotImplementedSnafu<__T0> {
        #[inline]
        fn clone(&self) -> IdentifyNotImplementedSnafu<__T0> {
            IdentifyNotImplementedSnafu {
                uri: ::core::clone::Clone::clone(&self.uri),
            }
        }
    }
    impl<__T0> IdentifyNotImplementedSnafu<__T0> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> DialectError
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            DialectError::IdentifyNotImplemented {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, DialectError>
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0> ::snafu::IntoError<DialectError> for IdentifyNotImplementedSnafu<__T0>
    where
        DialectError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<AbsoluteUri>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> DialectError {
            DialectError::IdentifyNotImplemented {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                uri: ::core::convert::Into::into(self.uri),
            }
        }
    }
    impl ::core::convert::From<Error> for DialectError {
        #[track_caller]
        fn from(error: Error) -> Self {
            let error: Error = (|v| v)(error);
            DialectError::UriPFailedToParse {
                source: error,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for DialectError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                DialectError::DefaultNotFound { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("default dialect not found: {0}", uri))
                }
                DialectError::FragmentedId { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "dialect ids may not contain fragments; found: \"{0}\"", uri
                            ),
                        )
                }
                DialectError::PrimaryMetaschemaNotFound { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "primary metaschema with id \"{0}\" not found within the supplied metaschemas",
                                uri
                            ),
                        )
                }
                DialectError::IsPertinentToNotImplemented { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "exactly one `Keyword` must implemenet the `is_pertinent_to` method; none were found"
                            ),
                        )
                }
                DialectError::DialectNotImplemented { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "at least one `Keyword` must implement the `dialect` method; none were found"
                            ),
                        )
                }
                DialectError::IdentifyNotImplemented { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "at least one `Keyword` must implement the `identify` method; none were found"
                            ),
                        )
                }
                DialectError::UriPFailedToParse { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for DialectError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                DialectError::DefaultNotFound { .. } => "DialectError :: DefaultNotFound",
                DialectError::FragmentedId { .. } => "DialectError :: FragmentedId",
                DialectError::PrimaryMetaschemaNotFound { .. } => {
                    "DialectError :: PrimaryMetaschemaNotFound"
                }
                DialectError::IsPertinentToNotImplemented { .. } => {
                    "DialectError :: IsPertinentToNotImplemented"
                }
                DialectError::DialectNotImplemented { .. } => {
                    "DialectError :: DialectNotImplemented"
                }
                DialectError::IdentifyNotImplemented { .. } => {
                    "DialectError :: IdentifyNotImplemented"
                }
                DialectError::UriPFailedToParse { .. } => {
                    "DialectError :: UriPFailedToParse"
                }
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                DialectError::DefaultNotFound { .. } => ::core::option::Option::None,
                DialectError::FragmentedId { .. } => ::core::option::Option::None,
                DialectError::PrimaryMetaschemaNotFound { .. } => {
                    ::core::option::Option::None
                }
                DialectError::IsPertinentToNotImplemented { .. } => {
                    ::core::option::Option::None
                }
                DialectError::DialectNotImplemented { .. } => {
                    ::core::option::Option::None
                }
                DialectError::IdentifyNotImplemented { .. } => {
                    ::core::option::Option::None
                }
                DialectError::UriPFailedToParse { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                DialectError::DefaultNotFound { .. } => ::core::option::Option::None,
                DialectError::FragmentedId { .. } => ::core::option::Option::None,
                DialectError::PrimaryMetaschemaNotFound { .. } => {
                    ::core::option::Option::None
                }
                DialectError::IsPertinentToNotImplemented { .. } => {
                    ::core::option::Option::None
                }
                DialectError::DialectNotImplemented { .. } => {
                    ::core::option::Option::None
                }
                DialectError::IdentifyNotImplemented { .. } => {
                    ::core::option::Option::None
                }
                DialectError::UriPFailedToParse { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for DialectError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                DialectError::DefaultNotFound { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                DialectError::FragmentedId { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                DialectError::PrimaryMetaschemaNotFound { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                DialectError::IsPertinentToNotImplemented { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                DialectError::DialectNotImplemented { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                DialectError::IdentifyNotImplemented { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                DialectError::UriPFailedToParse { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
            }
        }
    }
    /// Failed to associate a schema to a location within a source.
    #[snafu(visibility(pub), context(suffix(Ctx)), module)]
    pub enum LinkError {
        /// A conflict occurred (i.e. a source was linked from multiple locations).
        #[snafu(
            display(
                "source address {:?} @ {:?} already assigned to {:?}",
                uri,
                new_path,
                existing_path
            )
        )]
        SourceConflict {
            uri: AbsoluteUri,
            /// The existing schema location.
            existing_path: Pointer,
            /// The new schema location.
            new_path: Pointer,
        },
        /// Failed to resolve a path
        #[snafu(display("failed to resolve link path: {source}"))]
        PathNotFound { source: jsonptr::Error, backtrace: Backtrace },
        /// Failed to resolve a URI
        #[snafu(display("source not found: \"{uri}\""))]
        SourceNotFound { uri: AbsoluteUri, backtrace: Backtrace },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LinkError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                LinkError::SourceConflict {
                    uri: __self_0,
                    existing_path: __self_1,
                    new_path: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "SourceConflict",
                        "uri",
                        __self_0,
                        "existing_path",
                        __self_1,
                        "new_path",
                        &__self_2,
                    )
                }
                LinkError::PathNotFound { source: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "PathNotFound",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                LinkError::SourceNotFound { uri: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "SourceNotFound",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
            }
        }
    }
    pub mod link_error {
        use super::*;
        ///SNAFU context selector for the `LinkError::SourceConflict` variant
        pub struct SourceConflictCtx<__T0, __T1, __T2> {
            #[allow(missing_docs)]
            pub uri: __T0,
            #[allow(missing_docs)]
            pub existing_path: __T1,
            #[allow(missing_docs)]
            pub new_path: __T2,
        }
        #[automatically_derived]
        impl<
            __T0: ::core::fmt::Debug,
            __T1: ::core::fmt::Debug,
            __T2: ::core::fmt::Debug,
        > ::core::fmt::Debug for SourceConflictCtx<__T0, __T1, __T2> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "SourceConflictCtx",
                    "uri",
                    &self.uri,
                    "existing_path",
                    &self.existing_path,
                    "new_path",
                    &&self.new_path,
                )
            }
        }
        #[automatically_derived]
        impl<
            __T0: ::core::marker::Copy,
            __T1: ::core::marker::Copy,
            __T2: ::core::marker::Copy,
        > ::core::marker::Copy for SourceConflictCtx<__T0, __T1, __T2> {}
        #[automatically_derived]
        impl<
            __T0: ::core::clone::Clone,
            __T1: ::core::clone::Clone,
            __T2: ::core::clone::Clone,
        > ::core::clone::Clone for SourceConflictCtx<__T0, __T1, __T2> {
            #[inline]
            fn clone(&self) -> SourceConflictCtx<__T0, __T1, __T2> {
                SourceConflictCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                    existing_path: ::core::clone::Clone::clone(&self.existing_path),
                    new_path: ::core::clone::Clone::clone(&self.new_path),
                }
            }
        }
        impl<__T0, __T1, __T2> SourceConflictCtx<__T0, __T1, __T2> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> LinkError
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
                __T1: ::core::convert::Into<Pointer>,
                __T2: ::core::convert::Into<Pointer>,
            {
                LinkError::SourceConflict {
                    uri: ::core::convert::Into::into(self.uri),
                    existing_path: ::core::convert::Into::into(self.existing_path),
                    new_path: ::core::convert::Into::into(self.new_path),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, LinkError>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
                __T1: ::core::convert::Into<Pointer>,
                __T2: ::core::convert::Into<Pointer>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1, __T2> ::snafu::IntoError<LinkError>
        for SourceConflictCtx<__T0, __T1, __T2>
        where
            LinkError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
            __T1: ::core::convert::Into<Pointer>,
            __T2: ::core::convert::Into<Pointer>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> LinkError {
                LinkError::SourceConflict {
                    uri: ::core::convert::Into::into(self.uri),
                    existing_path: ::core::convert::Into::into(self.existing_path),
                    new_path: ::core::convert::Into::into(self.new_path),
                }
            }
        }
        ///SNAFU context selector for the `LinkError::PathNotFound` variant
        pub struct PathNotFoundCtx;
        #[automatically_derived]
        impl ::core::fmt::Debug for PathNotFoundCtx {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "PathNotFoundCtx")
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for PathNotFoundCtx {}
        #[automatically_derived]
        impl ::core::clone::Clone for PathNotFoundCtx {
            #[inline]
            fn clone(&self) -> PathNotFoundCtx {
                *self
            }
        }
        impl ::snafu::IntoError<LinkError> for PathNotFoundCtx
        where
            LinkError: ::snafu::Error + ::snafu::ErrorCompat,
        {
            type Source = jsonptr::Error;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> LinkError {
                let error: jsonptr::Error = (|v| v)(error);
                LinkError::PathNotFound {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                }
            }
        }
        ///SNAFU context selector for the `LinkError::SourceNotFound` variant
        pub struct SourceNotFoundCtx<__T0> {
            #[allow(missing_docs)]
            pub uri: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for SourceNotFoundCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SourceNotFoundCtx",
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy
        for SourceNotFoundCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone
        for SourceNotFoundCtx<__T0> {
            #[inline]
            fn clone(&self) -> SourceNotFoundCtx<__T0> {
                SourceNotFoundCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0> SourceNotFoundCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> LinkError
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                LinkError::SourceNotFound {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, LinkError>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<LinkError> for SourceNotFoundCtx<__T0>
        where
            LinkError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> LinkError {
                LinkError::SourceNotFound {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
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
                LinkError::SourceConflict {
                    ref existing_path,
                    ref new_path,
                    ref uri,
                } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "source address {0:?} @ {1:?} already assigned to {2:?}",
                                uri, new_path, existing_path
                            ),
                        )
                }
                LinkError::PathNotFound { ref backtrace, ref source } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!("failed to resolve link path: {0}", source),
                        )
                }
                LinkError::SourceNotFound { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("source not found: \"{0}\"", uri))
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for LinkError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                LinkError::SourceConflict { .. } => "LinkError :: SourceConflict",
                LinkError::PathNotFound { .. } => "LinkError :: PathNotFound",
                LinkError::SourceNotFound { .. } => "LinkError :: SourceNotFound",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                LinkError::SourceConflict { .. } => ::core::option::Option::None,
                LinkError::PathNotFound { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                LinkError::SourceNotFound { .. } => ::core::option::Option::None,
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                LinkError::SourceConflict { .. } => ::core::option::Option::None,
                LinkError::PathNotFound { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                LinkError::SourceNotFound { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for LinkError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                LinkError::SourceConflict { .. } => ::core::option::Option::None,
                LinkError::PathNotFound { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                LinkError::SourceNotFound { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
    /// Various errors that can occur while building an [`Interrogator`](crate::Interrogator).
    #[snafu(visibility(pub), context(suffix(Ctx)), module)]
    pub enum BuildError<C> {
        #[snafu(transparent)]
        /// A [`Schema`](crate::schema::Schema) failed to compile.
        FailedToCompile { #[snafu(backtrace)] source: C },
        #[snafu(transparent)]
        /// An issue with [`Dialect`]s occurred.
        FailedToCreateDialects { #[snafu(backtrace)] source: DialectsError },
        #[snafu(transparent)]
        /// An error occurred while adding, resolving, or deserializing a
        /// [`Source`](crate::source::Source).
        FailedToSource { #[snafu(backtrace)] source: SourceError },
        /// Failed to parse a number
        #[snafu(transparent)]
        FailedToParseNumber { #[snafu(backtrace)] source: NumberError },
    }
    #[automatically_derived]
    impl<C: ::core::fmt::Debug> ::core::fmt::Debug for BuildError<C> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                BuildError::FailedToCompile { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "FailedToCompile",
                        "source",
                        &__self_0,
                    )
                }
                BuildError::FailedToCreateDialects { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "FailedToCreateDialects",
                        "source",
                        &__self_0,
                    )
                }
                BuildError::FailedToSource { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "FailedToSource",
                        "source",
                        &__self_0,
                    )
                }
                BuildError::FailedToParseNumber { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "FailedToParseNumber",
                        "source",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub mod build_error {
        use super::*;
        impl<C> ::core::convert::From<C> for BuildError<C> {
            #[track_caller]
            fn from(error: C) -> Self {
                let error: C = (|v| v)(error);
                BuildError::FailedToCompile {
                    source: error,
                }
            }
        }
        impl<C> ::core::convert::From<DialectsError> for BuildError<C> {
            #[track_caller]
            fn from(error: DialectsError) -> Self {
                let error: DialectsError = (|v| v)(error);
                BuildError::FailedToCreateDialects {
                    source: error,
                }
            }
        }
        impl<C> ::core::convert::From<SourceError> for BuildError<C> {
            #[track_caller]
            fn from(error: SourceError) -> Self {
                let error: SourceError = (|v| v)(error);
                BuildError::FailedToSource {
                    source: error,
                }
            }
        }
        impl<C> ::core::convert::From<NumberError> for BuildError<C> {
            #[track_caller]
            fn from(error: NumberError) -> Self {
                let error: NumberError = (|v| v)(error);
                BuildError::FailedToParseNumber {
                    source: error,
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl<C> ::core::fmt::Display for BuildError<C> {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                BuildError::FailedToCompile { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                BuildError::FailedToCreateDialects { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                BuildError::FailedToSource { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                BuildError::FailedToParseNumber { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl<C> ::snafu::Error for BuildError<C>
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                BuildError::FailedToCompile { .. } => "BuildError :: FailedToCompile",
                BuildError::FailedToCreateDialects { .. } => {
                    "BuildError :: FailedToCreateDialects"
                }
                BuildError::FailedToSource { .. } => "BuildError :: FailedToSource",
                BuildError::FailedToParseNumber { .. } => {
                    "BuildError :: FailedToParseNumber"
                }
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                BuildError::FailedToCompile { ref source, .. } => {
                    source.as_error_source().source()
                }
                BuildError::FailedToCreateDialects { ref source, .. } => {
                    source.as_error_source().source()
                }
                BuildError::FailedToSource { ref source, .. } => {
                    source.as_error_source().source()
                }
                BuildError::FailedToParseNumber { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                BuildError::FailedToCompile { ref source, .. } => {
                    source.as_error_source().source()
                }
                BuildError::FailedToCreateDialects { ref source, .. } => {
                    source.as_error_source().source()
                }
                BuildError::FailedToSource { ref source, .. } => {
                    source.as_error_source().source()
                }
                BuildError::FailedToParseNumber { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl<C> ::snafu::ErrorCompat for BuildError<C> {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                BuildError::FailedToCompile { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
                BuildError::FailedToCreateDialects { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
                BuildError::FailedToSource { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
                BuildError::FailedToParseNumber { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
            }
        }
    }
    /// An error occurred while parsing a [`Number`] as a [`num::BigRational`].
    #[snafu(visibility(pub), context(suffix(Ctx)), module)]
    pub enum NumberError {
        /// Failed to parse exponent of a number.
        #[snafu(display("failed to parse exponent of number \"{value}\":\n\t{source}"))]
        FailedToParseExponent {
            /// the value of the string being parsed
            value: String,
            /// the underlying error
            source: ParseIntError,
            backtrace: Backtrace,
        },
        /// Unexpected character found in a number.
        #[snafu(
            display(
                "failed to parse number \"{value}\":\n\tunexpected character: '{character}' at index {index}"
            )
        )]
        UnexpectedChar {
            /// the value of the string being parsed
            value: String,
            /// the character which caused the error
            character: char,
            /// the index of the character which caused the error
            index: usize,
            backtrace: Backtrace,
        },
        /// The number is not an integer.
        #[snafu(display("failed to parse number \"{value}\":\n\tnot an integer"))]
        NotAnInteger {
            /// value of string being parsed
            value: String,
            backtrace: Backtrace,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NumberError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                NumberError::FailedToParseExponent {
                    value: __self_0,
                    source: __self_1,
                    backtrace: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "FailedToParseExponent",
                        "value",
                        __self_0,
                        "source",
                        __self_1,
                        "backtrace",
                        &__self_2,
                    )
                }
                NumberError::UnexpectedChar {
                    value: __self_0,
                    character: __self_1,
                    index: __self_2,
                    backtrace: __self_3,
                } => {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
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
                    )
                }
                NumberError::NotAnInteger { value: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "NotAnInteger",
                        "value",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for NumberError {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for NumberError {
        #[inline]
        fn eq(&self, other: &NumberError) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        NumberError::FailedToParseExponent {
                            value: __self_0,
                            source: __self_1,
                            backtrace: __self_2,
                        },
                        NumberError::FailedToParseExponent {
                            value: __arg1_0,
                            source: __arg1_1,
                            backtrace: __arg1_2,
                        },
                    ) => {
                        *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                            && *__self_2 == *__arg1_2
                    }
                    (
                        NumberError::UnexpectedChar {
                            value: __self_0,
                            character: __self_1,
                            index: __self_2,
                            backtrace: __self_3,
                        },
                        NumberError::UnexpectedChar {
                            value: __arg1_0,
                            character: __arg1_1,
                            index: __arg1_2,
                            backtrace: __arg1_3,
                        },
                    ) => {
                        *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                            && *__self_2 == *__arg1_2 && *__self_3 == *__arg1_3
                    }
                    (
                        NumberError::NotAnInteger {
                            value: __self_0,
                            backtrace: __self_1,
                        },
                        NumberError::NotAnInteger {
                            value: __arg1_0,
                            backtrace: __arg1_1,
                        },
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for NumberError {}
    #[automatically_derived]
    impl ::core::cmp::Eq for NumberError {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<ParseIntError>;
            let _: ::core::cmp::AssertParamIsEq<Backtrace>;
            let _: ::core::cmp::AssertParamIsEq<char>;
            let _: ::core::cmp::AssertParamIsEq<usize>;
        }
    }
    pub mod number_error {
        use super::*;
        ///SNAFU context selector for the `NumberError::FailedToParseExponent` variant
        pub struct FailedToParseExponentCtx<__T0> {
            #[allow(missing_docs)]
            pub value: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug
        for FailedToParseExponentCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "FailedToParseExponentCtx",
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy
        for FailedToParseExponentCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone
        for FailedToParseExponentCtx<__T0> {
            #[inline]
            fn clone(&self) -> FailedToParseExponentCtx<__T0> {
                FailedToParseExponentCtx {
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        impl<__T0> ::snafu::IntoError<NumberError> for FailedToParseExponentCtx<__T0>
        where
            NumberError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<String>,
        {
            type Source = ParseIntError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> NumberError {
                let error: ParseIntError = (|v| v)(error);
                NumberError::FailedToParseExponent {
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
        ///SNAFU context selector for the `NumberError::UnexpectedChar` variant
        pub struct UnexpectedCharCtx<__T0, __T1, __T2> {
            #[allow(missing_docs)]
            pub value: __T0,
            #[allow(missing_docs)]
            pub character: __T1,
            #[allow(missing_docs)]
            pub index: __T2,
        }
        #[automatically_derived]
        impl<
            __T0: ::core::fmt::Debug,
            __T1: ::core::fmt::Debug,
            __T2: ::core::fmt::Debug,
        > ::core::fmt::Debug for UnexpectedCharCtx<__T0, __T1, __T2> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "UnexpectedCharCtx",
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
        impl<
            __T0: ::core::marker::Copy,
            __T1: ::core::marker::Copy,
            __T2: ::core::marker::Copy,
        > ::core::marker::Copy for UnexpectedCharCtx<__T0, __T1, __T2> {}
        #[automatically_derived]
        impl<
            __T0: ::core::clone::Clone,
            __T1: ::core::clone::Clone,
            __T2: ::core::clone::Clone,
        > ::core::clone::Clone for UnexpectedCharCtx<__T0, __T1, __T2> {
            #[inline]
            fn clone(&self) -> UnexpectedCharCtx<__T0, __T1, __T2> {
                UnexpectedCharCtx {
                    value: ::core::clone::Clone::clone(&self.value),
                    character: ::core::clone::Clone::clone(&self.character),
                    index: ::core::clone::Clone::clone(&self.index),
                }
            }
        }
        impl<__T0, __T1, __T2> UnexpectedCharCtx<__T0, __T1, __T2> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> NumberError
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<char>,
                __T2: ::core::convert::Into<usize>,
            {
                NumberError::UnexpectedChar {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                    character: ::core::convert::Into::into(self.character),
                    index: ::core::convert::Into::into(self.index),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, NumberError>
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<char>,
                __T2: ::core::convert::Into<usize>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1, __T2> ::snafu::IntoError<NumberError>
        for UnexpectedCharCtx<__T0, __T1, __T2>
        where
            NumberError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<String>,
            __T1: ::core::convert::Into<char>,
            __T2: ::core::convert::Into<usize>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> NumberError {
                NumberError::UnexpectedChar {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                    character: ::core::convert::Into::into(self.character),
                    index: ::core::convert::Into::into(self.index),
                }
            }
        }
        ///SNAFU context selector for the `NumberError::NotAnInteger` variant
        pub struct NotAnIntegerCtx<__T0> {
            #[allow(missing_docs)]
            pub value: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for NotAnIntegerCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "NotAnIntegerCtx",
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for NotAnIntegerCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for NotAnIntegerCtx<__T0> {
            #[inline]
            fn clone(&self) -> NotAnIntegerCtx<__T0> {
                NotAnIntegerCtx {
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        impl<__T0> NotAnIntegerCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> NumberError
            where
                __T0: ::core::convert::Into<String>,
            {
                NumberError::NotAnInteger {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, NumberError>
            where
                __T0: ::core::convert::Into<String>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<NumberError> for NotAnIntegerCtx<__T0>
        where
            NumberError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<String>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> NumberError {
                NumberError::NotAnInteger {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for NumberError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                NumberError::FailedToParseExponent {
                    ref backtrace,
                    ref source,
                    ref value,
                } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "failed to parse exponent of number \"{1}\":\n\t{0}",
                                source, value
                            ),
                        )
                }
                NumberError::UnexpectedChar {
                    ref backtrace,
                    ref character,
                    ref index,
                    ref value,
                } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "failed to parse number \"{2}\":\n\tunexpected character: \'{0}\' at index {1}",
                                character, index, value
                            ),
                        )
                }
                NumberError::NotAnInteger { ref backtrace, ref value } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "failed to parse number \"{0}\":\n\tnot an integer", value
                            ),
                        )
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for NumberError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                NumberError::FailedToParseExponent { .. } => {
                    "NumberError :: FailedToParseExponent"
                }
                NumberError::UnexpectedChar { .. } => "NumberError :: UnexpectedChar",
                NumberError::NotAnInteger { .. } => "NumberError :: NotAnInteger",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                NumberError::FailedToParseExponent { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                NumberError::UnexpectedChar { .. } => ::core::option::Option::None,
                NumberError::NotAnInteger { .. } => ::core::option::Option::None,
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                NumberError::FailedToParseExponent { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                NumberError::UnexpectedChar { .. } => ::core::option::Option::None,
                NumberError::NotAnInteger { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for NumberError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                NumberError::FailedToParseExponent { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                NumberError::UnexpectedChar { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                NumberError::NotAnInteger { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
    /// An error occurred while evaluating a [`Value`].
    #[snafu(visibility(pub), context(suffix(Ctx)), module)]
    pub enum EvaluateError {
        /// Failed to parse a [`Number`] in a [`].
        #[snafu(transparent)]
        FailedToParseNumber { #[snafu(backtrace)] source: NumberError },
        /// Failed to evaluate a regular expression.
        #[snafu(display("failed to evaluate regular expression: {source}"))]
        FailedToEvalRegex { source: regex::Error, backtrace: Backtrace },
        /// A [`Key`] was provided that is not known to the `Interrogator`
        #[snafu(transparent)]
        UnknownKey { #[snafu(backtrace)] source: UnknownKeyError },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EvaluateError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                EvaluateError::FailedToParseNumber { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "FailedToParseNumber",
                        "source",
                        &__self_0,
                    )
                }
                EvaluateError::FailedToEvalRegex {
                    source: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "FailedToEvalRegex",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                EvaluateError::UnknownKey { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UnknownKey",
                        "source",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub mod evaluate_error {
        use super::*;
        impl ::core::convert::From<NumberError> for EvaluateError {
            #[track_caller]
            fn from(error: NumberError) -> Self {
                let error: NumberError = (|v| v)(error);
                EvaluateError::FailedToParseNumber {
                    source: error,
                }
            }
        }
        ///SNAFU context selector for the `EvaluateError::FailedToEvalRegex` variant
        pub struct FailedToEvalRegexCtx;
        #[automatically_derived]
        impl ::core::fmt::Debug for FailedToEvalRegexCtx {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "FailedToEvalRegexCtx")
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for FailedToEvalRegexCtx {}
        #[automatically_derived]
        impl ::core::clone::Clone for FailedToEvalRegexCtx {
            #[inline]
            fn clone(&self) -> FailedToEvalRegexCtx {
                *self
            }
        }
        impl ::snafu::IntoError<EvaluateError> for FailedToEvalRegexCtx
        where
            EvaluateError: ::snafu::Error + ::snafu::ErrorCompat,
        {
            type Source = regex::Error;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> EvaluateError {
                let error: regex::Error = (|v| v)(error);
                EvaluateError::FailedToEvalRegex {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                }
            }
        }
        impl ::core::convert::From<UnknownKeyError> for EvaluateError {
            #[track_caller]
            fn from(error: UnknownKeyError) -> Self {
                let error: UnknownKeyError = (|v| v)(error);
                EvaluateError::UnknownKey {
                    source: error,
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for EvaluateError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                EvaluateError::FailedToParseNumber { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                EvaluateError::FailedToEvalRegex { ref backtrace, ref source } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "failed to evaluate regular expression: {0}", source
                            ),
                        )
                }
                EvaluateError::UnknownKey { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for EvaluateError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                EvaluateError::FailedToParseNumber { .. } => {
                    "EvaluateError :: FailedToParseNumber"
                }
                EvaluateError::FailedToEvalRegex { .. } => {
                    "EvaluateError :: FailedToEvalRegex"
                }
                EvaluateError::UnknownKey { .. } => "EvaluateError :: UnknownKey",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                EvaluateError::FailedToParseNumber { ref source, .. } => {
                    source.as_error_source().source()
                }
                EvaluateError::FailedToEvalRegex { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                EvaluateError::UnknownKey { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                EvaluateError::FailedToParseNumber { ref source, .. } => {
                    source.as_error_source().source()
                }
                EvaluateError::FailedToEvalRegex { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                EvaluateError::UnknownKey { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for EvaluateError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                EvaluateError::FailedToParseNumber { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
                EvaluateError::FailedToEvalRegex { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                EvaluateError::UnknownKey { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
            }
        }
    }
    /// Contains one or more errors that occurred during deserialization.
    pub struct DeserializeError {
        /// A table of errors keyed by the name of the format which failed to
        /// deserialize.
        pub sources: HashMap<&'static str, erased_serde::Error>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DeserializeError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "DeserializeError",
                "sources",
                &&self.sources,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for DeserializeError {
        #[inline]
        fn default() -> DeserializeError {
            DeserializeError {
                sources: ::core::default::Default::default(),
            }
        }
    }
    impl DeserializeError {
        /// Adds a [`erased_serde::Error`], key'ed by `format` to the table of
        /// deserialization errors.
        pub fn add(&mut self, format: &'static str, err: erased_serde::Error) {
            self.sources.insert(format, err);
        }
    }
    impl Display for DeserializeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("failed to deserialize"))?;
            for (format, err) in &self.sources {
                f.write_fmt(format_args!("\n\t{0}: {1}", format, err))?;
            }
            Ok(())
        }
    }
    impl StdError for DeserializeError {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            self.sources.iter().next().map(|(_, err)| err as _)
        }
    }
    /// A list of errors, one per implementation of
    /// [`Resolve`](crate::resolve::Resolve) attached to the
    /// [`Interrogator`](crate::Interrogator), indicating why a source failed to
    /// resolve.
    pub struct ResolveErrors {
        /// A list of errors, one per implementation of [`Resolve`].
        pub sources: Vec<ResolveError>,
        pub backtrace: Backtrace,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ResolveErrors {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "ResolveErrors",
                "sources",
                &self.sources,
                "backtrace",
                &&self.backtrace,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for ResolveErrors {
        #[inline]
        fn default() -> ResolveErrors {
            ResolveErrors {
                sources: ::core::default::Default::default(),
                backtrace: ::core::default::Default::default(),
            }
        }
    }
    impl IntoIterator for ResolveErrors {
        type Item = ResolveError;
        type IntoIter = std::vec::IntoIter<Self::Item>;
        fn into_iter(self) -> Self::IntoIter {
            self.sources.into_iter()
        }
    }
    impl<'a> IntoIterator for &'a ResolveErrors {
        type Item = &'a ResolveError;
        type IntoIter = std::slice::Iter<'a, ResolveError>;
        fn into_iter(self) -> Self::IntoIter {
            self.sources.iter()
        }
    }
    impl Deref for ResolveErrors {
        type Target = Vec<ResolveError>;
        fn deref(&self) -> &Self::Target {
            &self.sources
        }
    }
    impl From<ResolveError> for ResolveErrors {
        fn from(error: ResolveError) -> Self {
            Self {
                sources: <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([error])),
                backtrace: Backtrace::capture(),
            }
        }
    }
    impl ResolveErrors {
        #[must_use]
        /// Create a new [`ResolveErrors`].
        pub fn new() -> Self {
            Self {
                sources: Vec::default(),
                backtrace: Backtrace::capture(),
            }
        }
        /// Appends a new [`ResolveError`] to the list of errors.
        pub fn push(&mut self, err: ResolveError) {
            self.sources.push(err);
        }
        /// Appends a new [`NotFoundError`] to the list of errors.
        pub fn push_not_found(&mut self, uri: AbsoluteUri) {
            self.sources.push(ResolveError::not_found(uri));
        }
        /// Appends a new [`ResolveError`] from a [`ResolveErrorSource`] to the list
        /// of errors.
        pub fn push_new(
            &mut self,
            err: impl Into<ResolveErrorSource>,
            uri: AbsoluteUri,
        ) {
            self.sources
                .push(ResolveError {
                    source: err.into(),
                    uri,
                    referring_location: None,
                });
        }
        /// Sets the `referring_location` of each `ResolveError` to `referring_location`.
        pub fn set_referring_location(&mut self, referring_location: AbsoluteUri) {
            for err in &mut self.sources {
                err.referring_location = Some(referring_location.clone());
            }
        }
    }
    impl Display for ResolveErrors {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_fmt(format_args!("failed to resolve schema"))?;
            for err in &self.sources {
                f.write_fmt(format_args!("\n\t{0}", err))?;
            }
            Ok(())
        }
    }
    impl StdError for ResolveErrors {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            self.sources.first().map(|err| err as _)
        }
    }
    /// An error occurred while attempting to resolve a source within the source.
    #[snafu(
        display("failed to resolve source \"{uri}\"\n\ncaused by:\n\t{source}"),
        visibility(pub),
        context(suffix(Ctx)),
        module
    )]
    pub struct ResolveError {
        /// The source of the error.
        pub source: ResolveErrorSource,
        /// The [`AbsoluteUri`] of the source which was not able to be resolved.
        pub uri: AbsoluteUri,
        /// The [`AbsoluteUri`] of the referring keyword which was not found, if
        /// any.
        ///
        /// The path of the keyword can be found as a fragment of the URI.
        pub referring_location: Option<AbsoluteUri>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ResolveError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ResolveError",
                "source",
                &self.source,
                "uri",
                &self.uri,
                "referring_location",
                &&self.referring_location,
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for ResolveError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "ResolveError",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                Self { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                Self { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for ResolveError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for ResolveError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self { ref referring_location, ref source, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "failed to resolve source \"{1}\"\n\ncaused by:\n\t{0}",
                                source, uri
                            ),
                        )
                }
            }
        }
    }
    pub mod resolve_error {
        use super::*;
        ///SNAFU context selector for the `ResolveError` error
        pub struct ResolveCtx<__T0, __T1> {
            #[allow(missing_docs)]
            pub uri: __T0,
            #[allow(missing_docs)]
            pub referring_location: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
        for ResolveCtx<__T0, __T1> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ResolveCtx",
                    "uri",
                    &self.uri,
                    "referring_location",
                    &&self.referring_location,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
        for ResolveCtx<__T0, __T1> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
        for ResolveCtx<__T0, __T1> {
            #[inline]
            fn clone(&self) -> ResolveCtx<__T0, __T1> {
                ResolveCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                    referring_location: ::core::clone::Clone::clone(
                        &self.referring_location,
                    ),
                }
            }
        }
        impl<__T0, __T1> ::snafu::IntoError<ResolveError> for ResolveCtx<__T0, __T1>
        where
            ResolveError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
            __T1: ::core::convert::Into<Option<AbsoluteUri>>,
        {
            type Source = ResolveErrorSource;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> ResolveError {
                let error: ResolveErrorSource = (|v| v)(error);
                ResolveError {
                    source: error,
                    uri: ::core::convert::Into::into(self.uri),
                    referring_location: ::core::convert::Into::into(
                        self.referring_location,
                    ),
                }
            }
        }
    }
    impl ResolveError {
        /// Create a new [`ResolveError`].
        pub fn new(err: impl Into<ResolveErrorSource>, uri: AbsoluteUri) -> Self {
            Self {
                source: err.into(),
                uri,
                referring_location: None,
            }
        }
        /// Sets the `referring_location` of the `ResolveError` to `referring_location`.
        pub fn set_referring_location(&mut self, referring_location: AbsoluteUri) {
            self.referring_location = Some(referring_location);
        }
    }
    /// The source of a [`ResolveError`]
    #[snafu(visibility(pub), context(suffix(Ctx)), module)]
    pub enum ResolveErrorSource {
        /// The [`std::io::Error`] which occurred while resolving a source.
        #[snafu(transparent)]
        Io { source: std::io::Error, backtrace: Backtrace },
        /// The [`reqwest::Error`] which occurred while resolving a source.
        #[snafu(transparent)]
        Reqwest { source: reqwest::Error, backtrace: Backtrace },
        /// The path, as a JSON [`Pointer`], failed to resolve.
        #[snafu(transparent)]
        PointerMalformed { source: MalformedPointerError, backtrace: Backtrace },
        /// A source or schema could not be found.
        #[snafu(display("unable to resolve \"{uri}\" due to not being found"))]
        NotFound {
            /// The URI of the source which was not found.
            uri: AbsoluteUri,
            backtrace: Backtrace,
        },
        /// Any other error which occurred while resolving a source.
        #[snafu(whatever, display("{message}"))]
        Custom {
            message: String,
            #[snafu(source(from(Box<dyn'static+std::error::Error+Send+Sync>, Some)))]
            source: Box<dyn 'static + error::Error + Send + Sync>,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ResolveErrorSource {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ResolveErrorSource::Io { source: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Io",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                ResolveErrorSource::Reqwest { source: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Reqwest",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                ResolveErrorSource::PointerMalformed {
                    source: __self_0,
                    backtrace: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "PointerMalformed",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                ResolveErrorSource::NotFound { uri: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "NotFound",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                ResolveErrorSource::Custom { message: __self_0, source: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Custom",
                        "message",
                        __self_0,
                        "source",
                        &__self_1,
                    )
                }
            }
        }
    }
    pub mod resolve_error_source {
        use super::*;
        impl ::core::convert::From<std::io::Error> for ResolveErrorSource {
            #[track_caller]
            fn from(error: std::io::Error) -> Self {
                let error: std::io::Error = (|v| v)(error);
                ResolveErrorSource::Io {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                }
            }
        }
        impl ::core::convert::From<reqwest::Error> for ResolveErrorSource {
            #[track_caller]
            fn from(error: reqwest::Error) -> Self {
                let error: reqwest::Error = (|v| v)(error);
                ResolveErrorSource::Reqwest {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                }
            }
        }
        impl ::core::convert::From<MalformedPointerError> for ResolveErrorSource {
            #[track_caller]
            fn from(error: MalformedPointerError) -> Self {
                let error: MalformedPointerError = (|v| v)(error);
                ResolveErrorSource::PointerMalformed {
                    backtrace: {
                        use ::snafu::AsErrorSource;
                        let error = error.as_error_source();
                        ::snafu::GenerateImplicitData::generate_with_source(error)
                    },
                    source: error,
                }
            }
        }
        ///SNAFU context selector for the `ResolveErrorSource::NotFound` variant
        pub struct NotFoundCtx<__T0> {
            #[allow(missing_docs)]
            pub uri: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for NotFoundCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "NotFoundCtx",
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for NotFoundCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for NotFoundCtx<__T0> {
            #[inline]
            fn clone(&self) -> NotFoundCtx<__T0> {
                NotFoundCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0> NotFoundCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> ResolveErrorSource
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                ResolveErrorSource::NotFound {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, ResolveErrorSource>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<ResolveErrorSource> for NotFoundCtx<__T0>
        where
            ResolveErrorSource: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> ResolveErrorSource {
                ResolveErrorSource::NotFound {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
        impl ::snafu::FromString for ResolveErrorSource {
            type Source = Box<dyn 'static + std::error::Error + Send + Sync>;
            #[track_caller]
            fn without_source(message: String) -> Self {
                ResolveErrorSource::Custom {
                    source: core::option::Option::None,
                    message: message,
                }
            }
            #[track_caller]
            fn with_source(error: Self::Source, message: String) -> Self {
                ResolveErrorSource::Custom {
                    source: (Some)(error),
                    message: message,
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for ResolveErrorSource {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                ResolveErrorSource::Io { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                ResolveErrorSource::Reqwest { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                ResolveErrorSource::PointerMalformed { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                ResolveErrorSource::NotFound { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "unable to resolve \"{0}\" due to not being found", uri
                            ),
                        )
                }
                ResolveErrorSource::Custom { ref message, ref source } => {
                    __snafu_display_formatter.write_fmt(format_args!("{0}", message))
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for ResolveErrorSource
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                ResolveErrorSource::Io { .. } => "ResolveErrorSource :: Io",
                ResolveErrorSource::Reqwest { .. } => "ResolveErrorSource :: Reqwest",
                ResolveErrorSource::PointerMalformed { .. } => {
                    "ResolveErrorSource :: PointerMalformed"
                }
                ResolveErrorSource::NotFound { .. } => "ResolveErrorSource :: NotFound",
                ResolveErrorSource::Custom { .. } => "ResolveErrorSource :: Custom",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                ResolveErrorSource::Io { ref source, .. } => {
                    source.as_error_source().source()
                }
                ResolveErrorSource::Reqwest { ref source, .. } => {
                    source.as_error_source().source()
                }
                ResolveErrorSource::PointerMalformed { ref source, .. } => {
                    source.as_error_source().source()
                }
                ResolveErrorSource::NotFound { .. } => ::core::option::Option::None,
                ResolveErrorSource::Custom { ref source, .. } => {
                    source.as_ref().map(|e| e.as_error_source())
                }
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                ResolveErrorSource::Io { ref source, .. } => {
                    source.as_error_source().source()
                }
                ResolveErrorSource::Reqwest { ref source, .. } => {
                    source.as_error_source().source()
                }
                ResolveErrorSource::PointerMalformed { ref source, .. } => {
                    source.as_error_source().source()
                }
                ResolveErrorSource::NotFound { .. } => ::core::option::Option::None,
                ResolveErrorSource::Custom { ref source, .. } => {
                    source.as_ref().map(|e| e.as_error_source())
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for ResolveErrorSource {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                ResolveErrorSource::Io { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                ResolveErrorSource::Reqwest { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                ResolveErrorSource::PointerMalformed { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                ResolveErrorSource::NotFound { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                ResolveErrorSource::Custom { .. } => ::core::option::Option::None,
            }
        }
    }
    /// The expected type of a [`Value`].
    pub enum Expected {
        /// Expected a boolean
        Bool,
        /// Expected a number
        Number,
        /// Expected a string
        String,
        /// Execpted an array
        Array,
        /// Expected an object
        Object,
        /// Expected any of the types in the slice
        AnyOf(&'static [Expected]),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Expected {
        #[inline]
        fn clone(&self) -> Expected {
            let _: ::core::clone::AssertParamIsClone<&'static [Expected]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Expected {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Expected::Bool => ::core::fmt::Formatter::write_str(f, "Bool"),
                Expected::Number => ::core::fmt::Formatter::write_str(f, "Number"),
                Expected::String => ::core::fmt::Formatter::write_str(f, "String"),
                Expected::Array => ::core::fmt::Formatter::write_str(f, "Array"),
                Expected::Object => ::core::fmt::Formatter::write_str(f, "Object"),
                Expected::AnyOf(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AnyOf",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Expected {}
    impl Display for Expected {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expected::Bool => f.write_fmt(format_args!("Bool")),
                Expected::Number => f.write_fmt(format_args!("Number")),
                Expected::String => f.write_fmt(format_args!("String")),
                Expected::Array => f.write_fmt(format_args!("Array")),
                Expected::Object => f.write_fmt(format_args!("Object")),
                Expected::AnyOf(anyof) => {
                    f.write_fmt(format_args!("["))?;
                    for (i, expected) in anyof.iter().enumerate() {
                        if i > 0 {
                            f.write_fmt(format_args!(", "))?;
                        }
                        f.write_fmt(format_args!("{0}", expected))?;
                    }
                    f.write_fmt(format_args!("]"))
                }
            }
        }
    }
    /// A [`Value`] was not of the expected type.
    #[snafu(
        display("expected value with type {expected}, found {actual:?}"),
        context(suffix(Ctx)),
        module
    )]
    pub struct InvalidTypeError {
        /// The expected type of value.
        pub expected: Expected,
        /// The actual value.
        pub actual: Box<Value>,
        pub backtrace: snafu::Backtrace,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InvalidTypeError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "InvalidTypeError",
                "expected",
                &self.expected,
                "actual",
                &self.actual,
                "backtrace",
                &&self.backtrace,
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for InvalidTypeError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "InvalidTypeError",
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
    impl ::snafu::ErrorCompat for InvalidTypeError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for InvalidTypeError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self { ref actual, ref backtrace, ref expected } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "expected value with type {1}, found {0:?}", actual,
                                expected
                            ),
                        )
                }
            }
        }
    }
    mod invalid_type_error {
        use super::*;
        ///SNAFU context selector for the `InvalidTypeError` error
        pub(super) struct InvalidTypeCtx<__T0, __T1> {
            #[allow(missing_docs)]
            pub(super) expected: __T0,
            #[allow(missing_docs)]
            pub(super) actual: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
        for InvalidTypeCtx<__T0, __T1> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "InvalidTypeCtx",
                    "expected",
                    &self.expected,
                    "actual",
                    &&self.actual,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
        for InvalidTypeCtx<__T0, __T1> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
        for InvalidTypeCtx<__T0, __T1> {
            #[inline]
            fn clone(&self) -> InvalidTypeCtx<__T0, __T1> {
                InvalidTypeCtx {
                    expected: ::core::clone::Clone::clone(&self.expected),
                    actual: ::core::clone::Clone::clone(&self.actual),
                }
            }
        }
        impl<__T0, __T1> InvalidTypeCtx<__T0, __T1> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub(super) fn build(self) -> InvalidTypeError
            where
                __T0: ::core::convert::Into<Expected>,
                __T1: ::core::convert::Into<Box<Value>>,
            {
                InvalidTypeError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    expected: ::core::convert::Into::into(self.expected),
                    actual: ::core::convert::Into::into(self.actual),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub(super) fn fail<__T>(
                self,
            ) -> ::core::result::Result<__T, InvalidTypeError>
            where
                __T0: ::core::convert::Into<Expected>,
                __T1: ::core::convert::Into<Box<Value>>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1> ::snafu::IntoError<InvalidTypeError>
        for InvalidTypeCtx<__T0, __T1>
        where
            InvalidTypeError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<Expected>,
            __T1: ::core::convert::Into<Box<Value>>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> InvalidTypeError {
                InvalidTypeError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    expected: ::core::convert::Into::into(self.expected),
                    actual: ::core::convert::Into::into(self.actual),
                }
            }
        }
    }
    /// An error occurred while attempting to identify a schema
    #[snafu(context(suffix(Ctx)), module)]
    pub enum IdentifyError {
        /// The URI could not be parsed.
        #[snafu(transparent)]
        InvalidUri { #[snafu(backtrace)] source: Error },
        /// The URI is not absolute (i.e. contains a non-empty fragment).
        #[snafu(display("the $id of a schema is not absolute: {uri}"))]
        FragmentedId { uri: Uri, backtrace: Backtrace },
        /// The value of `$id` was not a string
        #[snafu(
            display(
                "the {keyword} of a schema must be a string in the form of a uri; found {value:?}"
            )
        )]
        NotAString {
            /// The keyword which was not a string
            keyword: &'static str,
            /// The value of the keyword
            value: Box<Value>,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IdentifyError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                IdentifyError::InvalidUri { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "InvalidUri",
                        "source",
                        &__self_0,
                    )
                }
                IdentifyError::FragmentedId { uri: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "FragmentedId",
                        "uri",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
                IdentifyError::NotAString { keyword: __self_0, value: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "NotAString",
                        "keyword",
                        __self_0,
                        "value",
                        &__self_1,
                    )
                }
            }
        }
    }
    mod identify_error {
        use super::*;
        impl ::core::convert::From<Error> for IdentifyError {
            #[track_caller]
            fn from(error: Error) -> Self {
                let error: Error = (|v| v)(error);
                IdentifyError::InvalidUri {
                    source: error,
                }
            }
        }
        ///SNAFU context selector for the `IdentifyError::FragmentedId` variant
        pub(super) struct FragmentedIdCtx<__T0> {
            #[allow(missing_docs)]
            pub(super) uri: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for FragmentedIdCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "FragmentedIdCtx",
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for FragmentedIdCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for FragmentedIdCtx<__T0> {
            #[inline]
            fn clone(&self) -> FragmentedIdCtx<__T0> {
                FragmentedIdCtx {
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0> FragmentedIdCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub(super) fn build(self) -> IdentifyError
            where
                __T0: ::core::convert::Into<Uri>,
            {
                IdentifyError::FragmentedId {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub(super) fn fail<__T>(self) -> ::core::result::Result<__T, IdentifyError>
            where
                __T0: ::core::convert::Into<Uri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<IdentifyError> for FragmentedIdCtx<__T0>
        where
            IdentifyError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<Uri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> IdentifyError {
                IdentifyError::FragmentedId {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
        ///SNAFU context selector for the `IdentifyError::NotAString` variant
        pub(super) struct NotAStringCtx<__T0, __T1> {
            #[allow(missing_docs)]
            pub(super) keyword: __T0,
            #[allow(missing_docs)]
            pub(super) value: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
        for NotAStringCtx<__T0, __T1> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "NotAStringCtx",
                    "keyword",
                    &self.keyword,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
        for NotAStringCtx<__T0, __T1> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
        for NotAStringCtx<__T0, __T1> {
            #[inline]
            fn clone(&self) -> NotAStringCtx<__T0, __T1> {
                NotAStringCtx {
                    keyword: ::core::clone::Clone::clone(&self.keyword),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        impl<__T0, __T1> NotAStringCtx<__T0, __T1> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub(super) fn build(self) -> IdentifyError
            where
                __T0: ::core::convert::Into<&'static str>,
                __T1: ::core::convert::Into<Box<Value>>,
            {
                IdentifyError::NotAString {
                    keyword: ::core::convert::Into::into(self.keyword),
                    value: ::core::convert::Into::into(self.value),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub(super) fn fail<__T>(self) -> ::core::result::Result<__T, IdentifyError>
            where
                __T0: ::core::convert::Into<&'static str>,
                __T1: ::core::convert::Into<Box<Value>>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1> ::snafu::IntoError<IdentifyError> for NotAStringCtx<__T0, __T1>
        where
            IdentifyError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<&'static str>,
            __T1: ::core::convert::Into<Box<Value>>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> IdentifyError {
                IdentifyError::NotAString {
                    keyword: ::core::convert::Into::into(self.keyword),
                    value: ::core::convert::Into::into(self.value),
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for IdentifyError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                IdentifyError::InvalidUri { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                IdentifyError::FragmentedId { ref backtrace, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!("the $id of a schema is not absolute: {0}", uri),
                        )
                }
                IdentifyError::NotAString { ref keyword, ref value } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "the {0} of a schema must be a string in the form of a uri; found {1:?}",
                                keyword, value
                            ),
                        )
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for IdentifyError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                IdentifyError::InvalidUri { .. } => "IdentifyError :: InvalidUri",
                IdentifyError::FragmentedId { .. } => "IdentifyError :: FragmentedId",
                IdentifyError::NotAString { .. } => "IdentifyError :: NotAString",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                IdentifyError::InvalidUri { ref source, .. } => {
                    source.as_error_source().source()
                }
                IdentifyError::FragmentedId { .. } => ::core::option::Option::None,
                IdentifyError::NotAString { .. } => ::core::option::Option::None,
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                IdentifyError::InvalidUri { ref source, .. } => {
                    source.as_error_source().source()
                }
                IdentifyError::FragmentedId { .. } => ::core::option::Option::None,
                IdentifyError::NotAString { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for IdentifyError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                IdentifyError::InvalidUri { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
                IdentifyError::FragmentedId { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                IdentifyError::NotAString { .. } => ::core::option::Option::None,
            }
        }
    }
    /// A [`Dialect`] with the [`AbsoluteUri`] was not able to be found.
    #[snafu(display("dialect not found: {id}"), context(suffix(Ctx)), module)]
    pub struct DialectNotFoundError {
        /// The [`AbsoluteUri`] of the [`Dialect`] that was not able
        /// to be found.
        pub id: AbsoluteUri,
        pub backtrace: Backtrace,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DialectNotFoundError {
        #[inline]
        fn clone(&self) -> DialectNotFoundError {
            DialectNotFoundError {
                id: ::core::clone::Clone::clone(&self.id),
                backtrace: ::core::clone::Clone::clone(&self.backtrace),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DialectNotFoundError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "DialectNotFoundError",
                "id",
                &self.id,
                "backtrace",
                &&self.backtrace,
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for DialectNotFoundError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "DialectNotFoundError",
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
    impl ::snafu::ErrorCompat for DialectNotFoundError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for DialectNotFoundError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self { ref backtrace, ref id } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("dialect not found: {0}", id))
                }
            }
        }
    }
    mod dialect_not_found_error {
        use super::*;
        ///SNAFU context selector for the `DialectNotFoundError` error
        pub(super) struct DialectNotFoundCtx<__T0> {
            #[allow(missing_docs)]
            pub(super) id: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for DialectNotFoundCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "DialectNotFoundCtx",
                    "id",
                    &&self.id,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy
        for DialectNotFoundCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone
        for DialectNotFoundCtx<__T0> {
            #[inline]
            fn clone(&self) -> DialectNotFoundCtx<__T0> {
                DialectNotFoundCtx {
                    id: ::core::clone::Clone::clone(&self.id),
                }
            }
        }
        impl<__T0> DialectNotFoundCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub(super) fn build(self) -> DialectNotFoundError
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                DialectNotFoundError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    id: ::core::convert::Into::into(self.id),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub(super) fn fail<__T>(
                self,
            ) -> ::core::result::Result<__T, DialectNotFoundError>
            where
                __T0: ::core::convert::Into<AbsoluteUri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<DialectNotFoundError> for DialectNotFoundCtx<__T0>
        where
            DialectNotFoundError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> DialectNotFoundError {
                DialectNotFoundError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    id: ::core::convert::Into::into(self.id),
                }
            }
        }
    }
    impl DialectNotFoundError {
        #[must_use]
        /// Create a new [`DialectNotFoundError`].
        pub fn new(id: AbsoluteUri) -> Self {
            Self {
                id,
                backtrace: Backtrace::capture(),
            }
        }
    }
    /// A schema [`Key`](crate::schema::Key) was not found.
    ///
    /// If this is encountered, odds are it is because you have two
    /// [`Interrogator`](crate::Interrogator)s and mismatched keys.
    #[snafu(
        display("the provided key could not be found"),
        context(suffix(Ctx)),
        module
    )]
    pub struct UnknownKeyError {
        pub key: Key,
        pub backtrace: Backtrace,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnknownKeyError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "UnknownKeyError",
                "key",
                &self.key,
                "backtrace",
                &&self.backtrace,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UnknownKeyError {
        #[inline]
        fn clone(&self) -> UnknownKeyError {
            UnknownKeyError {
                key: ::core::clone::Clone::clone(&self.key),
                backtrace: ::core::clone::Clone::clone(&self.backtrace),
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for UnknownKeyError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "UnknownKeyError",
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
    impl ::snafu::ErrorCompat for UnknownKeyError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for UnknownKeyError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self { ref backtrace, ref key } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("the provided key could not be found"))
                }
            }
        }
    }
    mod unknown_key_error {
        use super::*;
        ///SNAFU context selector for the `UnknownKeyError` error
        pub(super) struct UnknownKeyCtx<__T0> {
            #[allow(missing_docs)]
            pub(super) key: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for UnknownKeyCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "UnknownKeyCtx",
                    "key",
                    &&self.key,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for UnknownKeyCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for UnknownKeyCtx<__T0> {
            #[inline]
            fn clone(&self) -> UnknownKeyCtx<__T0> {
                UnknownKeyCtx {
                    key: ::core::clone::Clone::clone(&self.key),
                }
            }
        }
        impl<__T0> UnknownKeyCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub(super) fn build(self) -> UnknownKeyError
            where
                __T0: ::core::convert::Into<Key>,
            {
                UnknownKeyError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    key: ::core::convert::Into::into(self.key),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub(super) fn fail<__T>(self) -> ::core::result::Result<__T, UnknownKeyError>
            where
                __T0: ::core::convert::Into<Key>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<UnknownKeyError> for UnknownKeyCtx<__T0>
        where
            UnknownKeyError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<Key>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> UnknownKeyError {
                UnknownKeyError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    key: ::core::convert::Into::into(self.key),
                }
            }
        }
    }
    /// A slice or string overflowed an allowed length maximum of `M`.
    #[snafu(
        display("The value {value} overflowed {}", Self::MAX),
        context(suffix(Ctx)),
        module
    )]
    pub struct OverflowError {
        pub value: u64,
        pub backtrace: Backtrace,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for OverflowError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "OverflowError",
                "value",
                &self.value,
                "backtrace",
                &&self.backtrace,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for OverflowError {
        #[inline]
        fn clone(&self) -> OverflowError {
            OverflowError {
                value: ::core::clone::Clone::clone(&self.value),
                backtrace: ::core::clone::Clone::clone(&self.backtrace),
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for OverflowError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "OverflowError",
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
    impl ::snafu::ErrorCompat for OverflowError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for OverflowError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self { ref backtrace, ref value } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "The value {1} overflowed {0}", Self::MAX, value
                            ),
                        )
                }
            }
        }
    }
    mod overflow_error {
        use super::*;
        ///SNAFU context selector for the `OverflowError` error
        pub(super) struct OverflowCtx<__T0> {
            #[allow(missing_docs)]
            pub(super) value: __T0,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for OverflowCtx<__T0> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "OverflowCtx",
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy> ::core::marker::Copy for OverflowCtx<__T0> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone> ::core::clone::Clone for OverflowCtx<__T0> {
            #[inline]
            fn clone(&self) -> OverflowCtx<__T0> {
                OverflowCtx {
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        impl<__T0> OverflowCtx<__T0> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub(super) fn build(self) -> OverflowError
            where
                __T0: ::core::convert::Into<u64>,
            {
                OverflowError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub(super) fn fail<__T>(self) -> ::core::result::Result<__T, OverflowError>
            where
                __T0: ::core::convert::Into<u64>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0> ::snafu::IntoError<OverflowError> for OverflowCtx<__T0>
        where
            OverflowError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<u64>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> OverflowError {
                OverflowError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    value: ::core::convert::Into::into(self.value),
                }
            }
        }
    }
    impl OverflowError {
        /// The maximum allowed size.
        pub const MAX: u64 = usize::MAX as u64;
    }
    impl From<u64> for OverflowError {
        fn from(value: u64) -> Self {
            Self(value)
        }
    }
    /// An error occurred while parsing a ref
    #[snafu(context(suffix(Ctx)), module)]
    pub enum RefError {
        /// The ref value was not a string.
        #[snafu(transparent)]
        UnexpectedType { #[snafu(backtrace)] source: InvalidTypeError },
        /// The ref value failed to parse as a URI.
        #[snafu(transparent)]
        UriError { #[snafu(backtrace)] source: Error },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RefError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                RefError::UnexpectedType { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UnexpectedType",
                        "source",
                        &__self_0,
                    )
                }
                RefError::UriError { source: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "UriError",
                        "source",
                        &__self_0,
                    )
                }
            }
        }
    }
    mod ref_error {
        use super::*;
        impl ::core::convert::From<InvalidTypeError> for RefError {
            #[track_caller]
            fn from(error: InvalidTypeError) -> Self {
                let error: InvalidTypeError = (|v| v)(error);
                RefError::UnexpectedType {
                    source: error,
                }
            }
        }
        impl ::core::convert::From<Error> for RefError {
            #[track_caller]
            fn from(error: Error) -> Self {
                let error: Error = (|v| v)(error);
                RefError::UriError {
                    source: error,
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for RefError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                RefError::UnexpectedType { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
                RefError::UriError { ref source, .. } => {
                    ::core::fmt::Display::fmt(source, __snafu_display_formatter)
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for RefError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                RefError::UnexpectedType { .. } => "RefError :: UnexpectedType",
                RefError::UriError { .. } => "RefError :: UriError",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                RefError::UnexpectedType { ref source, .. } => {
                    source.as_error_source().source()
                }
                RefError::UriError { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                RefError::UnexpectedType { ref source, .. } => {
                    source.as_error_source().source()
                }
                RefError::UriError { ref source, .. } => {
                    source.as_error_source().source()
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for RefError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                RefError::UnexpectedType { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
                RefError::UriError { ref source, .. } => {
                    ::snafu::ErrorCompat::backtrace(source)
                }
            }
        }
    }
    /// A [`Schema`] contains a cyclic dependency.
    #[snafu(display("schema \"{from}\" contains a cyclic dependency to \"{to}\""))]
    pub struct CyclicDependencyError {
        /// The [`AbsoluteUri`] of the schema which, through transitive
        /// dependencies, creates a cycle.
        pub from: AbsoluteUri,
        /// The [`AbsoluteUri`] of the schema which is the target of the cycle.
        pub to: AbsoluteUri,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CyclicDependencyError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CyclicDependencyError",
                "from",
                &self.from,
                "to",
                &&self.to,
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for CyclicDependencyError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "CyclicDependencyError",
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
    impl ::snafu::ErrorCompat for CyclicDependencyError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for CyclicDependencyError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self { ref from, ref to } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "schema \"{0}\" contains a cyclic dependency to \"{1}\"",
                                from, to
                            ),
                        )
                }
            }
        }
    }
    ///SNAFU context selector for the `CyclicDependencyError` error
    struct CyclicDependencySnafu<__T0, __T1> {
        #[allow(missing_docs)]
        from: __T0,
        #[allow(missing_docs)]
        to: __T1,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
    for CyclicDependencySnafu<__T0, __T1> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CyclicDependencySnafu",
                "from",
                &self.from,
                "to",
                &&self.to,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
    for CyclicDependencySnafu<__T0, __T1> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
    for CyclicDependencySnafu<__T0, __T1> {
        #[inline]
        fn clone(&self) -> CyclicDependencySnafu<__T0, __T1> {
            CyclicDependencySnafu {
                from: ::core::clone::Clone::clone(&self.from),
                to: ::core::clone::Clone::clone(&self.to),
            }
        }
    }
    impl<__T0, __T1> CyclicDependencySnafu<__T0, __T1> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        fn build(self) -> CyclicDependencyError
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
            __T1: ::core::convert::Into<AbsoluteUri>,
        {
            CyclicDependencyError {
                from: ::core::convert::Into::into(self.from),
                to: ::core::convert::Into::into(self.to),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[track_caller]
        fn fail<__T>(self) -> ::core::result::Result<__T, CyclicDependencyError>
        where
            __T0: ::core::convert::Into<AbsoluteUri>,
            __T1: ::core::convert::Into<AbsoluteUri>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0, __T1> ::snafu::IntoError<CyclicDependencyError>
    for CyclicDependencySnafu<__T0, __T1>
    where
        CyclicDependencyError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<AbsoluteUri>,
        __T1: ::core::convert::Into<AbsoluteUri>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> CyclicDependencyError {
            CyclicDependencyError {
                from: ::core::convert::Into::into(self.from),
                to: ::core::convert::Into::into(self.to),
            }
        }
    }
    #[snafu(
        context(suffix(Ctx)),
        module,
        display("unknown anchor: \"{anchor}\" in URI \"{uri}\"")
    )]
    pub struct UnknownAnchorError {
        /// The anchor which was not found.
        pub anchor: String,
        /// The URI of the keyword which referenced the anchor.
        pub uri: AbsoluteUri,
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for UnknownAnchorError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "UnknownAnchorError",
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
    impl ::snafu::ErrorCompat for UnknownAnchorError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for UnknownAnchorError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self { ref anchor, ref uri } => {
                    __snafu_display_formatter
                        .write_fmt(
                            format_args!(
                                "unknown anchor: \"{0}\" in URI \"{1}\"", anchor, uri
                            ),
                        )
                }
            }
        }
    }
    mod unknown_anchor_error {
        use super::*;
        ///SNAFU context selector for the `UnknownAnchorError` error
        pub(super) struct UnknownAnchorCtx<__T0, __T1> {
            #[allow(missing_docs)]
            pub(super) anchor: __T0,
            #[allow(missing_docs)]
            pub(super) uri: __T1,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug> ::core::fmt::Debug
        for UnknownAnchorCtx<__T0, __T1> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UnknownAnchorCtx",
                    "anchor",
                    &self.anchor,
                    "uri",
                    &&self.uri,
                )
            }
        }
        #[automatically_derived]
        impl<__T0: ::core::marker::Copy, __T1: ::core::marker::Copy> ::core::marker::Copy
        for UnknownAnchorCtx<__T0, __T1> {}
        #[automatically_derived]
        impl<__T0: ::core::clone::Clone, __T1: ::core::clone::Clone> ::core::clone::Clone
        for UnknownAnchorCtx<__T0, __T1> {
            #[inline]
            fn clone(&self) -> UnknownAnchorCtx<__T0, __T1> {
                UnknownAnchorCtx {
                    anchor: ::core::clone::Clone::clone(&self.anchor),
                    uri: ::core::clone::Clone::clone(&self.uri),
                }
            }
        }
        impl<__T0, __T1> UnknownAnchorCtx<__T0, __T1> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub(super) fn build(self) -> UnknownAnchorError
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<AbsoluteUri>,
            {
                UnknownAnchorError {
                    anchor: ::core::convert::Into::into(self.anchor),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub(super) fn fail<__T>(
                self,
            ) -> ::core::result::Result<__T, UnknownAnchorError>
            where
                __T0: ::core::convert::Into<String>,
                __T1: ::core::convert::Into<AbsoluteUri>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1> ::snafu::IntoError<UnknownAnchorError>
        for UnknownAnchorCtx<__T0, __T1>
        where
            UnknownAnchorError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<String>,
            __T1: ::core::convert::Into<AbsoluteUri>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> UnknownAnchorError {
                UnknownAnchorError {
                    anchor: ::core::convert::Into::into(self.anchor),
                    uri: ::core::convert::Into::into(self.uri),
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UnknownAnchorError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "UnknownAnchorError",
                "anchor",
                &self.anchor,
                "uri",
                &&self.uri,
            )
        }
    }
}
