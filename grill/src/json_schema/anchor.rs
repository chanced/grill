#[derive(Debug, Clone)]
pub enum Anchor<'v> {
    Recursive,
    Dynamic(&'v str),
    Static(&'v str),
}
