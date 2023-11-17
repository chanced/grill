use std::{ops::Range, str::FromStr};

use crate::big::usize_to_u32;

use super::{encode, write, RelativeUri, Uri};
use crate::error::{AuthorityError, InvalidPortError, UriError};
use url::Url;
use urn::Urn;

pub(super) fn uri(value: &str) -> Result<Uri, UriError> {
    Parse::uri(value)
}

pub(super) fn authority(value: &str) -> Result<super::Authority, AuthorityError> {
    Parse::authority(value)
}

#[derive(Default, Debug)]
struct Parse<'a> {
    state: State,
    input: &'a str,
    path_index: u32,
    username_index: Option<u32>,
    password_index: Option<u32>,
    host_index: Option<u32>,
    port_index: Option<u32>,
    query_index: Option<u32>,
    fragment_index: Option<u32>,
}

impl<'a> Parse<'a> {
    fn uri(input: &'a str) -> Result<Uri, UriError> {
        let mut parser = Self {
            state: State::Head,
            input,
            ..Default::default()
        };
        input
            .char_indices()
            .find_map(|(i, c)| parser.next(i, c))
            .unwrap_or_else(|| parser.finalize_uri())
    }

    fn authority(input: &'a str) -> Result<super::Authority, AuthorityError> {
        use AuthorityState::*;
        let mut parser = Self {
            state: State::Authority(Username),
            input,
            ..Default::default()
        };
        for (i, c) in input.char_indices() {
            //safety: there are no states which can error out before the authority is complete
            parser.next(i, c).transpose().unwrap();
        }

        if parser.host_index.is_none() && parser.username_index.is_some() {
            parser.port_index = parser.password_index.take();
            parser.host_index = parser.username_index.take();
        }

        if !parser.path().is_empty() {
            return Err(AuthorityError::ContainsPath(parser.path().to_string()));
        }
        let query = parser.query().unwrap_or_default();
        if !query.is_empty() {
            return Err(AuthorityError::ContainsQuery(query.to_string()));
        }
        let fragment = parser.fragment().unwrap_or_default();
        if !fragment.is_empty() {
            return Err(AuthorityError::ContainsFragment(fragment.to_string()));
        }

        Ok(super::Authority {
            value: input.into(),
            username_index: parser.username_index,
            password_index: parser.password_index,
            host_index: parser.host_index,
            port_index: parser.port_index,
            port: parser.port().transpose()?,
        })
    }

    fn next(&mut self, index: usize, next: char) -> Option<Result<Uri, UriError>> {
        use AuthorityState::*;
        use State::*;
        let index = match usize_to_u32(index) {
            Ok(i) => i,
            Err(err) => return Some(Err(err.into())),
        };
        let next_state = self.state.next(next);
        match next_state {
            Authority(Username) => self.set_username(index),
            Authority(Password) => self.set_password(index),
            Authority(Host) => self.set_host(index),
            Authority(Port) => self.set_port(index),
            UrlSchemeComplete => return self.parse_url(),
            UrnSchemeComplete => return self.parse_urn(),
            Path => self.set_path_index(index),
            Query => self.set_query(index),
            Fragment => self.set_fragment(index),
            LeadingSlash | LeadingSlash2 | UrlScheme | UrnScheme(_) => {}
            Head => unreachable!(),
        };
        self.state = next_state;
        None
    }

    fn finalize_uri(mut self) -> Result<Uri, UriError> {
        if self.username_index.is_some() && self.host().is_none() {
            self.host_index = self.username_index.take();
            self.port_index = self.password_index.take();
        }
        let mut buf = String::with_capacity(self.input.len());
        let username = encode::username(self.username());
        let password = encode::password(self.password());
        let host = encode::host(self.host());
        let port = self.port().transpose()?;
        let path = encode::path(self.path());
        let query = encode::query(self.query());
        let port_str = self.port_str();
        let fragment = encode::fragment(self.fragment());
        let has_path = !path.is_empty();
        let has_authority = username.is_some() || host.is_some();
        let username_index = write::username(&mut buf, username)?;
        let password_index = write::password(&mut buf, password)?;
        let host_index = write::host(&mut buf, host)?;
        let port_index = write::port(&mut buf, port_str)?;
        let path_index = write::path(&mut buf, path)?;
        let query_index = write::query(&mut buf, query, has_authority, has_path)?;
        let fragment_index = write::fragment(&mut buf, fragment)?;

        Ok(RelativeUri {
            value: buf,
            username_index,
            password_index,
            host_index,
            port,
            port_index,
            path_index,
            query_index,
            fragment_index,
        }
        .into())
    }
    fn parse_url(&mut self) -> Option<Result<Uri, UriError>> {
        Url::parse(self.input)
            .map(Uri::Url)
            .map_err(UriError::FailedToParseUrl)
            .into()
    }

