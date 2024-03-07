use clap::Parser;
use std::thread;

#[derive(Debug,Parser)]
struct Options{
    n : i32
}

fn main(){
    //segment de mémoire de l'int est Pile
    let x = 12345;
    let ptr_x = &x;

    let args = Options::parse();
    let mut handles = vec![];

    let message = "Bonjour";
    for i in 0..args.n{
        let handle = thread::spawn(move||{
            println!("{} n°{} {}",message,i, ptr_x);
            println!("Au revoir n° {} {}",i,ptr_x)
        }
    );
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
}