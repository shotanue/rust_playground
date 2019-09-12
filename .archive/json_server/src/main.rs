extern crate actix_web;
extern crate serde_json;
use actix_web::{http, server, App, HttpRequest};
use serde_json::{ Value};


fn main() {


    server::new(
        || App::new()
            .route("/", http::Method::GET, |r: HttpRequest| {
                // Some JSON input data as a &str. Maybe this comes from the user.
                let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

                // Parse the string of data into serde_json::Value.
                let v: Value = serde_json::from_str(data).unwrap();

                v.to_string()
            }))
        .workers(5)
        .backlog(5)
        .keep_alive(server::KeepAlive::Timeout(75))
        .server_hostname("test".to_string())
        .bind("127.0.0.1:8080").unwrap()
        .run();
}