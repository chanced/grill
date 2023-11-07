use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Error};
use grill::{AbsoluteUri, Uri};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    fs::{self, read_files},
    Case, Suite,
};

fn source(uri: AbsoluteUri, schema: String) -> TokenStream {
    let uri = uri.to_string();
    quote!(
        (#uri, json!(#schema)),
    )
}

pub(super) fn sources(suite: &Suite) -> Result<TokenStream, Error> {
    // let files = fs::read_files(suite.sources.iter())?;

    // let mut srcs = Vec::new();
    // for (path, content) in files {
    //     let path = path
    //         .to_str()
    //         .ok_or_else(|| anyhow!("path not valid utf8: \"{path:?}\""))?;

    //     let path = Uri::parse(strip_path_prefix(path, &suite.input)?)?;
    //     let uri = suite.base_uri.resolve(&path)?;
    //     srcs.push(source(uri, content));
    // }

    // let sources = to_sources(&suite.input, &suite.base_uri, files)?;
    // let src = quote! {
    //     pub fn sources() -> &'static HashMap<AbsoluteUri, Value> {
    //         static SOURCES: Lazy<HashMap> = Lazy::new(|| {
    //             HashMap::from([#(#sources)])
    //         });
    //         &SOURCES
    //     }
    // };
    // Ok(src)
    todo!()
}

fn interrogator(name: &str, suite: &Suite) -> TokenStream {
    quote! {
        pub fn interrogator() -> Interrogator {
            static INTERROGATOR: Lazy<Interrogator> = Lazy::new(|| {
                let mut interrogator = super::interrogator();
                interrogator.source_static_values(sources()).unwrap();
                interrogator
            });
            INTERROGATOR.clone()
        }
    }
}

fn test() {}

fn test_case<'a, T, S>(
    prefix: &Path,
    name: &str,
    patterns: T,
    case: &Case,
) -> Result<TokenStream, Error>
where
    T: 'a + IntoIterator<Item = S>,
    S: 'a + Deref<Target = str>,
{
    todo!()
}

fn test_suite(name: &str, suite: &Suite) -> Result<TokenStream, Error> {
    for (name, patterns) in &suite.tests {}
    todo!()
}

pub(crate) fn suite(name: String, suite: Suite) -> Result<Vec<(PathBuf, TokenStream)>, Error> {
    let sources = sources(&suite)?;
    let interrogator = interrogator(&name, &suite);
    let test_suite = test_suite(&name, &suite)?;
    // Ok(quote! {
    //     use std::collections::HashMap;
    //     use once_cell::sync::Lazy;
    //     use grill::{ AbsoluteUri, Interrogator };
    //     #sources
    //     #interrogator
    // })
    todo!()
}
