use clap::Parser;
//use std::thread;

#[derive(Debug,Parser)]
struct Options{
    n : i32
}

#[tokio::main]
async fn main(){
    let args = Options::parse();

    let mut handles = vec![];

    for i in 0..args.n{
        let handle = tokio::spawn(async move{
            println!("Bonjour n°{}",i);
            println!("Au revoir n°{}",i);
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.await.unwrap();
    }
}