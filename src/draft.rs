mod draft_04;
mod draft_07;
mod draft_2019_09;
mod draft_2020_12;

pub use draft_04::{
    hyper_schema_04, hyper_schema_04_uri, is_hyper_schema_04_uri, is_schema_04_uri, schema_04,
    schema_04_uri,
};

pub use draft_07::{
    hyper_schema_07, hyper_schema_07_uri, is_hyper_schema_07_uri, is_schema_07_uri, schema_07,
    schema_07_uri,
};

pub use draft_2019_09::{
    hyper_schema_2019_09, hyper_schema_2019_09_uri, is_hyper_schema_2019_09_uri,
    is_schema_2019_09_uri, schema_2019_09, schema_2019_09_uri,
};

pub use draft_2020_12::{
    hyper_schema_2020_12, hyper_schema_2020_12_uri, is_hyper_schema_2020_12_uri,
    is_schema_2020_12_uri, schema_2020_12, schema_2020_12_uri,
};
