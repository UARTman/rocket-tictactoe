use jsonwebtoken::{Header, encode, decode, EncodingKey, Validation, DecodingKey};
use rocket::{request::{FromRequest, self}, async_trait, Request, State};
use rocket_okapi::{request::{OpenApiFromRequest, RequestHeaderInput}, okapi::openapi3::{SecurityScheme, SecuritySchemeData, Object, SecurityRequirement}};
use serde::{Serialize, Deserialize};

pub struct Secret(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // exp: usize,
    pub username: String,
    pub exp: usize,
}

#[allow(dead_code)]
pub fn encode_token(claims: &Claims, secret: &str) -> Option<String> {
    encode(&Header::default(), claims, &EncodingKey::from_secret(secret.as_ref())).ok()
}

#[async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let secret_state: &State<Secret> = FromRequest::from_request(req).await.succeeded().unwrap(); 
        let mut validation = Validation::default();
        validation.validate_exp = false;
        if let Some(auth_header) = req.headers().get_one("Authorization") {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                let claims = decode::<Claims>(token, &DecodingKey::from_secret(secret_state.0.as_ref()), &validation);
                if let Ok(data) = claims {
                    request::Outcome::Success(data.claims)
                } else {
                    request::Outcome::Forward(())
                }
            } else {
                request::Outcome::Forward(())
            }
        } else {
            request::Outcome::Forward(())
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for Claims {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Requres a Bearer JWT token in Authorization header to accept".to_owned()),
            data: SecuritySchemeData::Http { scheme: "bearer".to_owned(), bearer_format: Some("Bearer".to_owned()) },
            extensions: Object::default(),
        };
        let mut security_req = SecurityRequirement::new();
        security_req.insert("JWTAuth".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "JWTAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}