use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs, time::Duration, thread};
use web_server::ThreadPool;

fn listener() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming().take(25) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn main() {
    listener()    
}

fn handle_connection(mut stream: TcpStream){

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
    let (status_line, file_name) = if http_request[0] == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src/resources/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/resources/error404.html")
    };
    thread::sleep(Duration::from_secs(10));
    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {:#?}", http_request);

}