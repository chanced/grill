use super::{encode, parse};

pub(crate) mod urn {
    use super::encode;
    use crate::error::UriError;
    use urn::Urn;

    pub(crate) fn fragment(
        urn: &mut Urn,
        fragment: Option<&str>,
    ) -> Result<Option<String>, UriError> {
        let existing = urn.f_component().map(ToString::to_string);
        // safety: encode_f_component does not currently return an error.
        let fragment = fragment.map(encode::f_component).map(Result::unwrap);
        urn.set_f_component(fragment.as_deref())?;
        Ok(existing)
    }

    pub(crate) fn nss(urn: &mut Urn, nss: &str) -> Result<String, UriError> {
        let existing = urn.nss().to_string();
        urn.set_nss(&encode::nss(nss)?)?;
        Ok(existing)
    }

    pub(crate) fn namespace(u: &mut Urn, namespace: &str) -> Result<Option<String>, UriError> {
        let prev_namespace = u.nid().to_string();
        u.set_nid(namespace)?;
        Ok(Some(prev_namespace))
    }
}

pub(crate) mod url {
    use crate::error::{AuthorityError, UriError};

    use super::parse;
    use snafu::Backtrace;
    use url::Url;
    pub(crate) fn fragment(url: &mut Url, fragment: Option<&str>) -> Option<String> {
        let existing = url.fragment().map(ToString::to_string);
        url.set_fragment(fragment);
        existing
    }
    pub(crate) fn path(url: &mut Url, path: &str) -> String {
        let existing = url.path().to_string();
        url.set_path(path);
        existing
    }

    pub(crate) fn authority(u: &mut Url, authority: &str) -> Result<Option<String>, UriError> {
        let prev_authority = crate::get::url::authority(u);
        let authority = parse::authority(authority)?;
        if u.set_username(authority.username().unwrap_or_default())
            .is_err()
        {
            // the url crate doesn't check for empty values before returning `Err(())`
            // https://github.com/servo/rust-url/issues/844
            let username = authority.username().unwrap_or_default();
            if !username.is_empty() {
                return Err(AuthorityError::UsernameNotAllowed {
                    value: username.to_string().into(),
                    scheme: u.scheme().to_string(),
                    backtrace: Backtrace::capture(),
                }
                .into());
            }
        }
        if u.set_password(authority.password()).is_err() {
            // the url crate doesn't check for empty values before returning `Err(())`
            // https://github.com/servo/rust-url/issues/844
            let password = authority.password().unwrap_or_default();
            if !password.is_empty() {
                return Err(AuthorityError::PasswordNotAllowed {
                    scheme: u.scheme().to_string(),
                    value: password.to_string(),
                    backtrace: Backtrace::capture(),
                }
                .into());
            }
        }
        u.set_host(authority.host())?;
        if u.set_port(authority.port()).is_err() {
            // the url crate doesn't check for empty values before returning `Err(())`
            // https://github.com/servo/rust-url/issues/844
            if let Some(port) = authority.port() {
                return Err(AuthorityError::PortNotAllowed {
                    port,
                    scheme: u.scheme().to_string(),
                }
                .into());
            }
        }
        Ok(prev_authority)
    }
}
