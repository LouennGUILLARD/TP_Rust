use std::time::Instant;
use port_scanner::is_open_sync;
use clap::Parser;
use std::sync::{Arc, Mutex};
//use std::thread;

#[derive(Parser)]
struct Options {
    host : String,
    port_min: u16,
    port_max: u16,
    timeout: u64,
}




fn main(){
  let instant = Instant::now();
  let my_parameters = Options::parse();
  let mut handles = vec![];
  let open_ports = Arc::new(Mutex::new(vec![]));

  let h = my_parameters.host;
  for i in my_parameters.port_min..my_parameters.port_max+1{
    let h_clone = h.clone();
    let open_ports_clone = Arc::clone(&open_ports);
    let handle = std::thread::spawn(move || {
      let k = is_open_sync(&h_clone, i, my_parameters.timeout);
      if k {
        let mut open_ports = open_ports_clone.lock().unwrap();
        open_ports.push(i);
      }
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  let open_ports = Arc::try_unwrap(open_ports).unwrap().into_inner().unwrap();
  println!("Ports ouverts : {:?}", open_ports);
  println!("{:?}", instant.elapsed());
}