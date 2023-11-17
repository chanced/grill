use std::{collections::HashMap, convert::AsRef, fs::File, io::Write};

use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use grill::{AbsoluteUri, Uri};
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;
use serde_json::value::RawValue;
use snafu::ResultExt;

use crate::{
    fs::{self, Path},
    Error, Sources, Suite, SynSnafu,
};

pub(crate) fn suite(
    ancestry: &[&Utf8Path],
    suite: &Suite,
) -> Result<HashMap<Utf8PathBuf, String>, Error> {
    let Suite {
        sources,
        base_uri,
        tests,
    } = suite;
    let suite_name = ancestry
        .get(1)
        .expect("ancestry should have at least 2 elements");
    println!("generating suite: {suite_name}");
    let suite_name = Name::new(suite_name.as_str());
    let suite_path = Utf8PathBuf::from(suite_name.snake_ident.to_string());
    let ancestry_path = Utf8PathBuf::from_iter(ancestry);
    let file_paths = find_test_files(&ancestry_path, tests)?;
    let sets = file_paths
        .into_iter()
        .map(|(name, cases)| TestSet::new(name, cases))
        .collect::<Vec<_>>();
    let mut generated_tests = gen_tests(&suite_name, &sets, base_uri)?;
    let mut files = HashMap::with_capacity(generated_tests.len());
    for (path, tokens) in generated_tests.drain() {
        files.insert(path, format_src(tokens.to_string())?);
    }
    let sources = gen_sources(base_uri, &ancestry_path, sources.as_ref())?;
    let has_sources = sources.is_some();
    let sources = sources
        .map(|sources| format_src(sources.to_string()))
        .transpose()?;
    if let Some(sources) = sources {
        files.insert(suite_path.join("sources.rs"), sources);
    }

    let (mod_path, mod_file) = gen_root_mod(&suite_name, sets, has_sources);
    files.insert(mod_path, format_src(mod_file.to_string())?);
    Ok(files)
}

fn gen_header(_suite: &Name, path: &Utf8Path, ancestry: &[&Name]) -> TokenStream {
    if ancestry.len() <= 1 && path.file_name() == Some("mod.rs") {
        // bailing on top level as that header is generated in `gen_tests_mods`
        return quote!();
    }
    let path = path.with_extension("");
    let mut setup_fn = Vec::new();
    if ancestry.len() > 1 {
        setup_fn.push(
            ancestry
                .last()
                .map(|ancestor| &ancestor.snake_ident)
                .unwrap(),
        );
    }

    let mut path_comps = path
        .components()
        .filter(|comp| matches!(comp, Utf8Component::Normal(_)))
        .skip(2)
        .collect::<Vec<_>>();
    let mod_comp = Utf8Component::Normal("mod");
    if path_comps.last() == Some(&mod_comp) {
        path_comps.pop();
    }
    let path = Utf8PathBuf::from_iter(path_comps);
    let setup_fn = format_ident!("setup_{}", path.as_str().to_snake_case());

    quote!(
        use super::*;
        fn interrogator() -> Result<Interrogator, &'static BuildError> {
            let mut interrogator = super::interrogator();
            if let Ok(interrogator) = interrogator.as_mut() {
                crate::Harness.#setup_fn(interrogator)
            }
            interrogator
        }
    )
}

