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
-   Dial in output noise with 4 levels of granularity per the current [JSON
    Schema (2020-12) recommendation
    structures](https://json-schema.org/draft/2020-12/json-schema-core#name-output-formats).
-   Confidently compare massive to minuscule numbers - they are all parsed as
    `BigRational`s.
-   Conserve bits with `Value`s and `BigRational`s caches.

## Compatibility

grill requires `std` and an `async` runtime to support `Resolver`s.

## Installation

From your favorite terminal at your project's root:

```bash
▶ cargo add grill
```
