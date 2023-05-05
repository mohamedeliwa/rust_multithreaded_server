use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use multithreaded_server::ThreadPool;

fn main() {
    let listener = match TcpListener::bind("127.0.0.rr:7878") {
        Ok(listener) => listener,
        Err(error) => {
            panic!("{}", error.to_string())
        }
    };
    let pool = ThreadPool::new(4);

    // this server is intended to take only 9 requests
    for stream in listener.incoming().take(9) {
        let stream = match stream {
            Ok(stream) => stream,
            Err(_) => continue,
        };
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // reading the first line of the http request
    // The first unwrap takes care of the Option and stops the program if the iterator has no items.
    // The second unwrap handles the Result
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
