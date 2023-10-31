use std::ops::ControlFlow;

use crate::schema::Reference;
use crate::Schema;
/// This trait is used to walk a [`Schema`](grill_core::schema::Schema), its
/// [`Reference`]s, [`Anchor`]s, and [`Keyword`]s.
#[allow(unused_variables)]
#[allow(clippy::needless_lifetimes)]
pub trait Visitor<'i> {
    /// Error type returned by the `Visitor`
    type Error;

    /// Visits a [`Schema`]
    ///
    /// A return value of `Ok(None)` indicates that the visitor should not
    /// traverse the `Schema`.
    fn visit_schema(&mut self, schema: Schema<'i>) -> Result<Option<&mut Self>, Self::Error> {
        Ok(Some(self))
    }
    /// Visits a [`Reference`] of a [`Schema`].
    ///
    /// A return value of `Ok(None)` indicates that the visitor should not follow
    /// the reference
    fn visit_reference(
        &mut self,
        reference: &'i Reference,
    ) -> Result<Option<&mut Self>, Self::Error> {
        Ok(Some(self))
    }
}

struct X;

impl<'i> Visitor<'i> for X {
    type Error = anyhow::Error;

    fn visit_schema(&mut self, schema: Schema<'i>) -> Result<Option<&mut Self>, Self::Error> {
        anyhow::bail!("schema")
    }

    fn visit_reference(
        &mut self,
        reference: &'i Reference,
    ) -> Result<Option<&mut Self>, Self::Error> {
        Ok(Some(self))
    }
}
