use glob::glob;
use std::{
    fs,
    ops::Deref,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Error};
use path_slash::PathBufExt as _;
pub(super) fn find_dirs(root: &Path, pattern: &Path) -> Result<Vec<PathBuf>, Error> {
    // let mut dirs = Vec::new();
    // for path in glob(pattern.to_str()).context(format!("failed to parse glob: \"{pattern}\""))? {
    //     let path = path.context(format!("failed to parse glob: \"{pattern}\""))?;
    //     if !path.is_dir() {
    //         continue;
    //     }
    //     let path = path.to_slash().context("path not valid utf8")?;
    //     let path = PathBuf::from(path.as_ref());
    //     dirs.push(path);
    // }
    // Ok(dirs)
    todo!()
}

pub(crate) fn read_files<'a, T, S>(
    root: &Path,
    patterns: T,
) -> Result<Vec<(PathBuf, String)>, Error>
where
    T: 'a + IntoIterator<Item = S>,
    S: 'a + Deref<Target = Path>,
{
    todo!()
}
//     let mut results = Vec::new();
//     for pattern in patterns {
//         let pattern = &*pattern;
//         for path in glob(pattern.as_str())? {
//             let path = path.context(format!("failed to parse glob: \"{pattern}\""))?;
//             if !path.is_file() {
//                 continue;
//             }
//             let content =
//                 fs::read_to_string(&path).context(format!("failed to read file \"{path:?}\""))?;
//             // let path = path.to_slash().context("path not valid utf8")?;
//             let path = RelativePathBuf::from_path(path).unwrap();
//             results.push((root.relative(path), content));
//         }
//     }
//     Ok(results)
// }

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use super::*;

    #[test]
    fn test_find_dirs() {
        let mut root = current_dir().unwrap();
        if root.ends_with("grill-test-builder") {
            root.pop();
        }
        println!("{root:?}");
        let target = PathBuf::from("./tests/*");
        let dirs = find_dirs(&root, &target).unwrap();
        println!("{dirs:#?}");
    }
}
