use std::{
    collections::{HashMap, HashSet},
    convert::AsRef,
    fs::File,
    io::Write,
};

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

const RESERVED: [&str; 52] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type", "union",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

#[derive(Debug)]
pub(crate) struct SuiteOutput {
    runner: String,
    tests: String,
}

pub(crate) fn gen_suite(ancestry: &[&Utf8Path], suite: &Suite) -> Result<SuiteOutput, Error> {
    let Suite {
        sources,
        base_uri,
        tests,
    } = suite;
    // janky
    let suite_name = ancestry
        .get(1)
        .expect("ancestry should have at least 2 elements");
    let suite_name = format_ident!("{}", suite_name.as_str().to_snake_case());

    let ancestry = Utf8PathBuf::from_iter(ancestry);
    let sources = gen_sources(base_uri, &ancestry, sources.as_ref())?;
    let file_paths = find_test_files(&ancestry, tests)?;
    let sets = file_paths
        .into_iter()
        .map(|(name, cases)| TestSet::new(name, cases))
        .collect::<Vec<_>>();

    let tests = gen_tests(&suite_name, &sets)?;
    let lib = gen_runners(&sets);
    todo!()
    // let tests = format_src(tests.to_string())?;
    // let lib = format_src(lib.to_string())?;
    // Ok(SuiteOutput { runner: lib, tests })
}

fn gen_runners(sets: &[TestSet]) -> TokenStream {
    let assoc_types = sets.iter().map(TestSet::runner_assoc_types);
    let accessors = sets.iter().map(TestSet::runner_accessors);
    let traits = sets.iter().map(TestSet::runner_trait);
    quote! {
        use serde_json::{json, Value};
        use once_cell::sync::Lazy;
        use grill::{ Interrogator, Key, Building };

        pub trait Runner: Copy {
            #(#assoc_types)*
            #(#accessors)*
        }
        #(#traits)*
    }
}

fn gen_tests(suite: &Ident, sets: &[TestSet]) -> Result<HashMap<Utf8PathBuf, TokenStream>, Error> {
    let mut tests = HashMap::new();
    for set in sets {
        let path = Utf8PathBuf::from(set.name.snake.to_string());
        let mut output = set.tests(suite, Vec::new())?;
        for (k, v) in output.drain() {
            tests.insert(path.join(k), v);
        }
    }
    Ok(tests)
}

fn format_src(src: String) -> Result<String, Error> {
    use std::process::{Command, Stdio};
    let mut cmd = Command::new("rustfmt")
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
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
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
) -> Result<TokenStream, Error> {
    let Some(sources) = sources else {
        return Ok(TokenStream::new());
    };
    let mut entries = Vec::new();
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
        let content = quote! {
            (#uri, json!(#content)),
        };
        entries.push(content);
    }

    let src = quote! {
        fn sources() -> &'static [(&'static str, Value)] {
            static VALUES: Lazy<Vec<(&'static str, Value)>> = Lazy::new(|| {
                Vec::from(
                    [#(#entries)*]
                )
            });
            &VALUES
        }
    };
    Ok(src)
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
    snake: Ident,
    pascal: Ident,
}

