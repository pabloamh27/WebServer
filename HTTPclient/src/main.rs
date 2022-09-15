use hyper::{Client,Body, Method, Request};
pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type GenericResult<T> = std::result::Result<T, GenericError>;
use hyper::body;
use std::env;
use std::fs;

// Funcion para enviar una solicitud de tipo GET al servidor 
async fn get(url:&str, filename: &str) ->  Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{}", filename);
    let url = format!("{}{}", url, filename);
    let request = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("accept", "application/json")
        .body(Body::from(filename.to_string())).unwrap();
    let client = Client::new();
    let response = client.request(request).await.unwrap();
    println!("Response GET: {}", response.status());
    Ok(())
}

// Funcion para enviar una solicitud de tipo POST al servidor
async fn post(url: &str, message: &str) ->  Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = fs::read_to_string(message).expect("Unable to read file");
    let request = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("accept", "application/json")
        .header("Content-type", "application/json; charset=UTF-8")
        .body(Body::from(data)).unwrap();
    let client = Client::new();
    let response = client.request(request).await.unwrap();
    let bytes = body::to_bytes(response.into_body()).await.unwrap();
    
    println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );
    Ok(())
}

// Funcion para enviar una solicitud de tipo DELETE al servidor
async fn delete(url: &str, message: &str) ->  Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = fs::read_to_string(message).expect("Unable to read file");
    let request = Request::builder()
        .method(Method::DELETE)
        .uri(url)
        .header("accept", "application/json")
        .header("Content-type", "application/json; charset=UTF-8")
        .body(Body::from(data)).unwrap();
    let client = Client::new();
    let response = client.request(request).await.unwrap();
    let bytes = body::to_bytes(response.into_body()).await.unwrap();
    
    println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );
    Ok(())
}

// Funcion para enviar una solicitud de tipo PUT al servidor
async fn put(url: &str, message: &str) ->  Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let data = fs::read_to_string(message).expect("Unable to read file");
    let request = Request::builder()
        .method(Method::PUT)
        .uri(url)
        .header("accept", "application/json")
        .header("Content-type", "application/json; charset=UTF-8")
        .body(Body::from(data)).unwrap();
    let client = Client::new();
    let response = client.request(request).await.unwrap();
    let bytes = body::to_bytes(response.into_body()).await.unwrap();
    
    println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );
    Ok(())
}

#[tokio::main]
// Funcion principal
async fn main() ->  Result<(), Box<dyn std::error::Error + Send + Sync>> { 
    //Lee argumentos de consola
    let argumentos: Vec<String> = env::args().collect();
    
    //GET METHOD
    //let input = "HTTPclient -h http://127.0.0.1:8080 GET /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/web_server/src/resources/hello.html";
    
    //POST METHOD
    //let input = "HTTPclient -h http://127.0.0.1:8080 POST /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/post.html";
    
    //DELETE METHOD
    //let input = "HTTPclient -h http://127.0.0.1:8080 DELETE /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/web_server/src/resources/post.html";
    
    //PUT METHOD
    //let input = "HTTPclient -h http://127.0.0.1:8080 PUT /home/pablo/Desktop/ReposGit/tarea3-sistemasoperativos/post.html";
   
   
    //stdin().read_line(&mut input).unwrap();
    //let argumentos: Vec<String> = input.split_whitespace().map(|x| x.to_string()).collect();
    
    
    for i in 1..argumentos.len() {
        println!("{}", argumentos[i]);
    }
    let choice = &argumentos[3];
    let host = &argumentos[2];
    let arguments;
    if argumentos.len() > 4 {
        arguments = &argumentos[4];
    } else {
        arguments = &argumentos[0];
    }

    let post_method = "POST";
    let get_method = "GET";
    let delete_method = "DELETE";
    let put_method = "PUT";
    
    //Verifica el tipo de metodos
    if choice == post_method {
        post(host, arguments).await;
    } else if choice == get_method {
        get(host,arguments).await;
    } else if choice == delete_method {
        delete(host,arguments).await;
    } else if choice == put_method {
        put(host,arguments).await;
    }

    Ok(())
}