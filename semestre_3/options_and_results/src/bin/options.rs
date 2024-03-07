fn main(){
    let sentence1 = "Bonjour Limoges";
    let sentence2 = "";

    print_first_word1(sentence1);
    print_first_word1(sentence2);

    iterative_over_words(sentence1);
    iterative_over_words(sentence2);
}

fn print_first_word1(chaine:&str){
    let mut words = chaine.split_whitespace();
    let word1 = words.next();
    match (word1){
        None=> println!("La Chaine est vide"),
        Some(w1) => println!("Le premier mot est :{}", w1)
    }
}

fn print_first_word2(chaine:&str){
    let mut words = chaine.split_whitespace();
    let word1 = words.next();
    word1.expect("La chaîne doit être non vide.");
}

fn iterative_over_words(chaine:&str){
    let mots: Vec<&str> = chaine.split_whitespace().collect();
    for mot in mots{
        println!("Mot : {}",mot);
    }
}

