use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Ord,PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
pub struct Voter (pub String);

#[derive(Ord,PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
pub struct Candidate (pub String);

pub struct Score (pub usize);

#[derive(Debug)]
pub struct AttendenceSheet (pub Set<Voter>);

pub struct Scoreboard{
    
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score: Score,
}

impl Scoreboard{
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let mut scores = Map::new();
        let blank_score = Score(0);
        let invalid_score = Score(0);
        
        for candidate in candidates {
            scores.insert(candidate, Score(0));
        }
        
        Scoreboard {
            scores,
            blank_score,
            invalid_score,
        }
    }
}


pub struct BallotPaper{
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum VoteOutcome{
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

pub struct VotingMachine{
    pub voters: AttendenceSheet,
    pub scoreboard: Scoreboard,
}

impl VotingMachine {
    pub fn vote(&mut self, ballot_paper: BallotPaper ) -> VoteOutcome { 
        match ballot_paper.candidate {
            Some(candidate) => {
                if self.voters.0.contains(&ballot_paper.voter) {
                    VoteOutcome::HasAlreadyVoted(ballot_paper.voter)
                
                } else if !self.scoreboard.scores.contains_key(&candidate) {
                    self.voters.0.insert(ballot_paper.voter.clone());
                    self.scoreboard.invalid_score.0 += 1;
                    VoteOutcome::InvalidVote(ballot_paper.voter)
                } else {
                    self.voters.0.insert(ballot_paper.voter.clone());
                    self.scoreboard.scores.entry(candidate.clone()).and_modify(|score| score.0 += 1);
                    VoteOutcome::AcceptedVote(ballot_paper.voter, candidate)
                }
            }
            None => {
                self.voters.0.insert(ballot_paper.voter.clone());
                self.scoreboard.blank_score.0 += 1;
                VoteOutcome::BlankVote(ballot_paper.voter)
            }
        }
    }
    
    pub fn get_scoreboard(&self) -> &Scoreboard {
        if self.scoreboard.scores.is_empty() {
            println!("Il n'y a pas de candidats");
            &self.scoreboard
        }
        else {
            &self.scoreboard
        }
        
    }
    
    pub fn get_voters(&self) -> &AttendenceSheet {
        if self.voters.0.is_empty() {
            println!("Personnes n'a voter");
            &self.voters
        }
        else {
        &self.voters
        }
    
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accepte_vote() {
        let mut liste_candidats = Vec::new();
        liste_candidats.push(Candidate("candidat1".to_string()));
        liste_candidats.push(Candidate("candidat2".to_string()));
        let liste_candidats = Scoreboard::new(liste_candidats);
        let mut courrant_vote = VotingMachine {
            voters: AttendenceSheet(Set::new()),
            scoreboard: liste_candidats,
        };
        let votant = Voter("votant1".to_string());
        let candidat = Candidate("candidat1".to_string());
        let vote = BallotPaper{voter : votant.clone(), candidate : Some(candidat.clone())};
        let result = courrant_vote.vote(vote);
        assert_eq!(result, VoteOutcome::AcceptedVote(votant, candidat));
    }

    #[test]
    fn test_has_already_voted() {
        let mut liste_candidats = Vec::new();
        liste_candidats.push(Candidate("candidat1".to_string()));
        liste_candidats.push(Candidate("candidat2".to_string()));
        let liste_candidats = Scoreboard::new(liste_candidats);
        let mut courrant_vote = VotingMachine {
            voters: AttendenceSheet(Set::new()),
            scoreboard: liste_candidats,
        };
        let votant = Voter("votant1".to_string());
        let candidat = Candidate("candidat1".to_string());
        let vote1 = BallotPaper{voter : votant.clone(), candidate : Some(candidat.clone())};
        courrant_vote.vote(vote1);
        let vote2 = BallotPaper{voter : votant.clone(), candidate : Some(candidat.clone())};
        let result = courrant_vote.vote(vote2);
        assert_eq!(result, VoteOutcome::HasAlreadyVoted(votant));
    }

    #[test]
    fn test_blank_vote() {
        let mut liste_candidats = Vec::new();
        liste_candidats.push(Candidate("candidat1".to_string()));
        liste_candidats.push(Candidate("candidat2".to_string()));
        let liste_candidats = Scoreboard::new(liste_candidats);
        let mut courrant_vote = VotingMachine {
            voters: AttendenceSheet(Set::new()),
            scoreboard: liste_candidats,
        };
        let votant = Voter("votant1".to_string());
        let vote = BallotPaper{voter : votant.clone(), candidate : None};
        let result = courrant_vote.vote(vote);
        assert_eq!(result, VoteOutcome::BlankVote(votant));
    }

    #[test]
    fn test_invalid_vote() {
        let mut liste_candidats = Vec::new();
        liste_candidats.push(Candidate("candidat1".to_string()));
        liste_candidats.push(Candidate("candidat2".to_string()));
        let liste_candidats = Scoreboard::new(liste_candidats);
        let mut courrant_vote = VotingMachine {
            voters: AttendenceSheet(Set::new()),
            scoreboard: liste_candidats,
        };
        let votant = Voter("votant1".to_string());
        let candidat = Candidate("candidat3".to_string());
        let vote = BallotPaper{voter : votant.clone(), candidate : Some(candidat.clone())};
        let result = courrant_vote.vote(vote);
        assert_eq!(result, VoteOutcome::InvalidVote(votant));
    }
}
