use std::{collections::HashMap, convert::AsRef, io::BufWriter, iter::once};

use camino::{Utf8Path, Utf8PathBuf};
use grill::{json_schema::keyword::pattern, uri::Url, AbsoluteUri, Uri};
use once_cell::sync::Lazy;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{fs::read_files, Error, Sources, Suite};
fn rand_array<const S: usize>() -> [u8; S] {
    todo!()
}
fn x() {
    rand_array::<8>();
}

fn source_uri(base_uri: &AbsoluteUri, path: &Utf8Path) -> String {
    let uri = base_uri
        .resolve(&Uri::parse(path.as_str()).unwrap())
        .unwrap()
        .to_string();
    uri
}

fn gen_source<'x>(
    base_uri: &'x AbsoluteUri,
    files: &'x [(Utf8PathBuf, String)],
) -> impl Iterator<Item = TokenStream> + 'x {
    files
        .iter()
        .map(|(path, content)| (source_uri(base_uri, path), content))
        .map(|(uri, content)| (uri, syn::parse_str::<TokenStream>(content).unwrap()))
        .map(|(uri, content)| {
            quote! {
                (#uri, json!(#content)),
            }
        })
}

fn gen_sources(
    base_uri: &AbsoluteUri,
    ancestry: impl AsRef<Utf8Path>,
    sources: Option<&Sources>,
) -> Result<TokenStream, Error> {
    let Some(sources) = sources else {
        return Ok(TokenStream::new());
    };

    let mut content = read_files([ancestry], &sources.paths)?;
    if let Some(strip_prefix) = &sources.strip_prefix {
        content = content
            .into_iter()
            .map(|(path, content)| {
                let path = path.strip_prefix(strip_prefix).unwrap();
                (path.to_owned(), content)
            })
            .collect();
    }
    let sources = gen_source(base_uri, &content);
    let src = quote! {
        fn sources() -> &'static [(&'static str, Value)] {
            static VALUES: Lazy<Vec<(&'static str, Value)>> = Lazy::new(|| {
                Vec::from(
                    [#(#sources)*]
                )
            });
            &VALUES
        }
    };
    Ok(src)
}

fn gen_imports() -> TokenStream {
    quote! {
        use serde_json::{json, Value};
        use once_cell::sync::Lazy;
    }
}
fn gen_util_fns() -> TokenStream {
    quote! {}
}

fn gen_runner_test_cases_trait<C, V>(
    name: &Utf8Path,
    ancestry: &[&Utf8Path],
    case: C,
) -> Result<TokenStream, Error>
where
    C: IntoIterator<Item = V>,
    V: AsRef<Utf8Path>,
{
    let name = heck::ToPascalCase::to_pascal_case(name.as_str());

    println!("files: {files:?}");
    Ok(quote! {
        pub(super) trait #name {
        }
    })
}

fn gen_runnner_trait<K, V, C>(
    path: &Utf8Path,
    ancestry: &[&Utf8Path],
    tests: C,
) -> Result<TokenStream, Error>
where
    K: AsRef<Utf8Path>,
    V: IntoIterator<Item = K>,
    C: IntoIterator<Item = (K, V)>,
{
    println!("path: {path}");

    for (name, cases) in tests {
        let files = read_files(ancestry.iter().chain(once(&name.as_ref())), cases)?;
        let content = gen_runner_test_cases_trait(name.as_ref(), ancestry, cases)?;
    }
    Ok(quote! {})
}

fn gen_runner_mod(
    ancestry: &[&Utf8Path],
    path: &Utf8Path,
    tests: &HashMap<Utf8PathBuf, Vec<Utf8PathBuf>>,
) -> Result<TokenStream, Error> {
    let c = gen_runnner_trait(path, ancestry, tests)?;

    Ok(quote! {})
}

fn gen_runner(
    root: &Utf8Path,
    tests: &HashMap<Utf8PathBuf, HashMap<Utf8PathBuf, Vec<Utf8PathBuf>>>,
) -> Result<TokenStream, Error> {
    for (path, tests) in tests {
        let content = gen_runner_mod(&[root, path], path, tests)?;
    }
    Ok(quote! {})
}

pub(crate) fn suite(path: &Utf8Path, suite: &Suite) -> Result<String, Error> {
    let Suite {
        sources,
        base_uri,
        tests,
    } = suite;

    let root = Utf8Path::new("tests");
    let path = root.join(path);
    let sources = gen_sources(base_uri, &path, sources.as_ref())?;
    let util_fns = gen_util_fns();
    let runner = gen_runner(&path, tests)?;
    let imports = gen_imports();
    let src = quote!(
        #imports
        #runner
        #sources
        #util_fns

    );
    let src = src.to_string();
    let mut buf = BufWriter::new(Vec::new());
    let (_, filemap, _) =
        rustfmt::format_input(rustfmt::Input::Text(src), &rustfmt_cfg(), Some(&mut buf)).unwrap();
    let content = filemap[0].1.to_string();
    Ok(content)
}
pub(crate) fn rustfmt_cfg() -> rustfmt::config::Config {
    rustfmt::config::Config::default()
}

// fn insert_json(content: String, json: &[String]) -> String {
//     println!("{content}");
//     println!("{:?}", JSON_REGEX.find_iter(&content).collect::<Vec<_>>());
//     String::default()
// }

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{load_cfg, tests::set_test_cwd};
    #[test]
    fn test_suite() {
        set_test_cwd();
        let cfg = load_cfg(Utf8Path::new("grill-test-builder/fixtures/tests.toml")).unwrap();
        for (p, s) in &cfg.suite {
            // let file = syn::parse_file(&s.to_string()).unwrap();
            let content = suite(p, s).unwrap();
            // let content = prettyplease::unparse(&file).to_string();
            let mut output = String::new();
            std::fs::write("tests/src/tmp3.rs", content).unwrap();
        }
    }
}
