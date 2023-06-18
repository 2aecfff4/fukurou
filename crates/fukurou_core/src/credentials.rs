///
#[derive(Debug, Clone)]
pub enum Credentials {
    EmailPassword { email: String, password: String },
    LoginPassword { login: String, password: String },
    Token { token: String },
}
