use async_h1::server;
use async_net::TcpListener;
use http_types::{Request, Response, StatusCode};
use smol_potat::main;
use smol::prelude::*;

#[main]
async fn main() -> std::io::Result<()> {
    //Wait to receive any connection to port 8080, and  send it to proxy_handler
    let listener = TcpListener::bind("0.0.0.0:8080").await?; // "?" to handle errors
    println!("Proxy listening on http://0.0.0.0:8080");

    loop {
        let (stream, _) = listener.accept().await?;
        smol::spawn(async move {
            if let Err(err) = server::accept(stream, proxy_handler).await {
                eprintln!("Connection error: {}", err);
            }
        })
        .detach(); //Disconnect
    }
}

async fn proxy_handler(req: Request) -> http_types::Result<Response> {
    let path = req.url().path();
    let forward_url = format!("http://127.0.0.1:5000{}", path);

    match surf::get(forward_url).await {
        Ok(mut res) => {
            let status = res.status();
            let body = res.body_string().await.unwrap_or_default();
            let mut response = Response::new(status);
            response.set_body(body);
            Ok(response)
        }
        Err(e) => {
            eprintln!("Error forwarding request: {}", e);
            let mut res = Response::new(StatusCode::BadGateway);
            res.set_body("Bad Gateway");
            Ok(res)
        }
    }
}
