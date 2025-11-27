use tokio::net::{TcpListener, TcpStream,UdpSocket};
use tokio::io;
use tokio::io::Result;

/*DEJALO TODO LIMPIO*/
async fn TcpHandler(mut inStream : TcpStream) -> Result<()>{

   let mut outStream = TcpStream::connect("127.0.0.1:8001").await?;

   tokio::io::copy_bidirectional(&mut inStream, &mut outStream).await?;
   Ok(())
}

async fn UdpHandler(mut sock : UdpSocket) -> Result<()>{

   let mut buf = [0u8; 2048];

   let backed_socket = UdpSocket::bind("0.0.0.0:0").await?;
   backed_socket.connect("127.0.0.1:8001").await?;

   loop {
      let (len, addr) = sock.recv_from(&mut buf).await?;
      
      backed_socket.send(&buf[..len]).await?;

      let n = backed_socket.recv(&mut buf).await?;
      sock.send_to(&buf[..n], addr).await?;
   }

}

#[tokio::main]
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
      println!("TCP konexio berria: {}\n", source_addr);
      tokio::spawn(async move{
         if let Err(e) = TcpHandler(stream).await{
            eprintln!("TCP konexioak errorea eman du: {}", e);
         }
      });
   }

}
