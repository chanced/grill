use crate::{Error, GlobSnafu, IoSnafu};
use camino::{Utf8Path, Utf8PathBuf};
use glob::glob;
use snafu::prelude::*;
use std::{fs::read_to_string, path::PathBuf};

pub(crate) fn read_files<'a, R, I, P, S>(
    roots: R,
    patterns: P,
) -> Result<Vec<(Utf8PathBuf, String)>, Error>
where
    R: 'a + IntoIterator<Item = I>,
    I: 'a + AsRef<Utf8Path>,
    P: 'a + IntoIterator<Item = S>,
    S: 'a + AsRef<Utf8Path>,
{
    let mut results = Vec::new();
    let root = roots
        .into_iter()
        .fold(Utf8PathBuf::new(), |acc, p| acc.join(p));

    for pattern in patterns {
        let pattern = root.join(&pattern);
        let paths = glob(pattern.as_str()).context(GlobSnafu {
            pattern: pattern.to_path_buf(),
        })?;

        for path in paths {
            let path = match path {
                Ok(path) => path,
                Err(err) => {
                    return Err(Error::from_glob_error(err, pattern));
                }
            };

            if !path.is_file() {
                continue;
            }

            let content = read_to_string(&path).with_context(|_| IoSnafu {
                path: path.clone(),
                pattern: pattern.clone(),
            })?;

            // TODO: setup and use
            #[cfg(target_os = "windows")]
            let path = Utf8PathBuf::from_str(
                path_slash::PathBufExt::to_slash(&path)
                    .ok_or_else(|| Error::NotUtf8 { path: path.clone() })?
                    .as_ref(),
            )
            .unwrap();

            #[cfg(not(target_os = "windows"))]
            let path = Utf8PathBuf::from_path_buf(path).expect("utf8 path");

            let path = path.strip_prefix(&root).unwrap().to_owned();
            results.push((path, content));
        }
    }
    Ok(results)
}

pub fn write_files(items: Vec<(Utf8PathBuf, String, bool)>) -> Result<(), Error> {
    for (path, contents, overwrite) in items {
        match std::fs::metadata(&path) {
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
        std::fs::write(path, contents).with_context(|_| IoSnafu {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::current_dir;
    #[test]
    fn test_read_files() {
        set_test_current_dir();
        let root = current_dir().unwrap();
        let root = Utf8PathBuf::from_path_buf(root).unwrap();
        let patterns = vec![Utf8PathBuf::from("grill-test-builder/fixtures/*.json")];
        let files = read_files([&root], patterns).unwrap();
        assert!(!files.is_empty());
        let names = files.iter().map(|(name, _)| name).collect::<Vec<_>>();
        assert!(names.contains(&&Utf8PathBuf::from("grill-test-builder/fixtures/file.json")));
    }
}
