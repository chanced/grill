use once_cell::sync::Lazy;
use uniresid::AbsoluteUri;

/// The [AbsoluteUri] of Json Schema Draft 04.
pub static DRAFT_04_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::parse("http://json-schema.org/draft-04/schema#").unwrap());

/// The [AbsoluteUri] of Json Schema Draft 07.
pub static DRAFT_07_URI: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::parse("http://json-schema.org/draft-07/schema#").unwrap());

/// The [AbsoluteUri] of Json Schema Draft 2019-09.
pub static DRAFT_2019_09: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::parse("https://json-schema.org/draft/2019-09/schema").unwrap());

/// The [AbsoluteUri] of Json Schema Draft 2020-12.
pub static DRAFT_2020_12: Lazy<AbsoluteUri> =
    Lazy::new(|| AbsoluteUri::parse("https://json-schema.org/draft/2020-12/schema").unwrap());
