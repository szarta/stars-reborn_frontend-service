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
use argonautica::Verifier;
use chrono::prelude::*;
use time::Duration;

pub fn is_auth_valid(name: &str, password: &str) -> bool {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();

    let mut c = connection
        .prepare("SELECT * FROM users WHERE (username = ?)")
        .unwrap();

    c.bind(1, &sqlite::Value::String(name.to_string())).unwrap();

    while let State::Row = c.next().unwrap() {
    let mut verifier = Verifier::default();
        let hash : String = c.read::<String>(2).unwrap();
        let is_valid = verifier
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(configuration::get_secret_key())
            .verify()
            .unwrap();

        return is_valid;
    }

    return false;
}

pub fn get_id_for_user(name: &str) -> Option<i64> {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();

    let mut c = connection
        .prepare("SELECT * FROM users WHERE (username = ?)")
        .unwrap();

    c.bind(1, &sqlite::Value::String(name.to_string())).unwrap();

    while let State::Row = c.next().unwrap() {
        let id: i64 = c.read::<i64>(0).unwrap();
        return Some(id);
    }

    return None;
}

pub fn get_user_name_for_id(id: i64) -> Option<String> {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();

    let mut c = connection
        .prepare("SELECT * FROM users WHERE (id = ?)")
        .unwrap();

    c.bind(1, &sqlite::Value::Integer(id)).unwrap();

    while let State::Row = c.next().unwrap() {
        let username: String = c.read::<String>(1).unwrap();
        return Some(username);
    }

    return None;
}

pub fn validate_token(token: &str) -> bool {
    let db_filepath = configuration::get_db_filepath();
    let connection = sqlite::open(db_filepath).unwrap();

    let mut c = connection
        .prepare("SELECT * FROM tokens WHERE (token = ?)")
        .unwrap();

    c.bind(1, &sqlite::Value::String(token.to_string())).unwrap();

    while let State::Row = c.next().unwrap() {
        let dt = c.read::<String>(5).unwrap();

        let now = Utc::now();
        if DateTime::parse_from_rfc3339(&dt)
            .unwrap()
            .signed_duration_since(now) < Duration::seconds(1) {
                return false;
        }
        else {
            return true;
        }
    }

    return false;
}
