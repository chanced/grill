/// Returns [`&'static Uri`](`Uri`) of Hyper Schema Draft 07.
///
/// # Returns
/// [`&'static Uri`](`Uri`) equal to `"http://json-schema.org/draft-07/hyper-schema#"`
static HYPER_SCHEMA_07_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("http://json-schema.org/draft-07/hyper-schema#").unwrap());

/// [`Uri`] of Schema Draft 2020-12: <https://json-schema.org/draft/2020-12/schema>
static SCHEMA_2020_12_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2020-12/schema").unwrap());

/// [`Uri`] of Hyper Schema Draft 2020-12.
///
/// <https://json-schema.org/draft/2020-12/hyper-schema>
static HYPER_SCHEMA_2020_12_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2020-12/hyper-schema").unwrap());

/// [`Uri`] of Schema Draft 07.
/// # Returns
/// [`&'static Uri`](`Uri`) equal to `"http://json-schema.org/draft-07/schema#"`
static SCHEMA_07_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("http://json-schema.org/draft-07/schema#").unwrap());

/// [`Uri`] of Schema Draft 2019-09.
static SCHEMA_2019_09_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2019-09/schema").unwrap());

/// [`Uri`] of Hyper Schema Draft 2019-09.
static HYPER_SCHEMA_2019_09_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2019-09/hyper-schema").unwrap());

static SCHEMA_07_DIALECT: Lazy<Dialect> = Lazy::new(|| {
    let mut dialect = Dialect {
        id: schema_07_uri().clone(),
        vocabularies: HashMap::new(),
        meta_schemas: HashMap::new(),
    };
    dialect
});

/// Returns [`&'static Uri`](`Uri`) of Hyper Schema Draft 2020-12.
///
/// # Returns
/// [`&'static Uri`](`Uri`) equal to `"https://json-schema.org/draft/2020-12/hyper-schema"`
#[must_use]
pub fn hyper_schema_2020_12_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_2020_12_URI).unwrap_or_else(|| Lazy::force(&HYPER_SCHEMA_2020_12_URI))
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
/// Returns [`&'static Uri`](`Uri`) of Schema Draft 2019-09.
///
/// # Returns
/// [`&'static Uri`](`Uri`) equal to `"https://json-schema.org/draft/2019-09/schema"`
pub fn schema_2019_09_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_2019_09_URI).unwrap_or_else(|| Lazy::force(&SCHEMA_2019_09_URI))
}

/// Returns [`&'static Uri`](`Uri`) of Hyper Schema Draft 2019-09.
///
/// # Returns
/// [`&'static Uri`](`Uri`) equal to `"https://json-schema.org/draft/2019-09/hyper-schema"`.
#[must_use]
pub fn hyper_schema_2019_09_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_2019_09_URI).unwrap_or_else(|| Lazy::force(&HYPER_SCHEMA_2019_09_URI))
}

/// Returns [`&'static Uri`](`Uri`) of Schema Draft 07.
///
/// # Returns
/// [`&'static Uri`](`Uri`) equal to `http://json-schema.org/draft-07/schema#`
#[must_use]
pub fn schema_07_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_07_URI).unwrap_or_else(|| Lazy::force(&SCHEMA_07_URI))
}

/// Returns [`&'static Uri`](`Uri`) of Hyper Schema Draft 07
///
/// # Returns
/// [`&'static Uri`](`Uri`) equal to `"http://json-schema.org/draft-07/hyper-schema#"`.
#[must_use]
pub fn hyper_schema_07_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_07_URI).unwrap_or_else(|| Lazy::force(&HYPER_SCHEMA_07_URI))
}

/// Returns `true` if the given [`Uri`] equals Schema Draft 07.
///
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` equals `"https://json-schema.org/draft/2020-12/schema"`
/// - `true` if the [`Uri`] `meta_schema_id` equals `"http://json-schema.org/draft-07/schema#"`
/// - `false` otherwise
#[must_use]
pub fn is_schema_07_uri(meta_schema_id: &Uri) -> bool {
    let schema_07_uri = schema_07_uri();

    let scheme: &str = match meta_schema_id.scheme() {
        Some(s) => s,
        None => return false,
    };

    if scheme == "http" {
        return meta_schema_id == schema_07_uri;
    }
    scheme == "https"
        && meta_schema_id.host() == schema_07_uri.host()
        && meta_schema_id.path() == schema_07_uri.path()
}

/// Returns `true` if the given [`Uri`] equals Schema Draft 2020-12.
///
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` equals `"https://json-schema.org/draft/2020-12/schema"`
/// - `false` otherwise
#[must_use]
pub fn is_schema_2020_12_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == schema_2020_12_uri()
}

/// Returns `true` if the given [`Uri`] equals Hyper Schema draft
/// 2020-12.
///
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` equals `"https://json-schema.org/draft/2020-12/hyper-schema"`
#[must_use]
pub fn is_hyper_schema_2020_12_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == hyper_schema_2020_12_uri()
}

/// Returns `true` if the given [`Uri`] equals Schema Draft 2019-09.
///
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` equals
///   `"https://json-schema.org/draft/2020-12/schema"`
/// - `false` otherwise
#[must_use]
pub fn is_schema_2019_09_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == schema_2019_09_uri()
}

/// Returns `true` if the given [`Uri`] equals Hyper Schema 2019-09.
///
/// # Returns
/// - `true` if the [`Uri`] `meta_schema_id` equals `"https://json-schema.org/draft/2020-12/hyper-schema"`
#[must_use]
pub fn is_hyper_schema_2019_09_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == hyper_schema_2019_09_uri()
}

/// Returns [`&'static Uri`](`Uri`) of Schema Draft 2020-12.
///
/// # Returns
/// [`&'static Uri`](`Uri`) equal to `""https://json-schema.org/draft/2020-12/schema"`
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn schema_2020_12_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_2020_12_URI).unwrap()
}
