use slotmap::new_key_type;

new_key_type! {
    /// Reference to a [`CompiledSchema`]
    pub struct SchemaRef;
}
