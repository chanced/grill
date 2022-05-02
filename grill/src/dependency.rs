pub enum Dependency {
    /// A `String` value is invalid per the JSON Schema spec.
    /// However, http://json-schema.org/draft-04/links has a `String`
    /// value so it is required for backward compatibility.
    #[deprecated = "`String` is not supported by the JSON Schema specification however http://json-schema.org/draft-04/links uses a `String` value so it is required. Use `Strings` instead."]
    String(String),
    Strings(Vec<String>),
    Schema(Box<Schema>),
    HyperSchema(Box<HyperSchema>),
}
