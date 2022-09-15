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
        let root = root.clone();
        unsafe {
        print!( "{}\n", BUSY_WORKERS);
        if BUSY_WORKERS <= pool.workers.len() {
            BUSY_WORKERS += 1;
            pool.execute(|| {
                handle_connection(stream,root,0);
            });
        }
        else{
            handle_connection(stream, root, 1);
            println!("\n\n\n\n SE HA RECHAZADO UNA PETICION POR SOBRECARGA DE HILOS");
        }    
    }}
}
fn print_vector(vector: &Vec<String>){
    for i in vector{
        println!("{} ", i);
    }
    println!("");
}

fn handle_connection(mut stream: TcpStream, root: String ,error_code: u8) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
    print_vector(&http_request);
    let test_path = "/home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/";
    let (status_line, file_name) = 
    if error_code == 1 {
        ("HTTP/1.1 200 OK", "no_threads.html")
    } else if http_request[0] == "GET / HTTP/1.1" && error_code == 0 {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if http_request[0] == "POST / HTTP/1.1" && error_code == 0 {
        let data = fs::read(format!("{}post.html", test_path)).unwrap();
        fs::write(format!("{}post.html", root), data).expect("No se puede escribir el archivo");
        ("HTTP/1.1 200 OK", "success.html")
    } else if http_request[0] == "DELETE / HTTP/1.1" && error_code == 0 {
        fs::remove_file(format!("{}post.html", root)).expect("No se puede eliminar el archivo");
        ("HTTP/1.1 200 OK", "success.html")
    } else if http_request[0] == "PUT / HTTP/1.1" && error_code == 0 {
        let data = fs::read(format!("{}post.html", test_path)).unwrap();
        if fs::metadata(format!("{}post.html", root)).is_ok() {
            fs::remove_file(format!("{}post.html", root)).expect("No se puede eliminar el archivo");
            fs::write(format!("{}post.html", root), data).expect("No se puede escribir el archivo");
        } else {
            println!("No se puede modificar el archivo ya que no existe");
        }
        ("HTTP/1.1 200 OK", "success.html")
    }
    else {
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
    let input = "prethread-WebServer -n 2 -w /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/web_server/src/resources/ -p 127.0.0.1:8080";
    //stdin().read_line(&mut input).unwrap();
    let argumentos: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    //Lee argumentos de consola
    //let argumentos: Vec<String> = env::args().map(|x| x.to_string()).collect();
    comprobador(argumentos);  
}
