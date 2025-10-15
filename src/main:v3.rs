use tokio::net::UdpSocket;
use tokio::io::TcpListener;
use std::io;

/*DEJALO TODO LIMPIO*/
async fn TcpHandler() {

}

async fn UdpHandler(){

}

#[tokio:main]
async fn main()-> io::Result<()>{

   /*TODO: Preparar los dos sockets para escuchar. 
   Cuando se reciva una conexion en ambos casos guardarlos en la funcion de analizar en el otro hilo
   Cada vez que uno reciva una conexion spawnear una llamada asincrona a su funcion respectiva y que esa se encargue de gestionarlo
   */   


   let listenerTCP = TcpListener::bind("0.0.0.0:8080").await?;
   let listenerUDP = UdpSocket::bind("0.0.0.0:8080").await?; //Checkear que esto funciona asi y no hay que hacer nada mas

   println!("TCP eta UDP entzuten\n");

   
   tokio::spawn(async move {
      if let Err(e) = UdpHandler().await { //TODO
         eprintln!("UDP konexioak errorea eman du: {}", e);
      }
   });

   loop{
      let (stream, source_addr) = listenerTCP.accept().await?;
      println!("TCP konexio berria\n");
      tokio::spawn(async move{
         if let Err(e) = TcpHandler().await{ //TODO
            eprintln!("TCP konexioak errorea eman du: {}", e);
         }
      });
   }

}
