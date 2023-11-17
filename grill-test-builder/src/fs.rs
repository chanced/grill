use crate::{Error, GlobSnafu, IoSnafu};
use camino::{Utf8Path, Utf8PathBuf};
use glob::glob;
use snafu::prelude::*;
use std::{
    fs::{self},
    io::Read,
    iter::Peekable,
    path::PathBuf,
};

#[derive(Debug)]
pub(crate) struct File {
    pub(crate) inner: fs::File,
    pub(crate) path: Path,
}

impl File {
    pub(crate) fn read_to_string(&mut self) -> Result<String, Error> {
        let mut buf = String::new();
        self.inner
            .read_to_string(&mut buf)
            .map_err(|source| Error::Io {
                pattern: Some(self.path.pattern.clone()),
                path: self.path.full.clone(),
                source,
            })?;
        Ok(buf)
    }
}
impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Path {
    pub(crate) rel: Utf8PathBuf,
    pub(crate) full: PathBuf,
    pub(crate) pattern: Utf8PathBuf,
}
impl Path {
    pub(crate) fn is_file(&self) -> bool {
        self.full.is_file()
    }
}

#[derive(Debug)]
pub(crate) struct Paths {
    current: Option<(Utf8PathBuf, Peekable<glob::Paths>)>,
    patterns: Vec<Utf8PathBuf>,
    ancestry: Utf8PathBuf,
}

impl Paths {
    pub(crate) fn new(ancestry: Utf8PathBuf, patterns: Vec<Utf8PathBuf>) -> Self {
        Self {
            current: None,
            patterns,
            ancestry,
        }
    }
    pub(crate) fn files(self) -> Files {
        Files { paths: self }
    }

    fn take_current_or_next(
        &mut self,
    ) -> Option<Result<(Utf8PathBuf, Peekable<glob::Paths>), Error>> {
        self.current
            .take()
            .map(Result::Ok)
            .or_else(|| self.next_glob())
    }

    fn next_glob(&mut self) -> Option<Result<(Utf8PathBuf, Peekable<glob::Paths>), Error>> {
        let pattern = self.ancestry.join(
            self.patterns
                .pop()?
                .as_path()
                .as_str()
                .trim_start_matches("./"),
        );

        let next = glob(pattern.as_str())
            .with_context(|_| GlobSnafu {
                pattern: pattern.clone(),
            })
            .map(|paths| (pattern, paths.peekable()));
        Some(next)
    }
    fn set_current(
        &mut self,
        pattern: Utf8PathBuf,
        mut paths: Peekable<glob::Paths>,
    ) -> Option<Result<(), Error>> {
        if paths.peek().is_some() {
            self.current = Some((pattern, paths));
            return Some(Ok(()));
        }
        if !self.patterns.is_empty() {
            let next_glob = self.next_glob()?;
            match next_glob {
                Ok(current) => {
                    self.current = Some(current);
                    return Some(Ok(()));
                }
                Err(err) => return Some(Err(err)),
            }
        }
        None
    }
}

impl Iterator for Paths {
    type Item = Result<Path, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.take_current_or_next() {
            let Ok((pattern, mut paths)) = result else {
                return Some(Err(result.unwrap_err()));
            };
            let path = match paths.next() {
                Some(Ok(path)) => path,
                Some(Err(err)) => {
                    return Some(Err(Error::from_glob_error(err, pattern)));
                }
                None => match self.set_current(pattern, paths)? {
                    Ok(()) => continue,
                    Err(err) => return Some(Err(err)),
                },
            };
            let full = path.clone();
            let path = match unixify_path(&path) {
                Ok(path) => path,
                Err(err) => return Some(Err(err)),
            };
            let rel = path.strip_prefix(&self.ancestry).unwrap().to_owned();
            self.current = Some((pattern.clone(), paths));
            return Some(Ok(Path { rel, full, pattern }));
        }
        None
    }
}

#[derive(Debug)]
pub(crate) struct Files {
    paths: Paths,
}

impl Iterator for Files {
    type Item = Result<File, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.paths.next()? {
            Ok(path) => match fs::File::open(&path.full) {
                Ok(file) => Ok(File { inner: file, path }),
                Err(source) => Err(Error::Io {
                    pattern: Some(path.pattern),
                    path: path.full.clone(),
                    source,
                }),
            },
            Err(err) => Err(err),
        };
        Some(result)
    }
}

pub(crate) fn find<'a, A, I, P, S>(ancestry: A, patterns: P) -> Paths
where
    A: 'a + IntoIterator<Item = I>,
    I: 'a + AsRef<Utf8Path>,
    P: 'a + IntoIterator<Item = S>,
    S: 'a + AsRef<Utf8Path>,
{
    let ancestry = Utf8PathBuf::from_iter(ancestry);
    let patterns = patterns
        .into_iter()
        .map(|p| p.as_ref().to_path_buf())
        .collect::<Vec<_>>();
    Paths::new(ancestry, patterns)
}

pub(crate) fn open<'a, A, I, P, S>(ancestry: A, patterns: P) -> Files
where
    A: 'a + IntoIterator<Item = I>,
    I: 'a + AsRef<Utf8Path>,
    P: 'a + IntoIterator<Item = S>,
    S: 'a + AsRef<Utf8Path>,
{
    find(ancestry, patterns).files()
}

pub fn write(items: Vec<(Utf8PathBuf, String, bool)>) -> Result<(), Error> {
    for (path, contents, overwrite) in items {
        match fs::metadata(&path) {
            Ok(_) => {
                if !overwrite {
                    continue;
                }
            }
            Err(source) => {
                if !matches!(source.kind(), std::io::ErrorKind::NotFound) {
                    return Err(Error::Io {
                        pattern: None,
                        path: PathBuf::from(path),
                        source,
                    });
                }
            }
        }
        fs::write(&path, contents).with_context(|_| IoSnafu {
            path: PathBuf::from(path),
            pattern: None,
        })?;
    }
    Ok(())
}

#[cfg(test)]
pub(crate) fn set_test_current_dir() {
    let mut cwd = std::env::current_dir().unwrap();
    if cwd.ends_with("grill-test-builder") {
        cwd.pop();
    }
    std::env::set_current_dir(cwd).unwrap();
}

#[allow(clippy::unnecessary_wraps)]
pub(crate) fn unixify_path(path: &std::path::Path) -> Result<Utf8PathBuf, Error> {
    // TODO: setup and use
    #[cfg(target_os = "windows")]
    let path = Utf8PathBuf::from_str(
        path_slash::PathBufExt::to_slash(path)
            .ok_or_else(|| Error::NotUtf8 { path: path.clone() })?
            .as_ref(),
    )
    .unwrap();

    #[cfg(not(target_os = "windows"))]
    let path = Utf8PathBuf::from_path_buf(path.into()).expect("utf8 path");
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;
    #[test]
    fn test_read_files() {
        set_test_current_dir();
        let _cwd = std::env::current_dir().unwrap();
        let root = current_dir().unwrap();
        let root = Utf8PathBuf::from_path_buf(root).unwrap();
        let patterns = vec![Utf8PathBuf::from("grill-test-builder/fixtures/*.json")];
        let files = open([&root], patterns);
        let _files = files
            .map(|r| {
                if let Err(err) = r {
                    panic!("{err}")
                } else {
                    r.unwrap()
                }
            })
            .collect::<Vec<_>>();
        // assert!(files.contains(|f| f.));
    }
}
