use clap::Parser;


#[derive(Parser, Debug)]
pub struct Configuration{
    #[arg(short = 'c', long = "candidates", required = true, num_args = 1.., help = "Liste des candidats")]
    pub candidates: Vec<String>,
}

