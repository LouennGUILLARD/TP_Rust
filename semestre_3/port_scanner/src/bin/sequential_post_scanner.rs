use std::time::Instant;
use port_scanner::is_open_sync;
use clap::Parser;

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
  let h = my_parameters.host;
  for i in my_parameters.port_min..my_parameters.port_max+1{
    let k = is_open_sync(&h, i, my_parameters.timeout);
    println!("statut : {}, port :{}", k,i);
  }

  println!("{:?}", instant.elapsed());
}