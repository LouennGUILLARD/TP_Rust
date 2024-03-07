use clap::Parser;


#[derive(Debug, Parser)]
struct Options {
    n : usize
}
fn main(){
    
    let my_parametre: Options = Options::parse();
        for i in 0..my_parametre.n {
            let handler = std::thread::spawn(move || {
            println!("Bonjour n°{}", i);
            println!("Au revoir n°{}", i);
            });
            handler.puch()
        }
    

    handler.join().expect("fin de fils");
    
}