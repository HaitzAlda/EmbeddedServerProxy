use async_h1::server;
use async_net::TcpListener;
use http_types::{Request, Response, StatusCode, Url};
use smol_potat::main;
use std::str::FromStr;
use std::sync::atomic::{AtomicIsize,Ordering};
use std::sync::{Arc, Mutex};
use std::thread; 

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

    //Esperar a recibir cualquier conexión, y llamar a "server_connection"
    let listener = TcpListener::bind("0.0.0.0:8080").await?; // "?" para gestionar errores automaticamente
    println!("Proxy-a http://0.0.0.0:8080-n entzuten");

    let cliente = Arc::new(surf::Client::new());

	
	let zirk_array = Arc::new(
			(0..BUFFER_SZ)
				.map(|_| Mutex::new(Struct::default()))
				.collect::<Vec<_>>(),
	); //Array Zirkularra sarrerak gordetzeko.

	let flag = Arc::new(AtomicIsize::new(-1));

	//SUPONER QUE ESTO FUNCIONA ASI
	let flag_clone = flag.clone();
	let zirk_array_clone  = zirk_array.clone();
	thread::spawn(move || konexio_aztertu(zirk_array_clone,flag_clone));
    
	loop {
        let (stream, source_addr) = listener.accept().await?;
		let cliente_clone = cliente.clone();
		let zirk_array_clone = zirk_array.clone();
		let flag_clone = flag.clone();
		smol::spawn(async move {//Modu asinkronoan eskaera bakoitza kudeatu
				//TO-DO leer los datos que nos interesan en los IDS y pasarselo a la función

				let source_ip = source_addr.ip().to_string();
				let source_port = source_addr.port();

				if let Err(error) = server::accept(stream, move |req| {
					let zirk_array_clone = zirk_array_clone.clone();
					let flag_clone = flag_clone.clone();
					let cliente_clone = cliente_clone.clone();
					let source_ip = source_ip.clone();
					async move {
						
						let method = req.method().to_string();
						let headers = req.iter()
							.map(|(name, values)| (name.to_string(), values.to_string()))
							.collect::<Vec<_>>();
						let query = req.url().query().unwrap_or("").to_string();
						info_gehitu(zirk_array_clone, flag_clone, method, headers, source_ip, source_port, query);

						// Continuar con el proxy hacia el servidor real
						server_connection(req, cliente_clone).await
					}
				}).await{
					eprintln!("Konexio errorea: {}", error);
					}
		}).detach();
	} //Konexioa emanda (edo ez) desekonektatu
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

for (name, values) in req.iter() {
    if name.as_str().eq_ignore_ascii_case("host") {
        continue;
    }
    if name.as_str().eq_ignore_ascii_case("accept-encoding") {
        continue; // Skip forwarding this header
    }
    final_req.insert_header(name, values);
}
final_req.insert_header("Host", "localhost:8001");


	//if req.method() != http_types::Method::Get {
    final_req.set_body(req.take_body());
//}

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