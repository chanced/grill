use camino::{Utf8Path, Utf8PathBuf};
use grill_test_builder::{load_cfg, Error};
use snafu::ResultExt;
fn main() -> Result<(), Error> {
    let cfg = load_cfg("grill-test-builder/fixtures/tests.toml")?;
    let cwd = Utf8PathBuf::from_path_buf(std::env::current_dir().unwrap())
        .map_err(|path| Error::NotUtf8 { path })?;

    grill_test_builder::generate(cwd, &cfg)
}
