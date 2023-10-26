# grill

grill your data with [JSON Schema](https://json-schema.org/).

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
-   Upgrade, tweak or roll your own bespoke `Dialect`.
-   Hassle free source resolution with custom `Resolver` implementations.
-   Easily translate or customize validation error messages for your users.
-   Dial in output noise with 3 levels of granularity which conform to the
    current (2020-12) recommendation structures.
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

|                      Feature Flag                      | Usage                                                                       |   Version |     Re-export     | Feature Flag | Dev |
| :----------------------------------------------------: | --------------------------------------------------------------------------- | --------: | :---------------: | :----------: | --- |
|           [`anyhow`](https://docs.rs/anyhow)           | User-supplied, opaque errors                                                |  `1.0.75` |        ---        |     ---      | No  |
|          [`anymap`](https://docs.rs/anymap)\*          | Storage of state, translations                                              |       --- |  `grill::anymap`  |     ---      | No  |
|      [`async-trait`](https://docs.rs/async-trait)      | Async trait support for [`Resolver`]                                        |   `0.1.0` |        ---        |     ---      | No  |
|         [`bitflags`](https://docs.rs/bitflags)         | Bitfields                                                                   |   `2.4.1` |        ---        |     ---      | No  |
|        [`dyn-clone`](https://docs.rs/dyn-clone)        | `Clone` for boxed trait objects                                             |  `1.0.11` |        ---        |     ---      | No  |
|           [`either`](https://docs.rs/either)           | General purpose left/right sum enum                                         |   `1.8.1` |        ---        |     ---      | No  |
|     [`erased-serde`](https://docs.rs/erased-serde)     | Serde type erasure for `Deserializer`s                                      |  `0.3.25` |        ---        |     ---      | No  |
|         [`inherent`](https://docs.rs/inherent)         | Trait methods made callable without the trait in scope                      |  `1.0.10` |        ---        |     ---      | No  |
|          [`jsonptr`](https://docs.rs/jsonptr)          | JSON Pointers                                                               |   `0.4.4` |        ---        |     ---      | No  |
|              [`num`](https://docs.rs/num)              | Used to support precise comparison and big numbers                          |   `0.4.0` | `grill::big::num` |     ---      | No  |
|         [`num-rational`](https://docs.rs/num)          | Big rational numbers                                                        |   `0.4.1` |        ---        |     ---      | No  |
|        [`once_cell`](https://docs.rs/once_cell)        | Static values                                                               |  `1.10.0` |        ---        |     ---      | No  |
|            [`paste`](https://docs.rs/paste)            | declarative macro utility                                                   |  `1.0.14` |        ---        |     ---      | No  |
| [`percent-encoding`](https://docs.rs/percent-encoding) | URI encoding                                                                |   `2.3.0` |        ---        |     ---      | No  |
|            [`regex`](https://docs.rs/regex)            | Regular expressions                                                         |   `1.5.4` |        ---        |     ---      | No  |
|          [`reqwest`](https://docs.rs/reqwest)          | HTTP client for the optional [`HttpResolver`]                               |  `0.11.0` |        ---        |   `"http"`   | No  |
|            [`serde`](https://docs.rs/serde)            | Serialization and deserialization. Note: `"derive"` and `"rc"` are enabled. | `1.0.163` |        ---        |     ---      | No  |
|       [`serde_json`](https://docs.rs/serde_json)       | JSON support. Note: `"arbitrary_precision"` is enabled.                     | `1.0.107` |        ---        |     ---      | No  |
|       [`serde_yaml`](https://docs.rs/serde_yaml)       | YAML support                                                                |  `0.9.21` |        ---        |   `"yaml"`   | No  |
|          [`slotmap`](https://docs.rs/slotmap)          | Primary data store for `Interrogator`                                       |   `1.0.1` |        ---        |     ---      | No  |
|            [`strum`](https://docs.rs/strum)            | Derive macros for enum stringification                                      |  `0.24.0` |        ---        |     ---      | No  |
|        [`thiserror`](https://docs.rs/thiserror)        | Error enums                                                                 |  `1.0.25` |        ---        |     ---      | No  |
|             [`toml`](https://docs.rs/toml)             | TOML support.                                                               |   `0.8.2` |        ---        |   `"toml"`   | No  |
|              [`url`](https://docs.rs/url)              | URL parsing & representation                                                |   `2.2.2` | `grill::uri::url` |     ---      | No  |
|              [`urn`](https://docs.rs/urn)              | URN parsing & representation                                                |   `0.7.0` | `grill::uri::urn` |     ---      | No  |
|          [`similar`](https://docs.rs/similar)          | Test output comparison                                                      |   `2.2.1` |        ---        |     ---      | Yes |
|            [`tokio`](https://docs.rs/tokio)            | Running `async` tests and examples                                          |  `1.28.0` |        ---        |     ---      | Yes |
|          [`mockall`](https://docs.rs/mockall)          | Mock generator                                                              |  `0.11.4` |        ---        |     ---      | Yes |

\* `anymap` was copied in for technical reasons.
