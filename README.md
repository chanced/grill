# grill

grill is a [JSON Schema](https://json-schema.org/) implementation that focuses
on modularity and configurability.

## Table of contents

-   [High level features](#high-level-features)
-   [Compatibility](#compatibility)

## High level features

-   Has full support for JSON Schema
    [Draft 2020-12](https://json-schema.org/specification-links#2020-12),
    [Draft 2019-09](<https://json-schema.org/specification-links#draft-2019-09-(formerly-known-as-draft-8)>),
    [Draft 07](https://json-schema.org/specification-links#draft-7), and
    [Draft 04](https://json-schema.org/specification-links#draft-4)
-   Upgrade, tweak or roll your own bespoke `Dialect`.
-   Source resolution can be made automatic with `Resolver` implementations.
-   Error messages can be easily changed or translated with a `Translator`.
-   Output follows the current (2020-12) recommended output structure.

## Compatability

grill requires `std` and an `async` runtime to support `Resolver`s.

## Example

```rust
use grill::{ Interrogator, Structure, AbsoluteUri, JsonSchema };
use serde_json::json;
#[tokio::main]
async fn main() {
	let schema = json!({
		"$id": "https://example.com/schema",
		"type" "object",
		"properties": { "foo": { "type": "number" } },
		"required": ["foo"]
	});
	let interrogator = Interrogator::build()
		.json_schema_2020_12()
		.source_owned_value("https://example.com/schema", ).
}

```
