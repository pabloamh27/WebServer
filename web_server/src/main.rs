use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs, time::Duration, thread, process::exit};
use web_server::{ThreadPool};

static mut BUSY_WORKERS: usize = 0;

fn listener(threads: &String, root: &String,port: &String) {
    let threads = threads.parse::<usize>().unwrap();
    let listener = TcpListener::bind(port).unwrap();
    let pool = ThreadPool::new(threads);
    unsafe{
        BUSY_WORKERS = 0;
    }    



    for stream in listener.incoming() {
        let stream = stream.unwrap();
        unsafe {
        print!( "{}\n", BUSY_WORKERS);
        if BUSY_WORKERS <= pool.workers.len() {
            BUSY_WORKERS += 1;
            pool.execute(|| {
                handle_connection(stream,"/home/royner39/SistemasOperativos/tarea3-sistemasoperativos/web_server/src/resources/",0);
            });
        }
        else{
            handle_connection(stream, &root, 1);
            println!("\n\n\n\n SE HA RECHAZADO UNA PETICION POR SOBRECARGA DE HILOS");
        }    
    }}
}

fn handle_connection(mut stream: TcpStream, root: &str ,error_code: u8) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
    let (status_line, file_name) = 
    if error_code == 1 {
        ("HTTP/1.1 200 OK", "no_threads.html")
    } else if http_request[0] == "GET / HTTP/1.1" && error_code == 0 {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if http_request[0] == "POST / HTTP/1.1" && error_code == 0 {
        let data = &http_request[0];
        fs::write("resources/post.html", data).expect("No se puede escribir el archivo");

        ("HTTP/1.1 200 OK", "post.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "error404.html")
    };
    let full_root = format!("{}{}", root, file_name);
    let contents = fs::read_to_string(full_root).unwrap();
    let length = contents.len();
    thread::sleep(Duration::from_secs(2));
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {:#?}", http_request);
    unsafe{
        BUSY_WORKERS -= 1;
    }

}

fn comprobador (argumentos : Vec<String>) {
    //Condicional para verificar que el usuario ingrese "rastreador" como primer instrucción
    if argumentos [0] != "prethread-WebServer" {
        println!("El comando debe inciar con la palabra 'prethread-WebServer'");
        exit(1);
    }
    //Verifica que el usuario haya ingresado una opcion de rastreador válida
    if argumentos [1] == "-n"{
        if argumentos [3] == "-w" {
            if argumentos [5] == "-p" {
                println!("\nServidor iniciado con éxito!");
                listener(&argumentos [2], &argumentos[4].to_string(), &argumentos[6]);
                exit(1);
            }else{
                println!("El comando debe contener el puerto '-p'");
                exit(1);
            }
            
        }
        else{
            println!("El comando debe contener el HTTP Root '-w'");
            exit(1);
        }
    }
    else{
        println!("El comando debe contener el número de hilos '-n'");
        exit(1);
    }
}

fn main() {
    //let mut input = String::new();
    let input = "prethread-WebServer -n 20 -w /home/royner39/SistemasOperativos/tarea3-sistemasoperativos/web_server/src/resources/ -p 127.0.0.1:8080";
    //stdin().read_line(&mut input).unwrap();
    let argumentos: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    //let argumentos: Vec<String> = env::args().map(|x| x.to_string()).collect();
    comprobador(argumentos);  
}
