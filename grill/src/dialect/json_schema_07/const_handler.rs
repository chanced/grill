use crate::{handler::SyncHandler, Schema};

#[derive(Default, Clone, Debug)]
pub struct ConstHandler {
    pub expected: Option<serde_json::Value>,
}

impl SyncHandler for ConstHandler {
    fn setup<'s>(
        &mut self,
        compiler: &mut crate::Compiler<'s>,
        schema: &'s Schema,
    ) -> Result<bool, crate::error::SetupError> {
        match schema {
            Schema::Object(obj) if obj.constant.is_some() => {
                self.expected = obj.constant.clone();
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn evaluate<'v>(
        &self,
        scope: &mut crate::Scope,
        value: &'v serde_json::Value,
        output_structure: crate::Structure,
    ) -> Result<Option<crate::output::Annotation<'v>>, Box<dyn snafu::Error>> {
        todo!()
    }
}

pub struct ConstInvalid<'v> {
    pub expected: serde_json::Value,
    pub actual: &'v serde_json::Value,
}
