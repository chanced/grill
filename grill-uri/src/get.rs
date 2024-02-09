pub(super) mod url {
    use url::Url;

    pub(crate) fn authority(u: &Url) -> Option<String> {
        if !u.has_authority() {
            return None;
        }
        let host = u.host();
        let port = u.port();
        let username = u.username();
        let password = u.password();
        let mut prev_authority = String::new();
        if !username.is_empty() {
            prev_authority.push_str(username);
            if let Some(password) = password {
                prev_authority.push(':');
                prev_authority.push_str(password);
            }
        }
        if let Some(host) = host {
            if !prev_authority.is_empty() {
                prev_authority.push('@');
            }
            prev_authority.push_str(host.to_string().as_str());
        }
        if let Some(port) = port {
            if !prev_authority.is_empty() {
                prev_authority.push(':');
            }
            prev_authority.push_str(&port.to_string());
        }
        Some(prev_authority)
    }
}
