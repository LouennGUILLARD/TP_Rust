use std::io::{self};
// use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;
use v040_memory::configuration;
use v040_memory::domain::{AttendenceSheet, BallotPaper, Candidate, Scoreboard, Voter, VotingMachine};
use configuration::Configuration;

pub fn run_app(configuration:Configuration) -> anyhow::Result<()>{
    let stdin = io::stdin();

    let liste_votants = AttendenceSheet(Set::new());

    let liste_new_candidats = configuration.candidates;
    let liste_new_candidats = liste_new_candidats.iter().map(|c| Candidate(c.to_string())).collect::<Vec<_>>();
    let liste_candidats = Scoreboard::new(liste_new_candidats);
    let mut courrant_vote = VotingMachine {
        voters: liste_votants,
        scoreboard: liste_candidats,
    };
    
    

    for phrase in stdin.lines() {
        match phrase {
            Ok(line) => {
                
                if line == "exit" {
                    break;
                }
                if line == "votants" {
                    
                    for element in courrant_vote.get_voters().0.iter() {
                        println!("{}", element.0.to_string());
                    }
                }

                else if line == "scores"{
                    let liste_scores = courrant_vote.get_scoreboard();
                    for (candidat, score) in liste_scores.scores.iter() {
                        println!("{} : {}", candidat.0, score.0);
                    }
                }
                
                else if line.trim().starts_with("voter ") {
                    let mut courant_line = line.trim().split_whitespace();
                    courant_line.next();
                    let votant: Option<&str> = courant_line.next();
                    let candidat: Option<&str> = courant_line.next();
                    match votant {
                        Some(votant) => {
                            courrant_vote.vote(BallotPaper{voter : Voter(votant.to_string()), candidate : candidat.map(|c| Candidate(c.to_string()))}); 
                        }
                        None => {
                            println!("Vous avez oubliez le nom du votant!");
                        }
                    }
                }
                
                
            

                else if line=="voter" || line=="voter "{
                    println!("Vous avez oublié le nom du votant");
                }

                else {
                    println!("Commande inconnue, veuillez réessayer avec une commande valide suivant : voter, votants, scores, exit");
                }
            }
            
            Err(error) => {
                return Err(error.into());
            }
        }
    }
    Ok(())
}