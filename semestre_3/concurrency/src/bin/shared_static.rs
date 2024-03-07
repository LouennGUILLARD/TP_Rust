use clap::Parser;
use std::thread;

#[derive(Debug,Parser)]
struct Options{
    n : i32
}

fn main(){
    let args = Options::parse();
    let mut handles = vec![];

    let message = "Bonjour";
    for i in 0..args.n{
        let handle = thread::spawn(move||{
            println!("{} n°{}",message,i);
            println!("Au revoir n° {}",i)
        }
    );
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
}