use super::*;
#[test]
fn test_resolve() {
    let abs_uri = AbsoluteUri::parse("http://a/b/c/d;p?q").unwrap();
    let uri = Uri::parse("http://a/b/c/d;p?q").unwrap();
    let tests = [
        ("g:h", "g:h"),
        ("g", "http://a/b/c/g"),
        ("./g", "http://a/b/c/g"),
        ("g/", "http://a/b/c/g/"),
        ("/g", "http://a/g"),
        ("//g", "http://g/"),
        ("?y", "http://a/b/c/d;p?y"),
        ("g?y", "http://a/b/c/g?y"),
        ("#s", "http://a/b/c/d;p?q#s"),
        ("g#s", "http://a/b/c/g#s"),
        ("g?y#s", "http://a/b/c/g?y#s"),
        (";x", "http://a/b/c/;x"),
        ("g;x", "http://a/b/c/g;x"),
        ("g;x?y#s", "http://a/b/c/g;x?y#s"),
        ("", "http://a/b/c/d;p?q"),
        (".", "http://a/b/c/"),
        ("./", "http://a/b/c/"),
        ("..", "http://a/b/"),
        ("../", "http://a/b/"),
        ("../g", "http://a/b/g"),
        ("../..", "http://a/"),
        ("../../", "http://a/"),
        ("../../g", "http://a/g"),
        ("../../../g", "http://a/g"),
        ("../../../../g", "http://a/g"),
        ("/./g", "http://a/g"),
        ("/../g", "http://a/g"),
        ("g.", "http://a/b/c/g."),
        (".g", "http://a/b/c/.g"),
        ("g..", "http://a/b/c/g.."),
        ("..g", "http://a/b/c/..g"),
        ("./../g", "http://a/b/g"),
        ("./g/.", "http://a/b/c/g/"),
        ("g/./h", "http://a/b/c/g/h"),
        ("g/../h", "http://a/b/c/h"),
        ("g;x=1/./y", "http://a/b/c/g;x=1/y"),
        ("g;x=1/../y", "http://a/b/c/y"),
        ("g?y/./x", "http://a/b/c/g?y/./x"),
        ("g?y/../x", "http://a/b/c/g?y/../x"),
        ("g#s/./x", "http://a/b/c/g#s/./x"),
        ("g#s/../x", "http://a/b/c/g#s/../x"),
    ];
    for (input, expected) in tests {
        let input = Uri::parse(input);
        if let Err(e) = &input {
            println!(
                "\n\nfailed to parse input: {:?};\n\terror: {}\n\n",
                &input, e
            );
        }
        let input = input.unwrap();

        let result = abs_uri.resolve(&input);

        if let Err(e) = &result {
            println!("\n\nfailed to resolve:\n\tinput: \"{input:?}\"\n\terror: {e:?}\n\n");
        }
        let result = result.unwrap();
        assert_eq!(
            &result, expected,
            "\n\nfailed to resolve:\n\tinput:\t\t{input},\n\tresult:\t\t{result}\n\texpected:\t{expected}\n\n"
        );

        let result = uri.resolve(&input);

        if let Err(e) = &result {
            println!("\n\nfailed to resolve:\n\tinput: \"{input:?}\"\n\terror: {e:?}\n\n");
        }
        let result = result.unwrap();
        assert_eq!(
            &result, expected,
            "\n\nfailed to resolve:\n\tinput:\t\t{input},\n\tresult:\t\t{result}\n\texpected:\t{expected}\n\n"
        );
    }
}

#[test]
fn test_base_path_segments() {
    let uri = Uri::parse("/path/to/file").unwrap();
    let segments = uri.base_path_segments().collect::<Vec<_>>();
    assert_eq!(
        segments,
        vec![
            PathSegment::Root,
            PathSegment::normal("path"),
            PathSegment::normal("to")
        ]
    );
}

#[test]
fn test_join() {
    let base = "/a/b/c";
    assert_eq!(super::merge(base, "x/y/z"), "/a/b/c/x/y/z");
    assert_eq!(super::merge(base, "/x/y/z"), "/x/y/z");
}

#[test]
fn test_uri_components() {
    let uri = Uri::parse("http://example.com/path?query#fragment").unwrap();
    let mut components = uri.components();
    assert_eq!(components.next(), Some(Component::Scheme("http".into())));
    assert_eq!(
        components.next(),
        Some(Component::Host("example.com".into()))
    );
}

#[test]
fn test_relative_uri_parse() {
    let tests = [
        (
            "/path?query#fragment",
            None,
            "/path",
            Some("query"),
            Some("fragment"),
        ),
        (
            "//example.com/path/path2?query=str#fragment",
            Some("example.com"),
            "/path/path2",
            Some("query=str"),
            Some("fragment"),
        ),
    ];

    for (input, authority, path, query, fragment) in tests {
        let uri = Uri::parse(input).unwrap();
        assert_eq!(authority, uri.authority_or_namespace().as_deref());
        assert_eq!(path, uri.path_or_nss());
        assert_eq!(query, uri.query());
        assert_eq!(fragment, uri.fragment());
    }
}

