use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Person{
    name: String
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6969").unwrap();
    let mut data = HashMap::new();

    for stream in listener.incoming(){
        let input = handle_connection(stream.unwrap());
        data.insert("name", input.name);
        break;
    }
    println!("{:#?}", data);
}

fn handle_connection(mut stream:TcpStream)->Person{
    let buf_reader = BufReader::new(&mut stream);
    let http_req = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .collect::<Vec<_>>();
    let data = &http_req[7];
    let person: Person = serde_json::from_str(data).unwrap();
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    person
}