fn gen_root_mod(suite: &Name, sets: Vec<TestSet>, has_sources: bool) -> (Utf8PathBuf, TokenStream) {
    let harness = gen_harness(&sets);
    let mod_path = Utf8PathBuf::from(format!("{}/mod.rs", suite.snake_ident));
    let mut mods = sets
        .iter()
        .map(|s| {
            let name = &s.name.snake_ident;
            quote!(mod #name;)
        })
        .collect::<Vec<_>>();
    if has_sources {
        mods.push(quote!(
            mod sources;
        ));
    }
    (
        mod_path,
        quote! {
            #(#mods)*
            #harness
        },
    )
}

fn build_fn() -> TokenStream {
    quote! {
        fn build(build: grill::Build) -> Result<grill::Interrogator, grill::error::BuildError> {
            block_on(build.source_static_values(sources::sources()).finish())
        }
    }
}

fn gen_harness(sets: &[TestSet]) -> TokenStream {
    let assoc_types = sets.iter().map(TestSet::harness_assoc_types);
    let accessors = sets.iter().map(TestSet::harness_accessors);
    let traits = sets.iter().map(TestSet::harness_trait);
    let build_fn = build_fn();
    quote! {
        use grill::{ Interrogator};
        use futures::executor::block_on;

        pub trait Harness: Copy {
            #(#assoc_types)*
            #(#accessors)*
        }
        #(#traits)*
        #build_fn

    }
}

fn gen_tests(
    suite: &Name,
    sets: &[TestSet],
    base_uri: &AbsoluteUri,
) -> Result<HashMap<Utf8PathBuf, TokenStream>, Error> {
    let mut tests = HashMap::new();

    for set in sets {
        let path = Utf8PathBuf::from(suite.snake_ident.to_string());
        let mut output = set.tests(suite, base_uri, Vec::new())?;
        for (k, v) in output.drain() {
            tests.insert(path.join(k), v);
        }
    }
    Ok(tests
        .into_iter()
        .map(|(path, (ancestry, src))| {
            let mut header = gen_header(suite, &path, &ancestry);
            header.extend(src);
            (path, header)
        })
        .collect())
}

fn format_src(src: String) -> Result<String, Error> {
    use std::process::{Command, Stdio};
    let mut cmd = Command::new("rustfmt")
        .arg("--edition=2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to exceute rustfmt");

    cmd.stdin
        .as_mut()
        .unwrap()
        .write_all(src.as_bytes())
        .unwrap();
    let output = cmd.wait_with_output().unwrap();

    if !output.status.success() {
        return Err(Error::RustFmt {
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn source_uri(base_uri: &AbsoluteUri, path: &Utf8Path) -> String {
    let uri = base_uri
        .resolve(&Uri::parse(path.as_str()).expect("path should parse as a URI"))
        .expect("URI should resolve")
        .to_string();
    uri
}

fn gen_sources(
    base_uri: &AbsoluteUri,
    ancestry: impl AsRef<Utf8Path>,
    sources: Option<&Sources>,
) -> Result<Option<TokenStream>, Error> {
    let Some(sources) = sources else {
        return Ok(None);
    };
    let mut array_entries = Vec::new();
    let mut functions = Vec::new();

    for file in fs::open([ancestry], &sources.paths) {
        let mut file = file?;
        let content = file.read_to_string()?;
        let content: TokenStream =
            syn::parse_str(&content).with_context(|_| SynSnafu { content })?;
        let mut path = &*file.path.rel;
        if let Some(prefix) = &sources.strip_prefix {
            path = path.strip_prefix(prefix).expect("strip prefix of path");
        }
        let uri = source_uri(base_uri, path);

        let function_name = format_ident!("source_{}", path.as_str().to_snake_case());
        functions.push(quote! {
            fn #function_name() -> (&'static str, Value) {
                (#uri, json!(#content))
            }
        });
        let content = quote!(#function_name(),);

        array_entries.push(content);
    }
    let len = syn::Index::from(array_entries.len());
    let src = quote! {
        use serde_json::{json, Value};
        use once_cell::sync::Lazy;

        pub(super) fn sources() -> impl Iterator<Item = (&'static str, &'static Value)> {
            static VALUES: Lazy<[(&'static str, Value); #len]> = Lazy::new(|| {
                [#(#array_entries)*]
            });
            VALUES.iter().map(|(uri, schema)| (*uri, schema))
        }

        #(#functions)*
    };
    let src = format_src(src.to_string())?;
    Ok(Some(syn::parse_str(&src).unwrap()))
}

fn find_test_files(
    ancestry: &Utf8Path,
    suite_tests: &HashMap<Utf8PathBuf, HashMap<Utf8PathBuf, Vec<Utf8PathBuf>>>,
) -> Result<HashMap<Utf8PathBuf, Vec<Path>>, Error> {
    let mut test_suite_files: HashMap<Utf8PathBuf, Vec<Path>> = HashMap::new();
    for (test_root, cases) in suite_tests {
        for (name, globs) in cases {
            let files =
                fs::find([ancestry, test_root, name], globs).collect::<Result<Vec<_>, Error>>()?;
            test_suite_files
                .entry(name.clone())
                .or_default()
                .extend(files);
        }
    }
    Ok(test_suite_files)
}

#[derive(Debug)]
struct Name {
    string: String,
    snake_ident: Ident,
    pascal_ident: Ident,
}

impl Name {
    const RESERVED: [&str; 52] = [
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "Self", "self", "static", "struct", "super", "trait",
        "true", "type", "union", "unsafe", "use", "where", "while", "abstract", "become", "box",
        "do", "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
    ];
    pub fn new(name: &str) -> Self {
        let snake = Self::snake(name);
        Self {
            string: name.to_string(),
            snake_ident: format_ident!("{}", snake),
            pascal_ident: format_ident!("{}", name.to_pascal_case()),
        }
    }

    fn snake(name: &str) -> String {
        let mut name = name.to_snake_case();
        if Self::RESERVED.contains(&&*name) {
            name.push('_');
        }
        name
    }
}

#[derive(Debug)]
struct TestSet {
    name: Name,
    cases: Vec<Path>,
    sets: Vec<TestSet>,
}

impl TestSet {
    /// eg: `draft2020-12`, [tree.json, optional/formats/date.json ...]
    fn new<N>(name: N, paths: Vec<Path>) -> Self
    where
        N: AsRef<Utf8Path>,
    {
        let (sets, cases) = Self::parts(paths);
        Self {
            name: Name::new(name.as_ref().as_str()),
            sets,
            cases,
        }
    }

    fn tests<'t>(
        &'t self,
        suite: &'t Name,
        base_uri: &AbsoluteUri,
        mut ancestry: Vec<&'t Name>,
    ) -> Result<HashMap<Utf8PathBuf, (Vec<&'t Name>, TokenStream)>, Error> {
        let _ = base_uri;
        ancestry.push(&self.name);
        let mut mods = self
            .sets
            .iter()
            .map(|set| &set.name.snake_ident)
            .map(|name| quote!(mod #name;))
            .collect::<Vec<_>>();

        let mut files = HashMap::new();
        let set_path = Utf8PathBuf::from(self.name.snake_ident.to_string());
        for path in &self.cases {
            let cases = open_test_cases(path)?;
            let src: TokenStream = cases
                .iter()
                .enumerate()
                .map(|(i, case)| case.generate(i, base_uri, suite, path, &ancestry))
                .collect();
            let mut path = set_path.join(&path.rel);
            let mut filename = Utf8PathBuf::from(path.file_name().unwrap());
            path.pop();
            filename.set_extension("");
            let mod_name = Name::new(filename.as_str());
            let name = &mod_name.snake_ident;
            mods.push(quote! { mod #name; });
            let mut filename = Utf8PathBuf::from(name.to_string());
            filename.set_extension("rs");
            files.insert(path.join(filename), (ancestry.clone(), src));
        }

        let rel_path = Utf8PathBuf::from(self.name.snake_ident.to_string());

        let mod_content = gen_tests_mods(&ancestry, &mods);
        files.insert(rel_path.join("mod.rs"), (ancestry.clone(), mod_content));

        for set in &self.sets {
            let mut set_files = set.tests(suite, base_uri, ancestry.clone())?;
            for (path, src) in set_files.drain() {
                files.insert(rel_path.join(path), src);
            }
        }
        Ok(files)
    }

    fn harness_accessors(&self) -> TokenStream {
        let name_pascal = &self.name.pascal_ident;
        let name_snake = &self.name.snake_ident;
        quote! {
            fn #name_snake(&self) -> Self::#name_pascal;
        }
    }
    fn harness_assoc_types(&self) -> TokenStream {
        let name = &self.name.pascal_ident;
        quote! {
            type #name: #name;
        }
    }
    fn descendant_paths(&self) -> Vec<Utf8PathBuf> {
        let mut paths = Vec::new();
        let mut q = vec![(Utf8PathBuf::new(), self)];
        while let Some((rel, set)) = q.pop() {
            if !rel.as_str().is_empty() {
                paths.push(rel.clone());
            }
            for set in &set.sets {
                q.push((rel.join(&set.name.string), set));
            }
            paths.extend(set.cases.iter().map(|case| rel.join(&case.rel)));
        }
        paths
    }
    fn harness_trait_methods(&self) -> TokenStream {
        self.descendant_paths()
            .into_iter()
            .map(|mut path| {
                path.set_extension("");
                let name = format_ident!("setup_{}", path.as_str().to_snake_case());
                quote!(
                    fn #name(&self, interrogator: &mut Interrogator) {}
                )
            })
            .collect()
    }

    fn parts(paths: Vec<Path>) -> (Vec<TestSet>, Vec<Path>) {
        let mut sets: HashMap<String, Vec<Path>> = HashMap::new();
        let mut cases = Vec::new();
        for mut path in paths {
            let Some((name, rel)) = Self::next_component(&path) else {
                continue;
            };
            if rel.as_str().is_empty() {
                cases.push(path);
            } else {
                path.rel = rel;
                sets.entry(name.to_string()).or_default().push(path);
            }
        }

        let sets = sets
            .into_iter()
            .map(|(name, paths)| Self::new(name, paths))
            .collect();

        (sets, cases)
    }

    fn next_component(path: &Path) -> Option<(String, Utf8PathBuf)> {
        if !path.is_file() {
            return None;
        }
        let mut comps = path.rel.components();
        while let Some(comp) = comps.next() {
            let Utf8Component::Normal(comp) = comp else {
                continue;
            };
            let rel = comps.as_path().to_path_buf();
            return Some((comp.to_string(), rel));
        }
        None
    }

    fn harness_trait(&self) -> TokenStream {
        let name = &self.name.pascal_ident;
        let methods = self.harness_trait_methods();
        quote! {
            #[allow(unused_variables)]
            pub trait #name {
                fn build(&self) -> grill::Build;
                #methods
            }
        }
    }
}

// use crate::json_schema_test_suite::{Draft202012, Harness};
// async fn interrogator() -> Result<Interrogator, &'static BuildError> {
//     static INTERROGATOR: OnceLock<Result<Interrogator, BuildError>> = OnceLock::new();
//     INTERROGATOR.get_or_init(|| {
//         futures::executor::block_on(crate::json_schema_test_suite::build(
//             crate::Harness.draft2020_12().interrogator(),
//         ))
//     });
//     todo!()
// }

fn gen_tests_mods(ancestry: &[&Name], mods: &[TokenStream]) -> TokenStream {
    let method = &ancestry[0].snake_ident;
    let interrogator = if ancestry.len() == 1 {
        quote! {
            fn interrogator() -> Result<Interrogator, &'static BuildError> {
                use std::sync::OnceLock;
                use crate::Harness;
                static INTERROGATOR: OnceLock<Result<Interrogator, BuildError>> = OnceLock::new();
                INTERROGATOR
                    .get_or_init(|| build(Harness.#method().build()))
                    .as_ref()
                    .map(Clone::clone)
            }
        }
    } else {
        quote!()
    };

    quote! {
        use grill::{Interrogator, error::BuildError};
        use super::*;

        #interrogator
        #(#mods)*
    }
}

#[derive(Debug, Default, Deserialize)]
struct TestCase {
    description: String,
    schema: Box<RawValue>,
    tests: Vec<Test>,
}

fn open_test_cases(path: &Path) -> Result<Vec<TestCase>, Error> {
    let file = File::open(&path.full).map_err(|source| Error::Io {
        pattern: Some(path.pattern.clone()),
        path: path.full.clone(),
        source,
    })?;

    let test_case = serde_json::from_reader(file).map_err(|err| Error::Json {
        path: path.full.clone(),
        source: err,
    })?;
    Ok(test_case)
}

impl TestCase {
    fn generate(
        &self,
        i: usize,
        uri: &AbsoluteUri,
        _suite: &Name,
        path: &Path,
        _ancestry: &[&Name],
    ) -> TokenStream {
        let description = &self.description;

        let name = format_ident!("{}_{i}", Name::snake(description));
        let schema = self.schema.to_string();
        // this does not work because hashtags are being intercepted as placements
        let schema = format!(r###"r##"{schema}"##"###)
            .parse::<proc_macro2::Literal>()
            .unwrap();

        // println!("{schema}");
        let tests = self
            .tests
            .iter()
            .enumerate()
            .map(|(i, test)| test.generate(i));
        let rel_uri = Uri::parse(path.rel.as_str()).expect("path should parse as a URI");
        let uri = uri.resolve(&rel_uri).unwrap().to_string();

        quote! {
            mod #name {
                use super::*;
                use grill::{error::CompileError, Key, Structure};
                const SCHEMA: &str = #schema;
                const URI: &str = #uri;
                const DESCRIPTION: &str = #description;
                fn setup() -> Result<(Key, Interrogator), &'static CompileError> {
                    use std::sync::OnceLock;
                    static INTERROGATOR: OnceLock<Result<(Key, Interrogator), CompileError>> = OnceLock::new();
                    INTERROGATOR
                        .get_or_init(|| {
                            let mut interrogator = super::interrogator()
                                .map_err(|err| panic!("failed to build interrogator:\n{}", err))
                                .unwrap();
                            interrogator
                                .source_str(URI, SCHEMA)
                                .map_err(|err| panic!("failed to source schema:\n: {err}"))
                                .unwrap();
                            let key = block_on(interrogator.compile(#uri))?;
                            Ok((key, interrogator))
                        })
                        .as_ref()
                        .map(Clone::clone)
                }
                #(#tests)*
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct Test {
    data: Box<RawValue>,
    description: String,
    valid: bool,
}

impl Test {
    fn generate(&self, i: usize) -> TokenStream {
        let Self {
            data,
            description,
            valid,
        } = self;

        let data = data.to_string();

        let description = description
            .replace("<=", "lte")
            .replace(">=", "gte")
            .replace('<', "lt")
            .replace('>', "gt")
            .replace('=', "eq");
        let name = format_ident!("test{}_{}", i, Name::snake(&description));

        quote! {
            #[test]
            fn #name() {
                let description = #description;
                let data = #data;
                let expected_valid = #valid;

                let (key, interrogator) = match setup() {
                    Ok((key, interrogator)) => (key, interrogator),
                    Err(err) => {
                        panic!("failed to setup test for {}\n:{}", description, err);
                    }
                };

                let data = match serde_json::from_str(data) {
                    Ok(data) => data,
                    Err(err) => {
                        panic!("failed to parse data as json:\n{}", err);
                    }
                };
                let output = match interrogator.evaluate(Structure::Flag, key, &data) {
                    Ok(output) => output,
                    Err(err) => {
                        panic!("failed to evaluate schema:\n{}", err);
                    }
                };
                let valid_msg = if expected_valid { "valid" } else { "invalid" };
                assert_eq!(output.valid(), expected_valid,
                    "expected the evaluation to be {valid_msg} for: \n  case: {DESCRIPTION}\n  test: {description}\n  schema:{SCHEMA}\n  data: {data}\n  expected: {valid_msg}"
                )
            }
        }
    }
}
