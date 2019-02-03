/*
 *  Copyright 2019 Brandon Arrendondo
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a
 *  copy of this software and associated documentation files (the "Software"),
 *  to deal in the Software without restriction, including without limitation
 *  the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
 *  Software is furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in
 *  all copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 *  THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 *  FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 *  DEALINGS IN THE SOFTWARE.
 */
use hyper;
use hyper::server::{Request, Response, Service};
use hyper::{StatusCode};
use hyper::Method::{Get, Post};
use hyper::header::{Authorization, Bearer, ContentLength, ContentType, AccessControlAllowOrigin};

use serde_json;
use valico::json_schema;

use futures;
use futures::Stream;
use futures::future::{Future};

use std::collections::HashMap;

lazy_static! {
static ref SCHEMAS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("/auth", ::schemas::auth::AUTH_REQUEST_SCHEMA);
        m.insert("/auth/renew", ::schemas::auth::AUTH_RENEW_SCHEMA);
        return m;
    };
}

fn json_request_is_valid(payload: &hyper::Chunk, api: &str) -> bool {
    match serde_json::from_slice(payload) {
        Ok(json_payload) => {
            let mut scope = json_schema::Scope::new();
            let schema = SCHEMAS.get(&api).unwrap().clone();
            let schema = serde_json::from_str(schema).unwrap();
            let compiled_schema = scope.compile_and_return(schema, false).unwrap();
            return compiled_schema.validate(&json_payload).is_valid();
        }
        Err(_e) => {
            false
        }
    }
}

fn json_build_invalid_request_response() -> Response {
    let payload = json!({
        "request-is-valid": false
    }).to_string();

    Response::new()
        .with_header(ContentLength(payload.len() as u64))
        .with_header(ContentType::json())
        .with_body(payload)
}

pub const EXAMPLE_SPACE : &str = r#"
{
    "bound": {
        "xMax": 400,
        "yMax": 400
    },
    "planets": [
        {
            "id": 0,
            "name": "Mohlodi",
            "loc": {
                "x": 120,
                "y": 50
            },
            "seenBefore": false
        },
        {
            "id": 1,
            "name": "Strange World",
            "loc": {
                "x": 300,
                "y": 75
            },
            "seenBefore": true,
            "currentData": true,
            "relatedStarbase": true,
            "population": 30000,
            "currentHabVal": 75,
            "potentialHabVal": 100,
            "relatedOwnedFleets": [
                "Santa Maria #2",
                "Long Range Scout #3"
            ],
            "relatedFriendlyFleets": [
                "Some Fleet #23"
            ],
            "relatedEnemyFleets": [
                "Bombing Crew #40"
            ]
        }
    ]
}
"#;

fn get_space_details(query: &str) -> Response {
    let args = url::form_urlencoded::parse(&query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    match args.get("gid") {
        Some(game_id) => {
            let payload = EXAMPLE_SPACE.to_string();

            Response::new()
                .with_header(ContentLength(payload.len() as u64))
                .with_header(ContentType::json())
                .with_header(AccessControlAllowOrigin::Any) // TODO: replace this for security
                .with_body(payload)
        }
        None => {
            json_build_invalid_request_response()
        }
    }
}

pub struct FrontendService {
}

pub fn auth_token_is_valid(headers: &hyper::header::Headers) -> bool {
    return true;
    let response = headers.get::<Authorization<Bearer>>();
    return match response {
        Some(header) => {
            ::db::retrieval::validate_token(&header.token)
        }
        None => {
            false
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct RenewPayload {
    pub renewToken: String
}

impl Service for FrontendService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        info!("Received request: {:?}", request);
        info!("Request headers: {:?}", request.headers());

        match (request.method(), request.path()) {
            (&Get, "/version") => {
                let payload = json!({
                    "version": env!("CARGO_PKG_VERSION").to_string()
                }).to_string();

                let response = Response::new()
                    .with_header(ContentLength(payload.len() as u64))
                    .with_header(ContentType::json())
                    .with_body(payload);

                Box::new(futures::future::ok(response))
            },
            (&Get, "/turn/space") => {
                if auth_token_is_valid(request.headers()) {
                    let response = match request.query() {
                        Some(query) => {
                            info!("getting space details");
                            get_space_details(query)
                        }
                        None => {
                            json_build_invalid_request_response()
                        }
                    };

                    Box::new(futures::future::ok(response))
                }
                else {
                    let response = Response::new()
                        .with_status(StatusCode::Unauthorized);

                    Box::new(futures::future::ok(response))
                }
           },
            (&Post, "/auth/renew") => {
                let future = request.body().concat2().and_then( |body| {
                    if json_request_is_valid(&body, "/auth/renew") {

                        let renew : RenewPayload = serde_json::from_slice(&body).unwrap();
                        let result = ::db::storage::renew_auth(&renew.renewToken);
                        let payload = match result {
                            Some(credentials) => {
                                let (token, renew_token, expires) = credentials;
                                json!({
                                    "authToken": token,
                                    "renewToken": renew_token,
                                    "expires": expires
                                }).to_string()
                            },
                            None => {
                                json!({
                                    "requestIsValid": true,
                                    "errorReason": "authentication failure"
                                }).to_string()
                            }
                        };
 
                        let response = Response::new()
                            .with_header(ContentLength(payload.len() as u64))
                            .with_header(ContentType::json())
                            .with_body(payload);

                        futures::future::ok(response)
                    }
                    else {
                        futures::future::ok(json_build_invalid_request_response())
                    }
                });
                Box::new(future)
            },
            (&Post, "/auth") => {
                let future = request.body().concat2().and_then( |body| {
                    if json_request_is_valid(&body, "/auth") {
                        let auth : AuthPayload = serde_json::from_slice(&body).unwrap();
                        let result = ::db::retrieval::is_auth_valid(&auth.username, &auth.password);

                        let payload = match result {
                            true => {
                                let (token, renew_token, expires) = ::db::storage::create_auth_token(&auth.username).unwrap();
                                json!({
                                    "authToken": token,
                                    "renewToken": renew_token,
                                    "expires": expires
                                }).to_string()
                            },
                            false => {
                              json!({
                                    "requestIsValid": true,
                                    "errorReason": "authentication failure"
                                }).to_string()
                            }
                        };

                        let response = Response::new()
                            .with_header(ContentLength(payload.len() as u64))
                            .with_header(ContentType::json())
                            .with_body(payload);

                        futures::future::ok(response)
                    }
                    else {
                        futures::future::ok(json_build_invalid_request_response())
                    }
                });
                Box::new(future)
            },
            _=> Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
                )),
        }
    }
}
