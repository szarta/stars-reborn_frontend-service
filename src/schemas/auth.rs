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
pub const AUTH_REQUEST_SCHEMA : &str = r#"
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "$id": "http://www.stars-reborn.com/schemas/auth-request-schema.json",
  "description": "The payload required to authenticate a user.",
  "version": "0.0.1",
  "definitions": {},
  "additionalProperties": false,
  "required": [ 
    "username",
    "password"
   ],
  "properties": {
    "username": {
      "type": "string",
      "maxLength": 64
    },
    "password": {
      "type": "string",
      "maxLength": 256
    }
  }
}
"#;

pub const AUTH_RENEW_SCHEMA : &str = r#"
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "$id": "http://www.stars-reborn.com/schemas/auth-renew-schema.json",
  "description": "The payload required to renew an authentication token.",
  "version": "0.0.1",
  "definitions": {},
  "additionalProperties": false,
  "required": [ 
    "renewToken"
   ],
  "properties": {
    "renewToken": {
      "type": "string",
      "minLength": 64,
      "maxLength": 64
    }
  }
}
"#;


