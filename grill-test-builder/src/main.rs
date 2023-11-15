use camino::{Utf8Path, Utf8PathBuf};
use grill_test_builder::{load_cfg, Error};
use snafu::ResultExt;
fn main() -> Result<(), Error> {
    // TODO: this should have a param override
    let cfg = load_cfg("tests.toml")?;
    let files = grill_test_builder::generate(std::env::current_dir().unwrap(), &cfg)?;
    for (path, content, overwrite) in files {
        let meta = std::fs::metadata(&path);
        let file_name = path.file_name().unwrap();
        let dir_path = path.parent().unwrap();
        std::fs::create_dir_all(dir_path).unwrap();
        println!("writing {:?}", &path);
        std::fs::write(&path, content).unwrap();
    }
    Ok(())
}
