#![windows_subsystem = "windows"]

extern crate web_view;

use std::fs;
use web_view::*;

fn main() {
    let contents = fs::read_to_string("dist/index.html")
        .expect("Something went wrong reading the file");

    web_view::builder()
        .title("Minimal webview example")
        .content(Content::Url("http://localhost:8080/"))
        // .content(Content::Html(contents))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}































