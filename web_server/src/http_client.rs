use std::io::{stdout, Write};

use curl::easy::Easy;

fn http_client (){
    let mut easy = Easy::new();
    easy.url("127.0.0.1:8080").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

fn comprobador (argumentos : Vec<String>) {
    //Condicional para verificar que el usuario ingrese "rastreador" como primer instrucción
    if argumentos [0] != "HTTPclient" {
        println!("El comando debe inciar con la palabra 'HTTPclient'");
        exit(1);
    }
    //Verifica que el usuario haya ingresado una opcion de rastreador válida
    if argumentos [1] == "-h"{
            println!("\nCliente iniciado con éxito!");
            http_client();
            exit(1);
        }else{
            println!("El comando debe contener el puerto '-h'");
            exit(1);
        }
}
fn main(){
    let argumentos: Vec<String> = env::args().map(|x| x.to_string()).collect();
    comprobador(argumentos); 
}