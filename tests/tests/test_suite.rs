use std::path::PathBuf;

use grill::{AbsoluteUri, Interrogator, JsonSchema, Uri};

#[tokio::test]
async fn test_draft_2020_12() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let uri = AbsoluteUri::parse("http://localhost:1234/draft2020-12/root").unwrap();
    let resolved = uri.resolve(&Uri::parse("nested.json").unwrap());
    println!("{resolved:?}");
    let mut path = PathBuf::from(std::env::current_dir().unwrap().to_string_lossy().as_ref());
    if !path.ends_with("tests") {
        path.push("tests");
    }
    std::env::set_current_dir(path).unwrap();
    let interrogator = Interrogator::build()
        .json_schema_2020_12()
        .finish()
        .await
        .unwrap();
    tests::run(interrogator, "draft2020-12").await;
}
