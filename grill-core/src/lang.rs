pub trait Lang {
    type Keyword: super::keyword::Keyword<Context = Self::Context>;
    type Output: serde::Serialize + serde::de::DeserializeOwned;
    type Context;
    type Compile;
    type Error;
    type Translator;

    /// Creates a new context for the given `params`.
    fn new_context(&mut self, ctx: NewContext) -> Result<Self::Context, Self::Error>;

    fn new_compile(&mut self, ctx: Self::Context) -> Self::Compile;
}
pub struct NewContext<'i> {
    pub structure: Structure,
    pub eval_numbers: &'i mut Numbers,
    pub global_numbers: &'i Numbers,
    pub schemas: &'i mut Schemas,
    pub sources: &'i Sources,
    pub absolute_keyword_location: &'i AbsoluteUri,
    pub keyword_location: Pointer,
    pub instance_location: Pointer,
}