    fn parse_urn(&mut self) -> Option<Result<Uri, UriError>> {
        Urn::from_str(self.input)
            .map(Uri::Urn)
            .map_err(UriError::FailedToParseUrn)
            .into()
    }

    fn host(&self) -> Option<&str> {
        let end = self.port_index().unwrap_or(self.path_index());
        self.host_index().map(|mut i| {
            if self.password_index.is_some() || self.username_index.is_some() {
                i += 1;
            }
            &self.input[i..end]
        })
    }

    fn port(&self) -> Option<Result<u16, InvalidPortError>> {
        let port = self.port_str()?;
        port.parse::<u16>()
            .map_err(|_| InvalidPortError(port.into()))
            .into()
    }

    fn port_str(&self) -> Option<&str> {
        self.port_index()
            .map(|i| &self.input[i + 1..self.path_index()])
    }

    fn path(&self) -> &str {
        &self.input[self.path_range()]
    }

    fn username(&self) -> Option<&str> {
        self.username_index().map(|i| {
            &self.input[i..self
                .password_index()
                .or(self.host_index())
                .unwrap_or(self.path_index())]
        })
    }

    fn password(&self) -> Option<&str> {
        let end = self.host_index().unwrap_or(self.path_index());
        self.password_index().map(|i| &self.input[i + 1..end])
    }

    fn query(&self) -> Option<&str> {
        self.query_index()
            .map(|i| &self.input[i + 1..self.fragment_index().unwrap_or(self.input.len())])
    }

    fn fragment(&self) -> Option<&str> {
        self.fragment_index()
            .map(|i| &self.input[i..self.input.len()])
    }

    fn set_query(&mut self, index: u32) {
        self.maybe_finalize_authority(index);
        self.query_index = self.query_index.or(Some(index));
    }

    fn set_fragment(&mut self, index: u32) {
        self.maybe_finalize_authority(index);
        self.fragment_index = self.fragment_index.or(Some(index));
    }

    fn set_path_index(&mut self, index: u32) {
        self.maybe_finalize_authority(index);
    }

    fn set_username(&mut self, index: u32) {
        self.username_index.get_or_insert(index);
        self.path_index = index + 1;
    }

    fn set_password(&mut self, index: u32) {
        self.password_index.get_or_insert(index);
        self.path_index = index + 1;
    }

    fn set_port(&mut self, index: u32) {
        self.port_index.get_or_insert(index);
        self.path_index = index + 1;
    }

    fn set_host(&mut self, index: u32) {
        if self.state.is_port() {
            self.port_index = None;
        }
        self.host_index.get_or_insert(index);
        self.path_index = index + 1;
    }

    fn maybe_finalize_authority(&mut self, index: u32) {
        if self.state.is_authority() {
            self.path_index = index;
        }
        if self.host_index.is_none() && self.username_index.is_some() {
            self.host_index = self.username_index.take();
            self.password_index = self.password_index.take();
        }
    }

    fn path_index(&self) -> usize {
        self.path_index as usize
    }

    fn path_range(&self) -> Range<usize> {
        self.path_index()..self.path_end_index()
    }
    fn path_end_index(&self) -> usize {
        self.query_index
            .or(self.fragment_index)
            .map_or_else(|| self.input.len(), |i| i as usize)
    }

    fn fragment_index(&self) -> Option<usize> {
        self.fragment_index.map(|i| i as usize)
    }
    fn host_index(&self) -> Option<usize> {
        self.host_index.map(|i| i as usize)
    }
    fn port_index(&self) -> Option<usize> {
        self.port_index.map(|i| i as usize)
    }
    fn username_index(&self) -> Option<usize> {
        self.username_index.map(|i| i as usize)
    }

    fn password_index(&self) -> Option<usize> {
        self.password_index.map(|i| i as usize)
    }

