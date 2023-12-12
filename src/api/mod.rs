pub mod auth;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Api {
    pub host: String,
    pub auth: auth::Auth,
}

impl Api {
    pub fn new(host: String, auth: auth::Auth) -> Self {
        Self { host, auth }
    }
}
