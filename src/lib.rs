use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2, Params, Version,
};
use worker::{
    Context, Env, Headers, Method, Request, Response,
};

#[worker::event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> worker::Result<Response> {
    let result = match (req.method(), req.path().as_ref()) {
        // argon2 defaults to argon2id
        (Method::Post, "/argon2/hash") => argon2id_hash_handler(req).await,
        (Method::Post, "/argon2/verify") => argon2id_verify_handler(req).await,
        // TODO: maybe add the remaining argons
        (Method::Post, "/bcrypt/hash") => bcrypt_hash_handler(req).await,
        (Method::Post, "/bcrypt/verify") => bcrypt_verify_handler(req).await,
        _ => Err(Error::InvalidRoute),
    };

    let mut res_headers = Headers::new();
    res_headers.set("Content-Type", "application/json")?;

    match result {
        Ok(body) => Ok(Response::ok(body)?.with_headers(res_headers)),
        Err(err) => err.to_response(),
    }
}

// ## Hash
// ### Types
#[derive(serde::Deserialize)]
pub struct HashRequest<T> {
    pub password: String,
    pub options: Option<T>,
}

#[derive(serde::Serialize)]
pub struct HashResponse {
    pub hash: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Argon2HashOptions {
    pub time_cost: u32,
    pub memory_cost: u32,
    pub parallelism: u32,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BcryptHashOptions {
    pub work_factor: u32,
}

// ### Functions
async fn argon2id_hash_handler(mut req: Request) -> Result<String, Error> {
    let hash_req: HashRequest<Argon2HashOptions> = req
        .json()
        .await
        .map_err(|_err| Error::BadRequest)?;

    let password_hash = argon2id_hash(&hash_req.password, hash_req.options)?;

    let hash_response = HashResponse {
        hash: password_hash,
    };
    serde_json::to_string(&hash_response).map_err(|_err| Error::InternalServerError)
}

fn argon2id_hash(password: &str, options: Option<Argon2HashOptions>) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = match options {
        Some(opts) => {
            let params = Params::new(
                opts.memory_cost,
                opts.time_cost,
                opts.parallelism,
                None,
            ).map_err(|_err| Error::InvalidHashOptions)?;

            Ok(Argon2::new(
                argon2::Algorithm::Argon2id,
                Version::default(),
                params,
            ))
        }

        None => Ok(Argon2::default()),
    }?;

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|password_hash| password_hash.to_string())
        .map_err(|_err| Error::HashFailed)
}

async fn bcrypt_hash_handler(mut req: Request) -> Result<String, Error> {
    let hash_req: HashRequest<BcryptHashOptions> = req
        .json()
        .await
        .map_err(|_err| Error::BadRequest)?;

    let password_hash = match hash_req.options {
        Some(opts) => Ok(bcrypt::hash(&hash_req.password, opts.work_factor)),
        None => Ok(bcrypt::hash(&hash_req.password, bcrypt::DEFAULT_COST)),
    }?;

    let hash = password_hash
        .map(|hash| hash.to_string())
        .map_err(|_err| Error::HashFailed)?;

    let hash_response = HashResponse {
        hash: hash,
    };
    serde_json::to_string(&hash_response).map_err(|_err| Error::InternalServerError)
}

// ## Verify
// ### Types
#[derive(serde::Deserialize)]
pub struct VerifyRequest {
    pub password: String,
    pub hash: String,
}

#[derive(serde::Serialize)]
pub struct VerifyResponse {
    pub result: bool,
}

// ### Functions
async fn argon2id_verify_handler(mut req: Request) -> Result<String, Error> {
    let options: VerifyRequest = req
        .json()
        .await
        .map_err(|_err| Error::BadRequest)?;

    let result = argon2id_verify(&options)?;
    let verify_response = VerifyResponse { result };
    serde_json::to_string(&verify_response).map_err(|_err| Error::InternalServerError)
}

fn argon2id_verify(options: &VerifyRequest) -> Result<bool, Error> {
    let password_hash = PasswordHash::new(&options.hash)
        .map_err(|_err| Error::InvalidPasswordHash)?;

    let argon2 = Argon2::default();

    match argon2.verify_password(options.password.as_bytes(), &password_hash) {
        Ok(()) => Ok(true),

        Err(err) => match err {
            argon2::password_hash::Error::Password => Ok(false),
            _ => Err(Error::VerifyFailed),
        },
    }
}

async fn bcrypt_verify_handler(mut req: Request) -> Result<String, Error> {
    let options: VerifyRequest = req
        .json()
        .await
        .map_err(|_err| Error::BadRequest)?;

    let result = bcrypt::verify(options.password, &options.hash).map_err(|_err| Error::VerifyFailed)?;
    let verify_response = VerifyResponse { result };
    serde_json::to_string(&verify_response).map_err(|_err| Error::InternalServerError)
}

// ## Error handling
enum Error {
    InvalidRoute,
    BadRequest,
    InternalServerError,
    InvalidHashOptions,
    HashFailed,
    InvalidPasswordHash,
    VerifyFailed,
}

impl Error {
    fn to_response(&self) -> worker::Result<Response> {
        match self {
            Error::InvalidRoute => Response::error("Not found.", 404),
            Error::BadRequest => Response::error("Bad request.", 400),
            Error::InternalServerError => Response::error("Internal server error.", 500),
            Error::InvalidHashOptions => Response::error("Invalid option for specified hash algorithm.", 400),
            Error::HashFailed => Response::error("Hash failed.", 500),
            Error::InvalidPasswordHash => Response::error("Invalid hash", 400),
            Error::VerifyFailed => Response::error("Verification failed.", 500),
        }
    }
}
