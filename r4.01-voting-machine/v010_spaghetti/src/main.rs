use std::io::{self};
use std::collections::BTreeMap as Map;


fn main () -> anyhow::Result<()>{
    let stdin = io::stdin();

    let mut liste_votants = vec!["Jean".to_string(), "Paul".to_string(), "Jacques".to_string(),"AXEL".to_string()];

    let mut liste_candidats = Map::new();
    liste_candidats.insert("NixOS".to_string(), 30);
    liste_candidats.insert("Windows".to_string(), 50);
    liste_candidats.insert("MacOS".to_string(), 2);

    for phrase in stdin.lines() {
        match phrase {
            Ok(line) => {
                
                if line == "exit" {
                    break;
                }
                else if line == "votants" {
                    
                    for element in liste_votants.iter() {
                        println!("{}", element);
                    }
                }
                else if line == "scores"{
                    for (key, value) in liste_candidats.iter() {
                        println!("{} : {}", key, value);
                    }
                }
                
                else if line.trim().starts_with("voter ") {
                    let mut courant_line = line.trim().split_whitespace();
                    courant_line.next();
                    let votant: Option<&str> = courant_line.next();
                    let candidat: Option<&str> = courant_line.next();
                    match votant {
                        Some(votant) => {
                            
                            if liste_votants.contains(&votant.to_string()) {
                                println!("Vous avez déjà voté !");
                                continue;
                            }
                            
                            else {
                                match candidat
                                {
                                    Some(candidat) => {
                                        if let Some(score) = liste_candidats.get_mut(candidat) {
                                            *score += 1;
                                            println!("{} a voté pour {}", votant.to_string(), candidat.to_string());
                                            liste_votants.push(votant.to_string());
                                        } else {
                                            println!("Le candidat entré n'existe pas, {} a fait un vote nul", votant.to_string());
                                        liste_votants.push(votant.to_string());
                                        }
                                    }
                                    
                                    None => {
                                        println!("{} a voté pour Blanc", votant.to_string());
                                        liste_votants.push(votant.to_string());
                                    } 
                                }
                            }
                        }
                        None => {
                            println!("Vous avez oubliez le nom du votant!");
                        }
                    }
                    
                }
                
            

                else if line=="voter" || line=="voter "{
                    println!("Vous avez oublié le nom du candidat");
                }

                else {
                    println!("Commande inconnue, veuillez réessayer avec une commande valide suivant : vote, votants, scores, exit");
                }
            }
            
            Err(error) => {
                return Err(error.into());
            }
        }
    }
    Ok(())
}
