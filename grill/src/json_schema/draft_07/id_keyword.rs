use crate::{keyword::Keyword, output::Output};

#[derive(Debug, Clone, Default)]
pub struct IdKeyword;

impl Keyword for IdKeyword {
    fn compile<'i>(
        &mut self,
        compile: &mut crate::keyword::Compile<'i>,
        schema: crate::Schema<'i>,
    ) -> Result<bool, crate::error::CompileError> {
        todo!()
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut crate::keyword::Context,
        schema: &'v serde_json::Value,
        structure: crate::Structure,
    ) -> Result<Option<Output<'v>>, crate::error::EvaluateError> {
        todo!()
    }

    fn subschemas(
        &self,
        schema: &serde_json::Value,
    ) -> Result<Vec<jsonptr::Pointer>, crate::error::Unimplemented> {
        let v = vec![String::new()];
        let r = v.as_slice();
        let x: Vec<String> = r.into();

        Err(crate::error::Unimplemented)
    }

    fn anchors(
        &self,
        schema: &serde_json::Value,
    ) -> Result<
        Result<Vec<crate::schema::Anchor>, crate::error::AnchorError>,
        crate::error::Unimplemented,
    > {
        Err(crate::error::Unimplemented)
    }

    fn identify(
        &self,
        schema: &serde_json::Value,
    ) -> Result<
        Result<Option<crate::schema::Identifier>, crate::error::IdentifyError>,
        crate::error::Unimplemented,
    > {
        Err(crate::error::Unimplemented)
    }

    fn dialect(
        &self,
        schema: &serde_json::Value,
    ) -> Result<
        Result<Option<crate::AbsoluteUri>, crate::error::UriError>,
        crate::error::Unimplemented,
    > {
        Err(crate::error::Unimplemented)
    }

    fn is_pertinent_to(
        &self,
        schema: &serde_json::Value,
    ) -> Result<bool, crate::error::Unimplemented> {
        Err(crate::error::Unimplemented)
    }

    fn references(
        &self,
        schema: &serde_json::Value,
    ) -> Result<
        Result<Vec<crate::schema::Reference>, crate::error::UriError>,
        crate::error::Unimplemented,
    > {
        Err(crate::error::Unimplemented)
    }
}
