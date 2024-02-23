/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             CompileError                              ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An error occurred while compiling a schema.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(Ctx)), module)]
pub enum CompileError {
    /// The schema failed evaluation, represented by the failed [`Output`].
    #[snafu(display("schema failed evaluation: {source}"))]
    SchemaInvalid {
        source: Output<'static>,
        backtrace: Backtrace,
    },

    /// Failed to identify a schema
    #[snafu(transparent)]
    SchemaIdentificationFailed {
        #[snafu(backtrace)]
        source: IdentifyError,
    },

    /// The `$schema` is not known to the [`Interrogator`](crate::Interrogator).
    #[snafu(display("metaschema dialect not found: {metaschema_id}"))]
    DialectNotKnown {
        #[snafu(backtrace)]
        /// The schema's [`Dialect`] is not registered with the
        /// [`Interrogator`](crate::Interrogator).
        metaschema_id: String,
    },

    /// Failed to parse a [`Uri`] or
    /// [`AbsoluteUri`](`crate::uri::AbsoluteUri`)
    #[snafu(transparent)]
    FailedToParseUri {
        #[snafu(backtrace)]
        source: UriError,
    },

    /// Failed to resolve or deserialize a source
    #[snafu(transparent)]
    FailedToSource {
        #[snafu(backtrace)]
        source: SourceError,
    },

    #[snafu(transparent)]
    FailedToEvaluateSchema {
        #[snafu(backtrace)]
        source: EvaluateError,
    },

    /// If a [`Schema`] does not have an identifier, then the first [`AbsoluteUri`]
    /// returned from [`Dialect::locate`](`crate::schema::Dialect`) must have the
    /// schema's path as a JSON [`Pointer`].
    #[snafu(display("expected schema URI to contain path; found {uri}"))]
    LocatedUriMalformed {
        /// The [`MalformedPointerError`] which occurred.
        source: MalformedPointerError,
        /// The [`AbsoluteUri`] which was returned from
        uri: AbsoluteUri,
    },

    /// A [`Schema`] contains a cyclic dependency.
    #[snafu(visibility(pub), context(suffix(Ctx)), module)]
    CyclicGraph {
        /// The [`AbsoluteUri`] of the schema which, through transitive
        /// dependencies, creates a cycle.
        from: AbsoluteUri,
        /// The [`AbsoluteUri`] of the schema which is the target of the cycle.
        to: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// Failed to link sources
    #[snafu(display("failed to create source link: {source}"))]
    FailedToLinkSource {
        #[snafu(backtrace)]
        source: LinkError,
    },

    /// Could not locate an anchor referenced in a schema
    /// An unknown anchor (non-pointer fragment of a URI) was encountered
    #[snafu(display("unknown anchor: \"{anchor}\" in URI \"{uri}\""))]
    UnknownAnchor {
        /// The anchor which was not found.
        anchor: String,
        /// The URI of the keyword which referenced the anchor.
        uri: AbsoluteUri,
    },

    /// Failed to parse an anchor field
    #[snafu(transparent)]
    FailedToParseAnchor {
        #[snafu(backtrace)]
        source: AnchorError,
    },

    /// Failed to find a schema with the given uri
    #[snafu(display("schema not found: \"{uri}\""))]
    SchemaNotFound {
        uri: AbsoluteUri,
        backtrace: Backtrace,
    },

    /// Failed to parse a number
    #[snafu(transparent)]
    FailedToParseNumber {
        #[snafu(backtrace)]
        source: NumberError,
    },

    /// Failed to parse json pointer path
    #[snafu(transparent)]
    FailedToParsePointer {
        source: MalformedPointerError,
        backtrace: Backtrace,
    },

    /// A keyword encountered a value type which was not expected
    /// and was not caught by the schema
    #[snafu(transparent)]
    InvalidType {
        #[snafu(backtrace)]
        source: InvalidTypeError,
    },

    /// A keyword encountered a value which was not expected
    #[snafu(display("unexpected value; expected {expected} found {value:?}"))]
    UnexpectedValue {
        /// A description of the expected value
        expected: &'static str,
        /// The actual value.
        value: Box<Value>,
        backtrace: Backtrace,
    },

    /// An error occurred while parsing a ref field (e.g. `"$ref"`,
    /// `"$recursiveRef"`, `"$recursiveAnchor"`)
    #[snafu(transparent)]
    RefError {
        #[snafu(backtrace)]
        source: RefError,
    },

    /// A regular expression failed to parse
    #[snafu(display("failed to parse regular expression: {source}"))]
    FailedToCompileRegex {
        source: regex::Error,
        backtrace: Backtrace,
        pattern: String,
    },

    #[snafu(display("length of uri exceeds maximum size of 4GB after setting fragment"))]
    UriFragmentOverflow {
        uri: AbsoluteUri,
        fragment: String,
        backtrace: Backtrace,
    },
}
impl From<MalformedPointerError> for CompileError {
    fn from(err: MalformedPointerError) -> Self {
        Self::FailedToParsePointer(err.into())
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               BuildError                              ║
║                               ¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Various errors that can occur while building an [`Interrogator`](crate::Interrogator).
#[derive(Debug, Snafu)]
#[snafu(visibility(pub), context(suffix(Ctx)), module)]
pub enum BuildError<C: CompileError> {
    #[snafu(transparent)]
    /// A [`Schema`](crate::schema::Schema) failed to compile.
    FailedToCompile {
        #[snafu(backtrace)]
        source: C,
    },

    #[snafu(transparent)]
    /// An issue with [`Dialect`]s occurred.
    FailedToCreateDialects {
        #[snafu(backtrace)]
        source: DialectsError,
    },

    #[snafu(transparent)]
    /// An error occurred while adding, resolving, or deserializing a
    /// [`Source`](crate::source::Source).
    FailedToSource {
        #[snafu(backtrace)]
        source: SourceError,
    },

    /// Failed to parse a number
    #[snafu(transparent)]
    FailedToParseNumber {
        #[snafu(backtrace)]
        source: NumberError,
    },
}
