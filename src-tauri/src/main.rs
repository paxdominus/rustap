#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tinyjson::*;

#[derive(Serialize, Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, do_sql, serve])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct Data {
    nmae: String,
    job: String,
}

#[derive(Debug, PartialEq, Eq)]
struct TableUnit {
    one: i32,
    two: i32,
    three: Option<String>,
}

#[tauri::command]
fn do_sql() {
    let url = "mysql://root:@localhost:3307/test";
    let pool = Pool::new(url);

    let mut conn = pool.unwrap().get_conn();

    // Let's create a table for payments.
    conn.as_mut().expect("msg").query_drop(
        r"CREATE TABLE IF NOT EXISTS test (
            one int not null,
            two int not null,
            three text
        )",
    );

    let table_units = vec![
        TableUnit {
            one: 1,
            two: 2,
            three: None,
        },
        TableUnit {
            one: 3,
            two: 4,
            three: Some("foo".into()),
        },
        TableUnit {
            one: 5,
            two: 6,
            three: None,
        },
        TableUnit {
            one: 7,
            two: 8,
            three: None,
        },
        TableUnit {
            one: 9,
            two: 10,
            three: Some("bar".into()),
        },
    ];

    // Now let's insert payments to the database
    conn.as_mut().unwrap().exec_batch(
        r"INSERT INTO test (one, two, three)
          VALUES (:one, :two, :three)",
        table_units.iter().map(|p| {
            params! {
                "one" => p.one,
                "two" => p.two,
                "three" => &p.three,
            }
        }),
    );

    // Let's select payments from database. Type inference should do the trick here.
    let selected_units = conn
        .expect("asd")
        .query_map("SELECT one, two, three from test", |(one, two, three)| {
            TableUnit { one, two, three }
        });

    // Let's make sure, that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows
    // without `ORDER BY`, so assume we are lucky.
    println!("Yay!");
}

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{fs, thread};

fn handle_read(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_write(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "POST / HTTP/1.1" {
        let input = r#"{"name":"8", "job":"sfsdf"}"#;
        let m: JsonValue = input.parse();

        let response = b"HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{name:\"asdads\",name:\"asdads\"}\r\n";
        match stream.write(response) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
    }
    return;
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    handle_read(&stream);
    handle_write(stream);
}
#[tauri::command]
fn serve() {
    let listener = TcpListener::bind("192.168.29.247:8081").unwrap();
    println!("Listening for connections on port {}", 8081);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "POST / HTTP/1.1" {
        // let status_line = "HTTP/1.1 200 OK";
        // let contents = fs::read_to_string("").unwrap();
        // let length = contents.len();

        // let response = format!(
        // "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        // );

        // stream.write_all(response.as_bytes()).unwrap();
    } else {
        // some other request
    }
}
