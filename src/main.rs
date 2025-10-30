use tokio::net::UdpSocket;
use tokio::io::TcpListener;
use std::io;
use std::net::TcpStream;

/*DEJALO TODO LIMPIO*/
async fn TcpHandler(mut inStream : stream, source_addr : addr) {

   let mut outStream = TcpStream::connect("127.0.0.1:8001").await?;
   match tokio::io::copy_bidirectional(&mut inStream, &mut outStream).await {
      OK((_read, _write)) => Ok(()),
      Err(e) => Err(e),
   }
}

async fn UdpHandler(mut sock : UdpSocket){

   let backed_socket = UdpSocket::bind("0.0.0.0:0").await?;
   loop {
      let (len, addr) = sock.recv_from(&mut buf).await?;

      backed_socket.connect("127.0.0.1:8001").await?;
      backed_socket.send(&buf[..len]).await?;

      let n = backed_socket.recv(&mut buf).await?;
      sock.send_to(&buf[..n], addr).await?;
   }

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
      if let Err(e) = UdpHandler(listenerUDP).await { 
         eprintln!("UDP konexioak errorea eman du: {}", e);
      }
   });

   loop{
      let (stream, source_addr) = listenerTCP.accept().await?;
      println!("TCP konexio berria\n");
      tokio::spawn(async move{
         if let Err(e) = TcpHandler(stream, source_addr).await{
            eprintln!("TCP konexioak errorea eman du: {}", e);
         }
      });
   }

}
