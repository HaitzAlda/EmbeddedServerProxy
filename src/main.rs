use tokio::net::{TcpListener, TcpStream,UdpSocket};
use tokio::io;
use tokio::io::Result;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize};
use tokio::sync::Mutex;

/*DEJALO TODO LIMPIO*/
/*TODO:

*/
const BUFFER_SZ: usize = 100;

//REVISAR  LOS TIPOS
#[derive(Debug,Default,Clone,Copy)]
struct Struct {
   time: i32,
   packet_len:i32,
   high_layer:i32,
   ip_flags:i32,
   protocol:i32,
   len_tcp:i32,
   ack_len:i32,
   flags_tcp:i32,
   len_udp:i32,
   type_icmp:i32,
}

//Obtener siguiente posicion libre //No se si esto hace falta o es demasiado llamar a otra funcion
/*async fn obtain_ticket(last_pos: Arc<AtomicUsize>) -> usize {
    return last_pos.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}*/


//USA EL MUTEX.LOCK() PUTO PARA BLOQUEAR UN ASYNC Y QUE LOS DEMAS FUNCIONEN HASTA QUE TENGA EL PUTO MUTEX INUTIL
//Encargado de crear datos y borrar los que ya se encuentra ahi en una posicion
async fn create_new_data(mut pos: Arc<AtomicUsize>){
   //mutex noseke
   pos.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

}


//Encargado de actualizar datos en del array dado una posicion
async fn update_data(mut pos: Arc<AtomicUsize>){
   //mutex noseke
   pos.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

async fn TcpHandler(mut inStream : TcpStream, array: Arc<Mutex<[Struct; BUFFER_SZ]>>, last_pos: Arc<AtomicUsize>) -> io::Result<()>{

   let mut outStream = TcpStream::connect("127.0.0.1:8001").await?;

   tokio::io::copy_bidirectional(&mut inStream, &mut outStream).await?; //AQUI COMO MIERDAS RECOGO INFORMACION ENTRE PAQUETE Y PAQUETE MIRAR LA DOCUMENTACION DE ESTO
   print!("TCP amaituta");
   Ok(())

   /*Version alternativa:
      Crear un loop que funciona siempre que es stream este abierto por parte de uno de los dos
      Estar esperando un mensaje y cuando suceda transmitir tanto como guardar la info
    */
}

async fn UdpHandler(mut sock : UdpSocket, array: Arc<Mutex<[Struct; BUFFER_SZ]>>, last_pos : Arc<AtomicUsize>) -> io::Result<()>{

   let mut buf = [0u8; 2048];

   let backed_socket = UdpSocket::bind("0.0.0.0:0").await?;
   backed_socket.connect("127.0.0.1:8001").await?;

   loop {
      let (len, addr) = sock.recv_from(&mut buf).await?;
      
      backed_socket.send(&buf[..len]).await?;

      let n = backed_socket.recv(&mut buf).await?;
      sock.send_to(&buf[..n], addr).await?;
      //create_new_data(last_pos).await; //Meterle algo mas para que pille la info
   }

}

#[tokio::main]
async fn main()-> io::Result<()>{

   /*TODO: Preparar los dos sockets para escuchar. 
   Cuando se reciva una conexion en ambos casos guardarlos en la funcion de analizar en el otro hilo
   Cada vez que uno reciva una conexion spawnear una llamada asincrona a su funcion respectiva y que esa se encargue de gestionarlo
   */   

   let array = Arc::new(Mutex::new([Struct::default(); BUFFER_SZ]));
   let last_pos  = Arc::new(AtomicUsize::new(0));


   let listenerTCP = TcpListener::bind("0.0.0.0:8080").await?;
   let listenerUDP = UdpSocket::bind("0.0.0.0:8080").await?; //Checkear que esto funciona asi y no hay que hacer nada mas

   println!("TCP eta UDP entzuten\n");

   tokio::spawn(UdpHandler(listenerUDP, array.clone(), last_pos.clone()));

   loop{
      let (stream, source_addr) = listenerTCP.accept().await?;
      println!("TCP konexio berria: {}\n", source_addr);

      /*tokio::spawn(async move{
         if let Err(e) = TcpHandler(stream, array.clone()).await{
            eprintln!("TCP konexioak errorea eman du: {}", e);
         }
      });
      */
      tokio::spawn(TcpHandler(stream, array.clone(), last_pos.clone()));
   }

}


/*
Informacion a buscar:
   *Time: Tiempo transcurrido desde el ultimo paquete del mismo flujo. Si es el primer paquete el tiempo es 0
   *Packet Len: Longitud del paquete en bytes
   *Highest Layer: Nombre de la capa mas alta con SHA-256 aplicada y reducida mediante modulo 10⁸
   *IP Flags:  Los ip flags correspondientes en la cabezera IP, convertidos a decimal
   *Protocols: Firma binaria de los protocolos presentes en la pila del paquete. Se transforma a entero con producto escalar con un vector de potencias de dos
   *TCP Len: Longitud de segmento TCP. 0 si no es TCP y en decimal
   *TCP Ack: Numero de acuse de recibo (acknoledgement) del semento TCP. 0 si no es TCP y en decimal
   *TCP Flags: Flags del encabezado.  0 si no es TCP y en decimal
   *TCP Window Size: Tamaño de la ventana TCP. 0 si no es TCP y en decimal
   *UDP Len: Longitud del segmento UDP. 0 si no es UDP
   *ICMP Type: Tipo de mensaje ICMP. 0 si no es ICMP
 */

 /*
 DONDE GUARDAR INFORMACION:
Iniciar un array cirlular accesible por todos.

A cada conexion TCP se le asignara un numero en la lista con un mutex (ticket), despues escribiran en esa posicion una y otra vez hasta que hayan terminado.

A cada UDP se le asignara un numero en la lista con un mutex (ticket), despues escribirá en esa posicion cuando termine esa conexion. (No se si hace falta mantener trackeo de esa conexion).

Otro hilo analizara esas posiciones

Cada vez que a una conexion UDP o TCP se le asigne un ticket esa posicion se borrara, para asegurar que no hay basura y limpiar una conexion anterior no analizada o obsoleta.

Lasterketa egoeran nor:

*TCP ticket eskatu
*TCP idatzi
*UDP ticket eskatu
*UDP idatzi










pnet paCKETE DEFINITIVO CREO

*/