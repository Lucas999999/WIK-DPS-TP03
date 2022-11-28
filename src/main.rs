use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
   let port = match env::var("PING_LISTEN_PORT"){
    Ok(env_port) => format!("0.0.0.0:{env_port}"),
    Err(_e) => format!("0.0.0.0:3333"),
    };
    let listener = TcpListener::bind(port).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
    let headers = http_request.split_off(1);
    let request_line = &http_request[0];
    if request_line == "GET /ping HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let mut header_formated = String::new();
        header_formated = header_formated + "{";
        let mut first = true;
        for entries in headers {
            let entries_vec: Vec<_> = entries.split(": ").collect();
            if first {
                header_formated = header_formated +"\""+ &entries_vec[0] +"\""+":"+"\""+ &entries_vec[1]+"\"";
                first = false;
            } else {
                header_formated =  header_formated + "," + "\""+ &entries_vec[0] +"\""+":"+"\""+ &entries_vec[1]+"\"" ;
            }
        }
        header_formated = header_formated + "}";
        let response = format!(
            "{status_line}\r\nContent-Type: application/json;\r\n\r\n{header_formated}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let response = format!(
            "{status_line}\r\n\r\n"
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
}