    fn query_index(&self) -> Option<usize> {
        self.query_index.map(|i| i as usize)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum UrnSchemeState {
    #[default]
    U,
    R,
    N,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AuthorityState {
    Username,
    Password,
    Host,
    Port,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum State {
    #[default]
    Head,
    LeadingSlash,
    LeadingSlash2,
    Authority(AuthorityState),
    UrlScheme,
    UrlSchemeComplete,
    UrnScheme(UrnSchemeState),
    UrnSchemeComplete,
    Path,
    Query,
    Fragment,
}
impl State {
    #[allow(clippy::match_same_arms, clippy::too_many_lines)]
    fn next(self, next: char) -> Self {
        use AuthorityState::*;
        use State::*;
        use UrnSchemeState::{N, R, U};
        match self {
            Head => match next {
                '/' => LeadingSlash,
                '?' => Query,
                '#' => Fragment,
                'u' | 'U' => UrnScheme(U),
                _ if next.is_ascii_alphabetic() => UrlScheme,
                _ => Path,
            },
            LeadingSlash => match next {
                '/' => LeadingSlash2,
                '?' => Query,
                '#' => Fragment,
                _ => Path,
            },
            LeadingSlash2 => match next {
                '/' => Path,
                '?' => Query,
                '#' => Fragment,
                _ => Authority(Username),
            },
            Authority(Username) => match next {
                '/' => Path,
                '?' => Query,
                '#' => Fragment,
                '@' => Authority(Host),
                ':' => Authority(Password),
                _ => Authority(Username),
            },
            Authority(Password) => match next {
                '/' => Path,
                '?' => Query,
                '#' => Fragment,
                '@' => Authority(Host),
                _ => Authority(Password),
            },
            Authority(Host) => match next {
                '/' => Path,
                '?' => Query,
                '#' => Fragment,
                ':' => Authority(Port),
                _ => Authority(Host),
            },
            Authority(Port) => match next {
                '/' => Path,
                '?' => Query,
                '#' => Fragment,
                _ if next.is_ascii_digit() => Authority(Port),
                _ => Authority(Host),
            },
            UrlScheme => match next {
                ':' => UrlSchemeComplete,
                '?' => Query,
                '#' => Fragment,
                _ if next.is_ascii_alphanumeric() => UrlScheme,
                _ => Path,
            },
            UrnScheme(U) => match next {
                'r' | 'R' => UrnScheme(R),
                '?' => Query,
                '#' => Fragment,
                ':' => UrlSchemeComplete,
                _ if next.is_ascii_alphanumeric() => UrlScheme,
                _ => Path,
            },
            UrnScheme(R) => match next {
                'n' | 'N' => UrnScheme(N),
                '?' => Query,
                '#' => Fragment,
                ':' => UrlSchemeComplete,
                _ if next.is_ascii_alphanumeric() => UrlScheme,
                _ => Path,
            },
            UrnScheme(N) => match next {
                ':' => UrnSchemeComplete,
                '?' => Query,
                '#' => Fragment,
                _ if next.is_ascii_alphanumeric() => UrlScheme,
                _ => Path,
            },
            Path => match next {
                '?' => Query,
                '#' => Fragment,
                _ => Path,
            },
            Query => match next {
                '#' => Fragment,
                _ => Query,
            },
            Fragment => Fragment,
            _ => panic!("invalid state tranisition: \'{next:?}\' -> {self:?}"),
        }
    }

    fn is_authority(self) -> bool {
        matches!(self, Self::Authority(_))
    }

    fn is_port(self) -> bool {
        matches!(self, Self::Authority(AuthorityState::Port))
    }
}

#[cfg(test)]
mod tests {
    use super::UrnSchemeState::{N, R, U};
    use super::*;
    use std::panic;
    use AuthorityState::*;
    use State::*;
    #[derive(Debug, Default, PartialEq, Eq)]
    enum Expect {
        #[default]
        RelativeUri,
        Url,
        Urn,
    }
    #[derive(Default, Debug)]
    struct Test<'a> {
        input: &'a str,
        // error: Option<>,
        // expect: &'static str,
        kind: Expect,
        username: Option<&'a str>,
        password: Option<&'a str>,
        host: Option<&'a str>,
        port: Option<u16>,
        path: &'a str,
        query: Option<&'a str>,
        fragment: Option<&'a str>,
    }

    impl Test<'_> {
        fn run(&self) {
            let uri = uri(self.input).unwrap();
            let input = self.input;
            match uri {
                Uri::Relative(uri) => {
                    assert_eq!(
                        uri.host(),
                        self.host,
                        "\n\n* input: {input:?}\n* expected host: {:?}\n* received: {:?}\nuri: {uri:#?}\n\n",
                        self.host,
                        uri.host()
                    );
                    assert_eq!(
                        uri.path(),
                        self.path,
                        "\n\n* input: {input:?}\n* expected path: {:?}\n* received: {:?}\nuri: {uri:#?}\n\n",
                        self.path,
                        uri.path()
                    );
                    assert_eq!(
                        uri.query(),
                        self.query,
                        "\n\n* input: {input:?}\n* expected query: {:?}\n* received: {:?}\nuri: {uri:#?}\n\n",
                        self.query,
                        uri.query()
                    );
                    assert_eq!(
                        uri.fragment(),
                        self.fragment,
                        "\n\n* input: {input:?}\n* expected fragment: {:?}\n* received: {:?}\nuri: {uri:#?}\n\n",
                        self.fragment,
                        uri.fragment()
                    );
                    assert_eq!(
                        uri.port(),
                        self.port,
                        "\n\n* input: {input:?}\n* expected port: {:?}\n* received: {:?}\nuri: {uri:#?}\n\n",
                        self.port,
                        uri.port()
                    );
                    assert_eq!(
                        uri.username(),
                        self.username,
                        "\n\n* input: {input:?}\n* expected username: {:?}\n* received: {:?}\nuri: {uri:#?}\n\n",
                        self.username,
                        uri.username()
                    );
                    assert_eq!(
                        uri.password(),
                        self.password,
                        "\n\n* input: {input:?}\n* expected password: {:?}\n* received: {:?}\nuri: {uri:#?}\n\n",
                        self.password,
                        uri.password()
                    );
                }
                Uri::Url(url) => {
                    _ = url; // todo: test url
                    assert_eq!(Expect::Url, self.kind);
                }
                Uri::Urn(urn) => {
                    _ = urn; // todo: test urn
                    assert_eq!(Expect::Urn, self.kind);
                }
            }
        }
    }
    #[test]
    fn test_parse_uri() {
        let tests = [
            Test {
                input: "path/to/file",
                path: "path/to/file",
                ..Default::default()
            },
            Test {
                input: "//www.example.com",
                kind: Expect::RelativeUri,
                path: "",
                host: Some("www.example.com"),
                ..Default::default()
            },
            Test {
                input: "http://www.example.com",
                kind: Expect::Url,
                path: "",
                host: Some("www.example.com"),
                ..Default::default()
            },
            Test {
                input: "/example/path",
                kind: Expect::RelativeUri,
                path: "/example/path",
                ..Default::default()
            },
            Test {
                input: "//user:pass@domain:1111/path?query=string#fragment",
                kind: Expect::RelativeUri,
                username: Some("user"),
                password: Some("pass"),
                host: Some("domain"),
                port: Some(1111),
                path: "/path",
                fragment: Some("fragment"),
                query: Some("query=string"),
            },
        ];
        tests.iter().for_each(Test::run);
    }

    #[test]
    fn test_state_single_transitions() {
        type Test = (State, Box<dyn Fn(char) -> State>);
        let tests: &[Test] = &[
            (
                Head,
                b(|c| match c {
                    '/' => LeadingSlash,
                    '?' => Query,
                    '#' => Fragment,
                    'u' | 'U' => UrnScheme(U),
                    c if c.is_ascii_alphabetic() => UrlScheme,
                    _ => Path,
                }),
            ),
            (
                LeadingSlash,
                b(|c| match c {
                    '/' => LeadingSlash2,
                    '?' => Query,
                    '#' => Fragment,
                    _ => Path,
                }),
            ),
            (
                LeadingSlash2,
                b(|c| match c {
                    '/' => Path,
                    '?' => Query,
                    '#' => Fragment,
                    _ => Authority(Username),
                }),
            ),
            (
                Authority(Username),
                b(|c| match c {
                    '/' => Path,
                    '?' => Query,
                    '#' => Fragment,
                    '@' => Authority(Host),
                    ':' => Authority(Password),
                    _ => Authority(Username),
                }),
            ),
            (
                Authority(Host),
                b(|c| match c {
                    '/' => Path,
                    '?' => Query,
                    '#' => Fragment,
                    ':' => Authority(Port),
                    _ => Authority(Host),
                }),
            ),
            (
                Authority(Port),
                b(|c| match c {
                    '/' => Path,
                    '?' => Query,
                    '#' => Fragment,
                    _ if c.is_ascii_digit() => Authority(Port),
                    _ => Authority(Host),
                }),
            ),
            (
                UrlScheme,
                b(|c| match c {
                    '?' => Query,
                    '#' => Fragment,
                    ':' => UrlSchemeComplete,
                    _ if c.is_ascii_alphanumeric() => UrlScheme,
                    _ => Path,
                }),
            ),
            (
                UrnScheme(U),
                b(|c| match c {
                    'r' | 'R' => UrnScheme(R),
                    '?' => Query,
                    '#' => Fragment,
                    ':' => UrlSchemeComplete,
                    _ if c.is_ascii_alphanumeric() => UrlScheme,
                    _ => Path,
                }),
            ),
            (
                UrnScheme(R),
                b(|c| match c {
                    'n' | 'N' => UrnScheme(N),
                    '?' => Query,
                    '#' => Fragment,
                    ':' => UrlSchemeComplete,
                    _ if c.is_ascii_alphanumeric() => UrlScheme,
                    _ => Path,
                }),
            ),
            (
                UrnScheme(N),
                b(|c| match c {
                    '?' => Query,
                    '#' => Fragment,
                    ':' => UrnSchemeComplete,
                    _ if c.is_ascii_alphanumeric() => UrlScheme,
                    _ => Path,
                }),
            ),
            (
                Query,
                b(|c| match c {
                    '#' => Fragment,
                    _ => Query,
                }),
            ),
            (Fragment, b(|_| Fragment)),
        ];
        for (state, expect) in tests {
            assert_next_state(*state, expect);
        }
        assert_next_state_panics(UrlSchemeComplete);
        assert_next_state_panics(UrnSchemeComplete);
    }

    #[test]
    fn test_state_changes() {
        let tests = [
            (Head, "http", UrlScheme),
            (Head, "http:", UrlSchemeComplete),
            (Head, "h@ttp", Path),
            (Head, "u", UrnScheme(U)),
            (Head, "ur", UrnScheme(R)),
            (Head, "urn", UrnScheme(N)),
            (Head, "urn:", UrnSchemeComplete),
            (Head, "urx", UrlScheme),
            (Head, "u:", UrlSchemeComplete),
            (Head, "u@", Path),
            (Head, "/", LeadingSlash),
            (Head, "//", LeadingSlash2),
            (Head, "//u", Authority(Username)),
            (Head, "//username:", Authority(Password)),
            (Head, "//username:password", Authority(Password)),
            (Head, "//username:password@", Authority(Host)),
            (Head, "//username@", Authority(Host)),
            (Head, "//username", Authority(Username)),
            (Head, "//username:password@host:", Authority(Port)),
            (Head, "//username:password@host:1", Authority(Port)),
            (Head, "//username:password@host:12", Authority(Port)),
            (Head, "//username:password@host:123", Authority(Port)),
            (Head, "//username:password@host:1234", Authority(Port)),
            (Head, "//username:password@host:12345", Authority(Port)),
            (Head, "//username:password@host:123456", Authority(Port)),
            (Head, "///", Path),
            (Head, "?", Query),
            (Head, "#", Fragment),
            (Head, ":#", Fragment),
            (Head, "//?", Query),
        ];
        for (state, input, expected) in &tests {
            assert_state_change(*state, input, *expected);
        }
    }

    /*                        *
     *                        *
     * Helpers and assertions *
     * ********************** *
     *                        */

    /// Boxes t
    #[allow(clippy::unnecessary_box_returns)]
    fn b<T: 'static>(t: T) -> Box<T> {
        Box::new(t)
    }

    fn assert_next_state<F>(state: State, expect: F)
    where
        F: Fn(char) -> State,
    {
        // decent sized sampling
        for i in 0..70_000 {
            let Some(input) = char::from_u32(i) else {
                continue;
            };
            assert_transition(state, input, expect(input));
        }
    }

    fn assert_state_change(state: State, input: &str, expected: State) {
        let result = input.chars().fold(state, State::next);
        assert_eq!(
            result, expected,
            "\n\ninput:\t\t\'{input:?}\'\nexpected:\t{expected:?}\nresult:\t\t{result:?}\n\n"
        );
    }

    fn assert_transition(state: State, input: char, expected: State) {
        let result = state.next(input);
        assert_eq!(
            result, expected,
            "\n\nstate: {state:?}\ninput: \'{input:?}\'\nexpected: {expected:?}\nresult: {result:?}\n\n"
        );
    }

    fn assert_failed_transition(state: State, input: char) {
        assert!(
            panic::catch_unwind(|| state.next(input)).is_err(),
            "state: {state:?}\ninput: \'{input:?}\'\nexpected: panic\nresult: {:?}\n* \n",
            state.next(input)
        );
    }

    fn assert_next_state_panics(state: State) {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        for i in 0..255 {
            let input = char::from(i);
            assert_failed_transition(state, input);
        }
        panic::set_hook(prev_hook);
    }
}
