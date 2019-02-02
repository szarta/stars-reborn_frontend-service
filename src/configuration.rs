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
use std::env;

pub fn get_server_ip() -> String {
    let key = "SERVER_ADDR";
    match env::var(key) {
        Ok(val) => {
           info!("{}: {:?}", key, val);
           val
        },
        Err(_e) => {
            info!("{}: not in ENV, falling back to default", key);
            String::from("127.0.0.1:7120")
        },
    }
}

pub fn get_db_filepath() -> String {
    let key = "DB_FILEPATH";
    match env::var(key) {
        Ok(val) => {
           val
        },
        Err(_e) => {
            info!("{}: not in ENV, falling back to default", key);
            String::from("frontend.sqlite")
        },
    }
}

pub fn get_secret_key() -> String {
    let key = "SECRET_KEY";
    match env::var(key) {
        Ok(val) => {
           val
        },
        Err(_e) => {
            info!("{}: not in ENV, falling back to default", key);
            String::from("HOLY COW THIS IS YOUR SECRET KEY OMG")
        },
    }
}



