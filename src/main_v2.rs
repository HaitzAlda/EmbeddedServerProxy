use async_h1::{client,server};
use async_net::{TcpListener, TcpStream}; //Crear una conexion TCP con alguien.
use http_types::{Request, Response, StatusCode, Url};
use smol_potat::main;
use std::str::FromStr;
use std::sync::atomic::{AtomicIsize,Ordering};
use std::sync::{Arc, Mutex};
use std::{thread,io}; 

const BUFFER_SZ: isize = 100;

#[derive(Clone,Debug,Default)]
struct Struct{
	method : String,
	path : String,
	headers : Vec<(String, String)>,
	query : String,
}


#[main]
async fn main() -> std::io::Result<()> {

    //Crea un listener que escuchará en el puerto 8080 para desepues crear conexiones con lo recibido
    let listener = TcpListener::bind("0.0.0.0:8080").await?; // "?" para gestionar errores automaticamente
    println!("Proxy-a http://0.0.0.0:8080-n entzuten");

	let zirk_array = Arc::new(
			(0..BUFFER_SZ)
				.map(|_| Mutex::new(Struct::default()))
				.collect::<Vec<_>>(),
	); //Array Zirkularra sarrerak gordetzeko.

	let flag = Arc::new(AtomicIsize::new(-1));

	//SUPONER QUE ESTO FUNCIONA ASI
	let flag_clone = flag.clone();
	let zirk_array_clone  = zirk_array.clone();
	thread::spawn(move || konexio_aztertu(zirk_array_clone,flag_clone)); //Bigarren hari batek konexioaren informazioa aztertu
    
	loop {
        let (stream, source_addr) = listener.accept().await?;
		let zirk_array_clone = zirk_array.clone();
		let flag_clone = flag.clone();
		smol::spawn(async move {//Modu asinkronoan eskaera bakoitza kudeatu
				//TO-DO leer los datos que nos interesan en los IDS y pasarselo a la función

				let source_ip = source_addr.ip().to_string();
				let source_port = source_addr.port();

				if let Err(error) = server::accept(stream, move |req| {
					let zirk_array_clone = zirk_array_clone.clone();
					let flag_clone = flag_clone.clone();
					let source_ip = source_ip.clone();
					async move {
						
						//Hemen egin bestela req-en kopia bat egin behar beste metodo batera pasatzeko
						let method = req.method().to_string();
						let headers = req.iter()
							.map(|(name, values)| (name.to_string(), values.to_string()))
							.collect::<Vec<_>>();
						let query = req.url().query().unwrap_or("").to_string();
						info_gehitu(zirk_array_clone, flag_clone, method, headers, source_ip, source_port, query);

						// Continuar con el proxy hacia el servidor real
						server_connection(req, Arc::new(())).await
					}
				}).await{
					eprintln!("Konexio errorea: {}", error);
					}
		}).detach();
	} //Konexioa emanda (edo ez) desekonektatu
}

async fn server_connection(mut req: Request, _cliente: Arc<()>) -> http_types::Result<Response> {
    
	let server_host = "127.0.0.1";
    let server_port = 8001;
    let server_addr = format!("{}:{}", server_host, server_port);

    let mut url = format!("http://{}{}", server_addr, req.url().path());

    if let Some(query) = req.url().query() {
        url.push('?');
        url.push_str(query);
    }

    let url_parsed = Url::from_str(&url)?;

   *req.url_mut() = url_parsed;
   req.insert_header("Host", format!("{}:{}", server_host, server_port));


   let stream = TcpStream::connect(&server_addr).await.map_err(|e| {
       eprintln!("Connection to server failed: {}", e);
       io::Error::new(io::ErrorKind::Other, "Failed to connect to upstream server")
   })?;


   match client::connect(stream, req).await {
        Ok(mut response) => {
            // Clone headers and body correctly
			let body_bytes = response.body_bytes().await.unwrap_or_default();
            let mut final_response = Response::new(response.status());

            // Copy all headers
            for (name, value) in response.iter() {
                final_response.insert_header(name, value);
            }

            // Copy body
			
            final_response.set_body(body_bytes.clone());
			final_response.insert_header("Content-Length", body_bytes.len().to_string());

            Ok(final_response)
        }
        Err(err) => {
            eprintln!("Error sending request to upstream server: {}", err);
            let mut err_response = Response::new(StatusCode::BadGateway);
            err_response.set_body("Proxy forwarding error");
            Ok(err_response)
        }
    }
}

fn info_gehitu(zirk_array : Arc<Vec<Mutex<Struct>>>, flag : Arc<AtomicIsize>, method: String, headers : Vec<(String, String)>, source_ip : String, source_port : u16, query : String){
	//TO-DO Informazioa array zirkularrera gehitu
	let mut last_pos = flag.load(Ordering::Relaxed); //Revisar si hace falta hacer modulo buffer_sz
	last_pos = (last_pos + 1) % BUFFER_SZ as isize;
	if let Some(slot) = zirk_array.get(last_pos as usize){
		let mut data = slot.lock().unwrap();
		*data = Struct {
			method,
			path: format!("{}:{}", source_ip, source_port),
			headers,
			query,
		};
		
		flag.store(last_pos, Ordering::Relaxed);
	}
}

fn konexio_aztertu(zirk_array : Arc<Vec<Mutex<Struct>>>, flag : Arc<AtomicIsize>){

	let mut last_read_pos : isize = -1;
	loop{
		let read_flag = flag.load(Ordering::Relaxed);

		if last_read_pos != read_flag{
			if let Some(slot) = zirk_array.get(read_flag as usize){
				let data = slot.lock().unwrap();
				last_read_pos = (last_read_pos + 1) % BUFFER_SZ as isize;
				println!("Connection Analized. N.{}", last_read_pos);
				println!("Method {}\nPath {}", data.method, data.path);
				println!("Query {}\n", data.query);
				println!("Headers:\n");
				for (name, values) in &data.headers{
					println!("	{},{}\n", name, values);
				}
			}
		}
	}
	
}