impl Name {
    pub fn new(name: &str) -> Self {
        // let mut snake = name.as_str().to_snake_case();
        // if RESERVED.contains(&&*snake) {
        //     snake = format!("{snake}_");
        // }
        Self {
            string: name.to_string(),
            snake: format_ident!("{}", name.to_snake_case()),
            pascal: format_ident!("{}", name.to_pascal_case()),
        }
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
        suite: &'t Ident,
        mut ancestry: Vec<&'t Ident>,
    ) -> Result<HashMap<Utf8PathBuf, TokenStream>, Error> {
        ancestry.push(&self.name.snake);

        let mods = &self
            .sets
            .iter()
            .map(|set| {
                let name = &set.name.snake;
                quote!(mod #name;)
            })
            .collect::<TokenStream>();
        let mut files = HashMap::new();
        let set_path = Utf8PathBuf::from(self.name.snake.to_string());
        for path in &self.cases {
            let cases = open_test_cases(path)?;
            let src: TokenStream = cases
                .iter()
                .map(|case| case.generate(suite, path, &ancestry))
                .collect();

            let mut path = set_path.join(&path.rel);
            let mut filename = Utf8PathBuf::from(path.file_name().unwrap());
            path.pop();
            filename.set_extension("");
            let mut filename = Utf8PathBuf::from(filename.as_str().to_snake_case());
            filename.set_extension("rs");

            files.insert(path.join(filename), src);
        }

        let rel_path = Utf8PathBuf::from(self.name.snake.to_string());
        let mod_tokens = quote! {
            #mods
        };
        files.insert(rel_path.join("mod.rs"), mod_tokens);

        for set in &self.sets {
            let mut set_files = set.tests(suite, ancestry.clone())?;
            for (k, v) in set_files.drain() {
                files.insert(rel_path.join(k), v);
            }
        }

        println!("{:#?}", files.keys());

        Ok(files)
    }

    fn runner_accessors(&self) -> TokenStream {
        let name_pascal = &self.name.pascal;
        let name_snake = &self.name.snake;
        quote! {
            fn #name_snake(&self) -> Self::#name_pascal;
        }
    }
    fn runner_assoc_types(&self) -> TokenStream {
        let name = &self.name.pascal;
        quote! {
            type #name;
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
    fn runner_trait_methods(&self) -> TokenStream {
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

    fn runner_trait(&self) -> TokenStream {
        let name = &self.name.pascal;
        let methods = self.runner_trait_methods();
        quote! {
            pub trait #name {
                fn interrogator(&self) -> Building;
                #methods
            }
        }
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
    fn generate(&self, suite: &Ident, path: &Path, ancestry: &[&Ident]) -> TokenStream {
        let description = &self.description;
        let setup_fn_calls = ancestry
            .iter()
            .map(|&ancestor| format_ident!("setup_{ancestor}"))
            .map(|setup_fn| {
                quote! {
                    tests::#suite().#setup_fn(&mut interrogator);
                }
            });

        let mut name = description.to_snake_case();
        if RESERVED.contains(&&*name) {
            name.push('_');
        }
        let name = format_ident!("{}", description.to_snake_case());
        let schema = self.schema.to_string();
        let tests = self.tests.iter().map(|test| test.generate(&name, suite));

        quote! {
            mod #name {
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
    fn generate(&self, name: &Ident, suite: &Ident) -> TokenStream {
        let Self {
            data,
            description,
            valid,
        } = self;
        let setup_fn = format_ident!("setup_{name}");
        let data = data.to_string();
        let name = format_ident!("test_{}", description.to_snake_case());
        quote! {
            #[tokio::test]
            fn #name() {
            ///    let description = #description;
            ///    let data = match serde_json::from_str(#data) {
            ///        Ok(data) => data,
            ///        Err(err) => panic!("failed to parse data for {description} \n caused by:\n\n{err:?}"),
            ///    };
            ///    let key = match key().await {
            ///        Ok(key) => key,
            ///        Err(err) => panic!("failed to compile schema for {description} \n caused by:\n\n{err:?}"),
            ///    }
            ///    let test = Test{
            ///        schema_key: key,
            ///        description: #description,
            ///        data: data,
            ///        valid: #valid
            ///    };
            ///    let mut interrogator = interrogator().await;
            ///    let builder = RUNNER.#setup_fn(&mut builder, &test);
            ///    let result = interrogator.evaluate(Structure::Flag, key,  &data);
            ///    assert_eq!(result.valid, valid, "{description}");
            struct X;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{load_cfg, tests::set_test_cwd};
    #[test]
    fn test_suite() {
        set_test_cwd();
        let cwd = std::env::current_dir().unwrap();
        let cfg = load_cfg(Utf8Path::new("grill-test-builder/fixtures/tests.toml")).unwrap();

        for (path, suite) in &cfg.suite {
            // let file = syn::parse_file(&s.to_string()).unwrap();
            let content = match gen_suite(&[&cfg.tests_dir, path], suite) {
                Ok(content) => content,
                Err(err) => panic!("{err}"),
            };
            let runner = cwd.join(&cfg.tests_dir).join("tests/").join(path);
            std::fs::create_dir_all(&runner).unwrap();
            let runner = runner.join("runner.rs");
            let tests = cwd.join(&cfg.tests_dir).join("tests/").join(path);
            std::fs::create_dir_all(&tests).unwrap();
            let tests = tests.join("tests.rs");
            println!("writing {runner:?}");
            // let content = prettyplease::unparse(&file).to_string();
            std::fs::write(runner, content.runner).unwrap();
            println!("writing {tests:?}");
            std::fs::write(tests, content.tests).unwrap();
        }
    }
}
