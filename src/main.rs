use async_h1::server;
use async_net::TcpListener;
use http_types::{Request, Response, StatusCode, Url};
use smol_potat::main;
//use smol::prelude::*;
use std::str::FromStr;
use std::sync::Arc;

#[main]
async fn main() -> std::io::Result<()> {

    //Esperar a recibir cualquier conexión, y llamar a "server_connection"
    let listener = TcpListener::bind("0.0.0.0:8080").await?; // "?" para gestionar errores automaticamente
    println!("Proxy-a http://0.0.0.0:8080-n entzuten");
    let cliente = Arc::new(surf::Client::new());

    loop {
        let (stream, _) = listener.accept().await?;
	let cliente_clone = cliente.clone();        

	smol::spawn(async move {//Modu asinkronoan eskaera bakoitza kudeatu
            if let Err(error) = server::accept(stream, move |req|  server_connection(req,cliente_clone.clone())).await {
                eprintln!("Konexio errorea: {}", error);
            }
        })
        .detach(); //Konexioa emanda (edo ez) desekonektatu
    }
}

//Proxyak jasotako URI-a zerbitzariari bidali (Get bakarrik oraingoz)
async fn server_connection(mut req: Request, cliente : Arc<surf::Client>) -> http_types::Result<Response> {
	let server_helb = "http://127.0.0.1:8001"; //Helbide berria

	//Uri berria formateatu "req"-en informazioarekin
	let mut url = format!("{}{}", server_helb, req.url().path()).to_string();

	if let Some(query) = req.url().query() { //Comprobar que req.url.query existe
		
		url.push('?');
		url.push_str(query);
	}

	let url_parse = Url::from_str(&url)?; //Parsear el url

	let mut final_req = Request::new(req.method(), url_parse);

	final_req.set_body(req.take_body());

	//Uri berria  zerbitzariari pasa. Erantzuna itzuli main-era
	match cliente.send(final_req).await {
        	Ok(mut res) => {
            		let status = res.status();
            		let body = res.body_bytes().await.unwrap_or_default();
            		let mut response = Response::new(status);
            		response.set_body(body);
            		Ok(response)
        	}
        	Err(error) => {
            		eprintln!("Proxy errorea: {}", error);
            		let mut res = Response::new(StatusCode::BadGateway);
            		res.set_body("Pakete berbidalketa errorea");
            		Ok(res)
        	}
    	}
}
