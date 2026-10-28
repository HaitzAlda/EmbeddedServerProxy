use pnet

const BUFFER_SZ:usize = 100;

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

fn main(){

}