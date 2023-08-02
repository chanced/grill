use std::{ops::Range, str::FromStr};

use super::{encode, to_u32, write, RelativeUri, Uri};
use crate::error::{RelativeUriError, UriError};
use url::Url;
use urn::Urn;

pub(super) fn uri(value: &str) -> Result<Uri, UriError> {
    Parse::uri(value)
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
        let current = self;

        let new_state = match self {
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
        };
        println!("{next} -> {current:?} = {new_state:?}");
        new_state
    }

    fn is_authority(self) -> bool {
        matches!(self, Self::Authority(_))
    }
    fn is_port(self) -> bool {
        matches!(self, Self::Authority(AuthorityState::Port))
    }
}

#[derive(Default, Debug)]
struct Parse<'a> {
    state: State,
    input: &'a str,
    path_idx: u32,
    username_idx: Option<u32>,
    password_idx: Option<u32>,
    host_idx: Option<u32>,
    port_idx: Option<u32>,
    query_idx: Option<u32>,
    fragment_idx: Option<u32>,
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
            .unwrap_or_else(|| parser.finalize())
    }

    fn finalize(mut self) -> Result<Uri, UriError> {
        if self.username_idx.is_some() && self.host().is_none() && self.password().is_none() {
            self.host_idx = self.username_idx.take();
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

        println!("username: {username:?}");
        println!("password: {password:?}");
        println!("host: {host:?}");
        println!("port: {port:?}");
        println!("path: {path:?}");
        println!("query: {query:?}");
        println!("fragment: {fragment:?}");

        let has_path = !path.is_empty();
        let has_authority = username.is_some() || host.is_some();
        let username_idx = write::username(&mut buf, username)?;
        let password_idx = write::password(&mut buf, password)?;
        let host_idx = write::host(&mut buf, host)?;
        let port_idx = write::port(&mut buf, port_str)?;
        let path_idx = write::path(&mut buf, path)?;
        let query_idx = write::query(&mut buf, query, has_authority, has_path)?;
        let fragment_idx = write::fragment(&mut buf, fragment)?;

        Ok(RelativeUri {
            href: buf,
            username_idx,
            password_idx,
            host_idx,
            port,
            port_idx,
            path_idx,
            query_idx,
            fragment_idx,
        }
        .into())
    }

    fn next(&mut self, index: usize, next: char) -> Option<Result<Uri, UriError>> {
        use State::*;
        let index = match to_u32(index) {
            Ok(i) => i,
            Err(err) => return Some(Err(err.into())),
        };
        let next_state = self.state.next(next);
        match next_state {
            Authority(next_auth_state) => self.next_authority(index, next_auth_state),
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

    fn parse_url(&mut self) -> Option<Result<Uri, UriError>> {
        Url::parse(self.input)
            .map(Uri::Url)
            .map_err(UriError::Url)
            .into()
    }

    fn parse_urn(&mut self) -> Option<Result<Uri, UriError>> {
        Urn::from_str(self.input)
            .map(Uri::Urn)
            .map_err(UriError::Urn)
            .into()
    }

    fn host(&self) -> Option<&str> {
        let end = self.port_idx().unwrap_or(self.path_idx());
        self.host_idx().map(|i| &self.input[i + 1..end])
    }

    fn port(&self) -> Option<Result<u16, UriError>> {
        let port = self.port_str()?;
        port.parse::<u16>()
            .map_err(|_| RelativeUriError::PortOverflow(port.to_string()).into())
            .into()
    }

    fn port_str(&self) -> Option<&str> {
        self.port_idx().map(|i| &self.input[i + 1..self.path_idx()])
    }

    fn path(&self) -> &str {
        &self.input[self.path_range()]
    }

    fn username(&self) -> Option<&str> {
        self.username_idx()
            .map(|i| &self.input[i..self.password_idx().unwrap_or(self.path_idx())])
    }

    fn password(&self) -> Option<&str> {
        let end = self.host_idx().unwrap_or(self.path_idx());
        self.password_idx().map(|i| &self.input[i + 1..end])
    }

    fn query(&self) -> Option<&str> {
        self.query_idx()
            .map(|i| &self.input[i + 1..self.fragment_idx().unwrap_or(self.input.len())])
    }

    fn fragment(&self) -> Option<&str> {
        self.fragment_idx()
            .map(|i| &self.input[i..self.input.len()])
    }

    fn set_query(&mut self, index: u32) {
        self.maybe_finalize_authority(index);
        self.query_idx = self.query_idx.or(Some(index));
    }

    fn set_fragment(&mut self, index: u32) {
        self.maybe_finalize_authority(index);
        self.fragment_idx = self.fragment_idx.or(Some(index));
    }

    fn set_path_index(&mut self, index: u32) {
        self.maybe_finalize_authority(index);
    }

    fn next_authority(&mut self, index: u32, next_auth_state: AuthorityState) {
        use AuthorityState::*;
        match next_auth_state {
            Username => self.set_username(index),
            Password => self.set_password(index),
            Host => self.set_host(index),
            Port => self.set_port(index),
        }
        self.path_idx = index + 1;
    }

    fn set_username(&mut self, index: u32) {
        self.username_idx.get_or_insert(index);
    }

    fn set_password(&mut self, index: u32) {
        self.password_idx.get_or_insert(index);
    }

    fn set_port(&mut self, index: u32) {
        self.port_idx.get_or_insert(index);
    }

    fn set_host(&mut self, index: u32) {
        if self.state.is_port() {
            self.port_idx = None;
        }
        self.host_idx.get_or_insert(index);
    }

    fn maybe_finalize_authority(&mut self, index: u32) {
        if self.state.is_authority() {
            self.path_idx = index;
        }
        if self.host_idx.is_none() && self.username_idx.is_some() {
            self.host_idx = self.username_idx;
            self.username_idx = None;
        }
    }

    fn path_idx(&self) -> usize {
        self.path_idx as usize
    }

    fn path_range(&self) -> Range<usize> {
        self.path_idx()..self.path_end_idx()
    }
    fn path_end_idx(&self) -> usize {
        self.query_idx
            .or(self.fragment_idx)
            .map_or_else(|| self.input.len(), |i| i as usize)
    }

    fn fragment_idx(&self) -> Option<usize> {
        self.fragment_idx.map(|i| i as usize)
    }
    fn host_idx(&self) -> Option<usize> {
        self.host_idx.map(|i| i as usize)
    }
    fn port_idx(&self) -> Option<usize> {
        self.port_idx.map(|i| i as usize)
    }
    fn username_idx(&self) -> Option<usize> {
        self.username_idx.map(|i| i as usize)
    }

    fn password_idx(&self) -> Option<usize> {
        self.password_idx.map(|i| i as usize)
    }

    fn query_idx(&self) -> Option<usize> {
        self.query_idx.map(|i| i as usize)
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
            // Test {
            //     input: "//www.example.com",
            //     kind: Expect::RelativeUri,
            //     path: "",
            //     host: Some("www.example.com"),
            //     ..Default::default()
            // },
            // Test {
            //     input: "http://www.example.com",
            //     kind: Expect::Url,
            //     path: "",
            //     host: Some("www.example.com"),
            //     ..Default::default()
            // },
            // Test {
            //     input: "/example/path",
            //     kind: Expect::RelativeUri,
            //     path: "/example/path",
            //     ..Default::default()
            // },
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
            (Head, "//username:password@host:123456", Authority(Host)),
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
    fn b<T: 'static>(t: T) -> Box<T> {
        Box::new(t)
    }

    fn assert_next_state<F>(state: State, expect: F)
    where
        F: Fn(char) -> State,
    {
        // decent sized sampling
        for i in 0..70_000 {
            let Some(input) = char::from_u32(i) else { continue };
            assert_transition(state, input, expect(input));
        }
    }

    fn assert_state_change(state: State, input: &str, expected: State) {
        let result = input.chars().fold(state, State::next);
        assert_eq!(
            result, expected,
            "input: \'{input:?}\'\nexpected: {expected:?}\nresult: {result:?}\n\n"
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
