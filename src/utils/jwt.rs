use jwt::{decode, encode, Header, Validation};
use chrono::{Local, Duration};
use actix_web::HttpResponse;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    name: String,
    fullname: String,
    exp: usize
}

pub struct SlimUser {
    pub id: i32,
    pub email: String,
    pub fullname: String
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            id: claims.sub,
            email: claims.name,
            fullname: claims.fullname
        }
    }
}

impl Claims {
    fn with_email(id: i32, email: &str, fullname: &str) -> Self {
        Claims {
            sub: id,
            name: email.into(),
            fullname: fullname.into(),
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize
        }
    }
}

pub fn create_token(id: i32, email: &str, fullname: &str) -> Result<String, HttpResponse> {
    let claims = Claims::with_email(id, email, fullname);
    encode(&Header::default(), &claims, get_secret())
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
    decode::<Claims>(token, get_secret(), &Validation::default())
        .map(|data| data.claims.into())
        .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}

fn get_secret<'a>() -> &'a [u8] {
    dotenv!("JWT_SECRET").as_bytes()
}