//! Logical errors that can occur during usage of this crate.
//!
//! Validation errors are defined within their respective keyword's module.

mod error;

pub use error::{
    AnchorEmptyError, AnchorError, AnchorInvalidCharError, AnchorInvalidLeadCharError,
    AnchorNotEmptyError, AuthorityError, BuildError, CompileError, DeserializationError,
    DeserializeError, DialectError, DialectNotFoundError, DialectUnknownError, EvaluateError,
    EvaluateRegexError, IdentifyError, InvalidPortError, LocateSchemasError, MalformedPointerError,
    NotFoundError, NumberError, OverflowError, PointerError, RegexError, RelativeUriError,
    ResolveError, ResolveErrorSource, ResolveErrors, ResolvePointerError, SourceConflictError,
    SourceError, UnknownKeyError, UriError, UrlError, UrnError, ValidationError,
};
