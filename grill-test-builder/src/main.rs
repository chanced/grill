use camino::{Utf8Path, Utf8PathBuf};
use grill_test_builder::{load_cfg, Error};
use snafu::ResultExt;
fn main() -> Result<(), Error> {
    // TODO: this should have a param override
    let cfg = load_cfg("tests.toml")?;
    grill_test_builder::generate(std::env::current_dir().unwrap(), &cfg)?;
    Ok(())
}
