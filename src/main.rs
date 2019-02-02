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
extern crate dotenv;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate sqlite;

extern crate argparse;
extern crate argonautica;

extern crate hyper;
extern crate futures;
extern crate url;
extern crate valico;

#[macro_use]
extern crate lazy_static;

extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate time;
extern crate uuid;
extern crate crypto;

use dotenv::dotenv;
use argparse::{ArgumentParser, Print};

pub mod service;
pub mod schemas {
    pub mod auth;
}
pub mod configuration;
pub mod db {
    pub mod storage;
    pub mod retrieval;
}

fn main() {
    dotenv().ok();
    env_logger::init();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Runs the front-end API servicers for stars-reborn");
        ap.add_option(&["-V", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");

        match ap.parse_args() {
            Ok(()) => {}
            Err(x) => {
                std::process::exit(x);
            }
        }
    }

    ::db::storage::create_db_if_necessary();
    ::db::storage::store_new_user("brandon", "test1234");
    ::db::retrieval::validate_token_for_user("brann", "069567ec6338fb75ed66fd38b98d9953a6ce26c1364730a13b88f579687fe339");

    let server_addr = configuration::get_server_ip().parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&server_addr, || Ok(service::FrontendService {}))
        .unwrap();

    info!("Running server on {}", server_addr);
    server.run().unwrap();
}
