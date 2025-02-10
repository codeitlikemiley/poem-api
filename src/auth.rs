use poem::Request;
use poem_openapi::{auth::Bearer, SecurityScheme};
use serde::{Deserialize, Serialize};

use hmac::Hmac;
use jwt::VerifyWithKey;
use sha2::Sha256;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub username: String,
}

pub type ServerKey = Hmac<Sha256>;

#[derive(SecurityScheme)]
#[oai(ty = "bearer", checker = "verify_token")]
pub struct Authenticate(pub AuthUser);

async fn verify_token(req: &Request, bearer: Bearer) -> Option<AuthUser> {
    let server_key = req.data::<ServerKey>().unwrap();

    VerifyWithKey::<AuthUser>::verify_with_key(bearer.token.as_str(), server_key).ok()
}
