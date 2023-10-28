# grill

Interrogate your data with [JSON Schema](https://json-schema.org/) or roll your
own schema language.

## Table of contents

-   [High level features & attractions](#high-level-features--attractions)
-   [Compatibility](#compatibility)
-   [Installation](#installation)
-   [Feature flags](#feature-flags)
-   [Example](#example)
-   [Dependencies](#dependencies)

## High level features & attractions

-   Full support for JSON Schema
    [Draft 2020-12](https://json-schema.org/specification-links#2020-12),
    [Draft 2019-09](<https://json-schema.org/specification-links#draft-2019-09-(formerly-known-as-draft-8)>),
    [Draft 07](https://json-schema.org/specification-links#draft-7), and
    [Draft 04](https://json-schema.org/specification-links#draft-4)
-   Rest easier with immutable schemas.
-   Modify or roll your own `Dialect`.
-   Hassle free source resolution with custom `Resolver` implementations.
-   Easily translate or customize validation error messages.
-   Dial in output noise with 3 levels of granularity per the current [JSON
    Schema (2020-12) recommendation
    structures](https://json-schema.org/draft/2020-12/json-schema-core#name-output-formats).
-   Confidently compare massive to minuscule numbers - they are all parsed as
    `BigRational`s.
-   Conserve bits with `Value`s and `BigRational`s caches.

## Compatability

grill requires `std` and an `async` runtime to support `Resolver`s.

## Installation

From your favorite terminal at your project's root:

```bash
▶ cargo add grill
```

## Feature flags

|  Feature Flag   | Description                                          | Default |
| :-------------: | ---------------------------------------------------- | ------- |
|    `"yaml"`     | Enables YAML `Deserializer`, `add_yaml_deserializer` | No      |
|    `"toml"`     | Enables TOML `Deserializer`, `add_toml_deserializer` | No      |
|    `"http"`     | Adds a simple `reqwest` based HTTP `Resolver`        | No      |
| `"json-schema"` | Provides JSON Schema support                         | Yes     |

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
		.source_owned_value("https://example.com/schema", schema)
		.precompile(["https://example.com/schema"])
		.finish()
		.await
		.unwrap();

	let uri = AbsoluteUri::parse("https://example.com/schema").unwrap();
	let key = interrogator.schema_key_by_uri(&uri).unwrap();

	for (value, expect_valid) in [
		(json!({ "foo": 34 }), true),
		(json!({ "foo": 34.07 }), true),
		(json!({ "foo": "NaN" }), false),
		(json!({}), false),
		(json!(34), false),
	] {
		let o = interrogator.evaluate(key, Structure::Verbose, &value).unwrap();
		assert_eq!(o.is_valid(), expected_valid);
	}
}

```

## Dependencies

**Note**: This list may become stale. Check `Cargo.toml` for the most up to date
account.

| Dependency                                           | Usage                                                                       |     Re-export     | Feature Flag |
| :--------------------------------------------------- | --------------------------------------------------------------------------- | :---------------: | :----------: |
| [anyhow](https://docs.rs/anyhow)                     | User-supplied, opaque errors                                                |        ---        |     ---      |
| [anymap](https://docs.rs/anymap)\*                   | Storage of state, translations                                              |  `grill::anymap`  |     ---      |
| [async-trait](https://docs.rs/async-trait)           | Async trait support for [`Resolver`]                                        |        ---        |     ---      |
| [bitflags](https://docs.rs/bitflags)                 | Bitfields                                                                   |        ---        |     ---      |
| [dyn-clone](https://docs.rs/dyn-clone)               | `Clone` for boxed trait objects                                             |        ---        |     ---      |
| [either](https://docs.rs/either)                     | General purpose left/right sum enum                                         |        ---        |     ---      |
| [erased-serde](https://docs.rs/erased-serde)         | Serde type erasure for `Deserializer`s                                      |        ---        |     ---      |
| [inherent](https://docs.rs/inherent)                 | Trait methods made callable without the trait in scope                      |        ---        |     ---      |
| [jsonptr](https://docs.rs/jsonptr)                   | JSON Pointers                                                               |        ---        |     ---      |
| [num](https://docs.rs/num)                           | Used to support precise comparison and big numbers                          | `grill::big::num` |     ---      |
| [num-rational](https://docs.rs/num)                  | Big rational numbers                                                        |        ---        |     ---      |
| [once_cell](https://docs.rs/once_cell)               | Static values                                                               |        ---        |     ---      |
| [paste](https://docs.rs/paste)                       | declarative macro utility                                                   |        ---        |     ---      |
| [percent-encoding](https://docs.rs/percent-encoding) | URI encoding                                                                |        ---        |     ---      |
| [regex](https://docs.rs/regex)                       | Regular expressions                                                         |        ---        |     ---      |
| [reqwest](https://docs.rs/reqwest)                   | HTTP client for the optional [`HttpResolver`]                               |        ---        |   `"http"`   |
| [serde](https://docs.rs/serde)                       | Serialization and deserialization. Note: `"derive"` and `"rc"` are enabled. |        ---        |     ---      |
| [serde_json](https://docs.rs/serde_json)             | JSON support. Note: `"arbitrary_precision"` is enabled.                     |        ---        |     ---      |
| [serde_yaml](https://docs.rs/serde_yaml)             | YAML support                                                                |        ---        |   `"yaml"`   |
| [slotmap](https://docs.rs/slotmap)                   | Primary data store for `Interrogator`                                       |        ---        |     ---      |
| [strum](https://docs.rs/strum)                       | Derive macros for enum stringification                                      |        ---        |     ---      |
| [thiserror](https://docs.rs/thiserror)               | Error enums                                                                 |        ---        |     ---      |
| [toml](https://docs.rs/toml)                         | TOML support.                                                               |        ---        |   `"toml"`   |
| [url](https://docs.rs/url)                           | URL parsing & representation                                                | `grill::uri::url` |     ---      |
| [urn](https://docs.rs/urn)                           | URN parsing & representation                                                | `grill::uri::urn` |     ---      |

\* `anymap` was copied in for technical reasons.