#[test]
fn test_set_query() {
    let mut uri = Uri::parse("/path").unwrap();
    assert_eq!(uri.query(), None);
    assert_eq!(uri.fragment(), None);

    uri.set_query(Some("q=str")).unwrap();
    assert_eq!(uri.as_str(), "/path?q=str");
    assert_eq!(uri.query(), Some("q=str"));

    uri.set_fragment(Some("fragment")).unwrap();
    assert_eq!(uri.as_str(), "/path?q=str#fragment");
    assert_eq!(uri.fragment(), Some("fragment"));

    uri.set_query(None).unwrap();
    assert_eq!(uri.query(), None);
    assert_eq!(uri.as_str(), "/path#fragment");

    uri.set_query(Some("?q=str")).unwrap();
    assert_eq!(uri.as_str(), "/path?q=str#fragment");

    uri.set_query(Some("q=str")).unwrap();
    assert_eq!(uri.query(), Some("q=str"));
}

#[test]
fn test_get_url_authority() {
    let url = Url::parse("https://user:example@example.com:8080").unwrap();
    let uri: AbsoluteUri = url.into();
    assert_eq!(
        uri.authority_or_namespace().as_deref(),
        Some("user:example@example.com:8080")
    );
}

#[test]
fn test_uri_authority_or_namespace() {
    let tests = [
        ("https://www.example.com", Some("www.example.com")),
        ("urn:example:resource", Some("example")),
        (
            "https://username:password@example.com/path",
            Some("username:password@example.com"),
        ),
        ("http://127.0.0.0:3400", Some("127.0.0.0:3400")),
        (
            "https://username@example.com/somepath",
            Some("username@example.com"),
        ),
        ("mailto:example@example.com", None),
    ];

    for (input, expected) in tests {
        let absolute_uri = AbsoluteUri::parse(input).unwrap();
        assert_eq!(expected, absolute_uri.authority_or_namespace().as_deref());
    }

    let tests = [
        ("https://www.example.com", Some("www.example.com")),
        ("urn:example:com", Some("example")),
        (
            "https://username:password@example.com/path",
            Some("username:password@example.com"),
        ),
        ("http://127.0.0.0:3400", Some("127.0.0.0:3400")),
        (
            "https://username@example.com/somepath",
            Some("username@example.com"),
        ),
        ("mailto:example@example.com", None),
        ("/relative", None),
    ];

    for (input, expected) in tests {
        let uri = Uri::parse(input).unwrap();
        assert_eq!(expected, uri.authority_or_namespace().as_deref());
    }
}

#[test]
fn test_fragment() {
    let tests = [
        ("https://www.example.com", None),
        ("urn:example:resource", None),
        (
            "https://username:password@example.com/path#fraggle-rock",
            Some("fraggle-rock"),
        ),
        ("https://example.com:3400/path#with-port", Some("with-port")),
        (
            "https://username:password@example.com/somepath#with-credentials",
            Some("with-credentials"),
        ),
        ("mailto:example@example.com", None),
    ];

    for (input, expected) in tests {
        let absolute_uri = AbsoluteUri::parse(input).unwrap();
        assert_eq!(expected, absolute_uri.fragment());
    }
    let tests = [
        ("https://www.example.com", None),
        ("urn:example:resource", None),
        (
            "https://username:password@example.com/path#fraggle-rock",
            Some("fraggle-rock"),
        ),
        ("https://example.com:3400/path#with-port", Some("with-port")),
        (
            "https://username:password@example.com/somepath#with-credentials",
            Some("with-credentials"),
        ),
        ("mailto:example@example.com", None),
        ("/relative#fragment", Some("fragment")),
        ("#fragment", Some("fragment")),
    ];

    for (input, expected) in tests {
        let uri = Uri::parse(input).unwrap();
        assert_eq!(expected, uri.fragment());
    }
}

