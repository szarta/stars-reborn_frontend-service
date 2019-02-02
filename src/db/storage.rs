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
use sqlite::State;
use configuration;
use argonautica::Hasher;
use uuid::Uuid;
use chrono::prelude::*;
use time::Duration;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn create_db_if_necessary() {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();
    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, username TEXT NOT NULL, password_hash TEXT NOT NULL)
        "
    ).unwrap();

    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS tokens (id INTEGER PRIMARY KEY NOT NULL, uid INTEGER NOT NULL, uuid TEXT NOT NULL, token TEXT NOT NULL, renewal TEXT, expires DATETIME)
        "
    ).unwrap();
}

pub fn store_new_user(name: &str, password: &str) {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();

    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(password)
        .with_secret_key(configuration::get_secret_key())
        .hash()
        .unwrap();

    let mut statement = connection
        .prepare("INSERT INTO users (username, password_hash) VALUES (?,?)")
        .unwrap();

    statement.bind(1, &sqlite::Value::String(name.to_string())).unwrap();
    statement.bind(2, &sqlite::Value::String(hash)).unwrap();
    statement.next().unwrap();
}

// can explore JWT for auth tokens, probably not needed right now as API 
// requestor is the same owner as this service

pub fn invalidate_tokens_for_user(name: &str) {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();

    let result = ::db::retrieval::get_id_for_user(name);
    match result {
        Some(id) => {
            let mut c = connection
                .prepare("DELETE FROM tokens WHERE (uid = ?)")
                .unwrap();

            c.bind(1, &sqlite::Value::Integer(id)).unwrap();
            info!("{}", id);
            c.next().unwrap();
        }
        None => {}
    };
}

pub fn create_auth_token(name: &str) -> Option<(String, String, String)> {
    let result = ::db::retrieval::get_id_for_user(name);

    match result {
        Some(user_id) => {
            invalidate_tokens_for_user(name);

            let db_filepath = configuration::get_db_filepath();
            let connection = sqlite::open(db_filepath).unwrap();

            let unique_id = Uuid::new_v4().to_string();
            let renewal_dt = Utc::now() + Duration::hours(1);
            let renewal_string = format!("{}{}{}{}", unique_id, name, renewal_dt.to_rfc3339(), configuration::get_secret_key());
            let mut hasher = Sha256::new();
            hasher.input_str(&renewal_string);
            let renewal_token = hasher.result_str();

            let auth_string = format!("{}{}{}", name, renewal_token, configuration::get_secret_key());
            let mut hasher = Sha256::new();
            hasher.input_str(&auth_string);
            let auth_token = hasher.result_str();

            let mut statement = connection
                .prepare("INSERT INTO tokens (uid, uuid, token, renewal, expires) VALUES (?,?,?,?,?)")
                .unwrap();

            statement.bind(1, &sqlite::Value::Integer(user_id)).unwrap();
            statement.bind(2, &sqlite::Value::String(unique_id)).unwrap();
            statement.bind(3, &sqlite::Value::String(auth_token.to_string())).unwrap();
            statement.bind(4, &sqlite::Value::String(renewal_token.to_string())).unwrap();
            statement.bind(5, &sqlite::Value::String(renewal_dt.to_rfc3339())).unwrap();
            statement.next().unwrap();
    
            return Some((auth_token.to_string(), renewal_token.to_string(), renewal_dt.to_string()));
        },
        None => {
            return None;
        }
    }
}

pub fn get_renewal_token_user_name(renewal_token: &str) -> Option<String> {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();

    let mut c = connection
        .prepare("SELECT * FROM tokens WHERE (renewal = ?)")
        .unwrap();

    c.bind(1, &sqlite::Value::String(renewal_token.to_string())).unwrap();

    while let State::Row = c.next().unwrap() {
        let user_id = c.read::<i64>(1).unwrap();
        let result = ::db::retrieval::get_user_name_for_id(user_id);
        return result;
    }

    return None;
}

pub fn renew_auth(renewal_token: &str) -> Option<(String, String, String)> {
    let result = get_renewal_token_user_name(renewal_token);
    match result {
        Some(name) => {
            return create_auth_token(&name);
        }
        None => {
            return None;
        }
    }
}
