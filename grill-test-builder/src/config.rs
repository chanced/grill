use std::{collections::HashMap, path::PathBuf};

use grill::AbsoluteUri;
use serde::{Deserialize, Serialize};


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_config() {
        let raw = r#"
		[json-schema-test-suite]
		path    = "tests/json-schema-test-suite"
		sources = ["remotes/**/*.json"]
		[json-schema-test-suite.tests]
		draft2020-12 = ["*.json", "optional/format/*.json"]
		draft7       = ["*.json", "optional/format/*.json"]
		"#;

        let cfg = Config {
            suite: HashMap::from([(
                "json-schema-test-suite".into(),
                Suite {
                    path: "tests/json-schema-test-suite".into(),
                    base_uri: "http://localhost:1234".try_into().unwrap(),
                    sources: vec!["remotes".into()],
                    tests: HashMap::from([
                        (
                            "draft7".into(),
                            vec!["*".into(), "optional/format/*.json".into()],
                        ),
                        (
                            "draft2020-12".into(),
                            vec!["*".into(), "optional/format/*.json".into()],
                        ),
                    ]),
                },
            )]),
        };

        let t = toml::to_string_pretty(&cfg).unwrap();
        let cfg2: Config = toml::from_str(&t).unwrap();
        assert_eq!(cfg, cfg2);

        let cfg = std::fs::read_to_string("example-config.toml").unwrap();
        let cfg_value: toml::Value = toml::from_str(&cfg).unwrap();
        println!("{cfg_value:?}");
        let cfg_value: Config = toml::from_str(&cfg).unwrap();
        println!("{cfg_value:?}");
    }
}
