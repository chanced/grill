use crate::{handler::SyncHandler};
use std::{error::Error};

pub fn sync_handler<F>(
    _schema: serde_json::Value,
    _handler: impl 'static + SyncHandler,
    _f: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&dyn SyncHandler, &mut crate::Scope) -> Result<(), Box<dyn Error>>,
{
    todo!()
    // let mut state = crate::State::new();
    // let mut scope = crate::Scope::new(Location::default(), &mut state);
    // let mut numbers = HashMap::new();
    // let mut subschemas = HashMap::new();
    // let schema: Schema = serde_json::from_value(schema)?;

    // let mut compile = Compile::new(Location::default(), &mut subschemas, &mut numbers);
    // let process = handler.compile(&mut compile, &schema)?;
    // let schemas = HashMap::new();
    // let handlers = [Handler::Sync(Box::new(handler))];

    // let compiled_schema = CompiledSchema::new(
    //     String::default(),
    //     String::default(),
    //     &numbers,
    //     &schemas,
    //     &handlers,
    //     &schema,
    // );
    // let h = handlers[0].as_sync().unwrap();
    // f(h.as_ref(), &mut scope, &compiled_schema)
}
