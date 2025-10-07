use async_h1::{client,server};
use async_net::{TcpListener};


#[main]
async fn  main() -> std::io::Result<()> {

   let listener = TcpListener::bind{"0.0.0.0:8080"}.await?;
   println!("Proxy-a http://0.0.0.0:8080-n entzuten.");
   //Main loop, koxeio berriak aztertzeko prest egon. Behin bat jasota, modu asinkronoan aztertu eta honek iteratu beste konxeioak aztertzeko
   loop{
    let (stream, addr) = listener.accept().await?;

    smol::spawn(async move {
        
    })
   }
}