#[test]
fn test_set_fragment() {
    let tests = [
        (
            "https://www.example.com/",
            None,
            None,
            "https://www.example.com/",
        ),
        (
            "https://username:password@example.com/path#fragment",
            Some("fragment/nested"),
            Some("fragment/nested"),
            "https://username:password@example.com/path#fragment/nested",
        ),
        (
            "https://example.com/path#with-fragment",
            None,
            None,
            "https://example.com/path",
        ),
        (
            "urn:example:resource",
            Some("fragment"),
            Some("fragment"),
            "urn:example:resource#fragment",
        ),
        (
            "urn:example:resource",
            Some("some fragment with spaces"),
            Some("some%20fragment%20with%20spaces"),
            "urn:example:resource#some%20fragment%20with%20spaces",
        ),
        (
            "https://example.com/path#with-fragment",
            Some("fragment with spaces"),
            Some("fragment%20with%20spaces"),
            "https://example.com/path#fragment%20with%20spaces",
        ),
    ];

    for (input, fragment, expected_fragment, expected_uri) in tests {
        let mut absolute_uri = AbsoluteUri::parse(input).unwrap();
        absolute_uri.set_fragment(fragment).unwrap();
        assert_eq!(expected_uri, absolute_uri.to_string());
        assert_eq!(expected_fragment, absolute_uri.fragment());
    }

    let tests = [
        (
            "https://www.example.com/",
            None,
            None,
            "https://www.example.com/",
        ),
        (
            "https://username:password@example.com/path#fragment",
            Some("fragment/nested"),
            Some("fragment/nested"),
            "https://username:password@example.com/path#fragment/nested",
        ),
        (
            "https://example.com/path#with-fragment",
            None,
            None,
            "https://example.com/path",
        ),
        (
            "urn:example:resource",
            Some("fragment"),
            Some("fragment"),
            "urn:example:resource#fragment",
        ),
        (
            "urn:example:resource",
            Some("some fragment with spaces"),
            Some("some%20fragment%20with%20spaces"),
            "urn:example:resource#some%20fragment%20with%20spaces",
        ),
        (
            "https://example.com/path#with-fragment",
            Some("fragment with spaces"),
            Some("fragment%20with%20spaces"),
            "https://example.com/path#fragment%20with%20spaces",
        ),
        (
            "/partial/path#existing-fragment",
            Some("new-fragment"),
            Some("new-fragment"),
            "/partial/path#new-fragment",
        ),
        (
            "#existing-fragment",
            Some("new-fragment"),
            Some("new-fragment"),
            "#new-fragment",
        ),
        ("#existing-fragment", None, None, ""),
        (
            "/partial/path#existing-fragment",
            None,
            None,
            "/partial/path",
        ),
        (
            "#existing-fragment",
            Some("new fragment with spaces"),
            Some("new%20fragment%20with%20spaces"),
            "#new%20fragment%20with%20spaces",
        ),
        (
            "/partial/path",
            Some("fragment%20with%20spaces"),
            Some("fragment%20with%20spaces"),
            "/partial/path#fragment%20with%20spaces",
        ),
    ];
    for (input, fragment, expected_fragment, expected_uri) in tests {
        let mut uri = Uri::parse(input).unwrap();
        uri.set_fragment(fragment).unwrap();
        assert_eq!(expected_uri, uri.to_string());
        assert_eq!(expected_fragment, uri.fragment());
    }
}
#[test]
fn test_set_path() {
    let tests = [
        (
            "https://www.example.com",
            "/new-path",
            "/new-path",
            "https://www.example.com/new-path",
        ),
        (
            "https://username:password@example.com/path#fraggle-rock",
            "/new-path",
            "/new-path",
            "https://username:password@example.com/new-path#fraggle-rock",
        ),
        (
            "https://example.com/path#with-fragment",
            "",
            "/",
            "https://example.com/#with-fragment",
        ),
        (
            "urn:example:resource#fragment",
            "new-resource",
            "new-resource",
            "urn:example:new-resource#fragment",
        ),
        (
            "urn:example:resource",
            "new-resource",
            "new-resource",
            "urn:example:new-resource",
        ),
        (
            "https://example.com/",
            "new path",
            "/new%20path",
            "https://example.com/new%20path",
        ),
        (
            "urn:example:resource#fragment",
            "new resource",
            "new%20resource",
            "urn:example:new%20resource#fragment",
        ),
        (
            "urn:example:resource",
            "some path with spaces",
            "some%20path%20with%20spaces",
            "urn:example:some%20path%20with%20spaces",
        ),
    ];
    for (input, new_path, expected_path, expected) in tests {
        let mut absolute_uri = AbsoluteUri::parse(input).unwrap();
        absolute_uri.set_path_or_nss(new_path).unwrap();
        assert_eq!(expected, absolute_uri.to_string());
        assert_eq!(expected_path, absolute_uri.path_or_nss());
    }

    let tests = [
        (
            "https://www.example.com",
            "/new-path",
            "/new-path",
            "https://www.example.com/new-path",
        ),
        (
            "https://username:password@example.com/path#fraggle-rock",
            "/new-path",
            "/new-path",
            "https://username:password@example.com/new-path#fraggle-rock",
        ),
        (
            "https://example.com/path#with-fragment",
            "",
            "/",
            "https://example.com/#with-fragment",
        ),
        (
            "urn:example:resource#fragment",
            "new-resource",
            "new-resource",
            "urn:example:new-resource#fragment",
        ),
        (
            "urn:example:resource",
            "new-resource",
            "new-resource",
            "urn:example:new-resource",
        ),
        ("", "/new-path", "/new-path", "/new-path"),
        ("/", "/resource", "/resource", "/resource"),
        (
            "/path#fragment",
            "/new-path",
            "/new-path",
            "/new-path#fragment",
        ),
        (
            "https://example.com/",
            "new path",
            "/new%20path",
            "https://example.com/new%20path",
        ),
        (
            "urn:example:resource#fragment",
            "new resource",
            "new%20resource",
            "urn:example:new%20resource#fragment",
        ),
    ];
    for (input, new_path, expected_path, expected) in tests {
        let mut uri = Uri::parse(input).unwrap();
        uri.set_path_or_nss(new_path).unwrap();
        assert_eq!(expected, uri.to_string());
        assert_eq!(expected_path, uri.path_or_nss());
    }
}
