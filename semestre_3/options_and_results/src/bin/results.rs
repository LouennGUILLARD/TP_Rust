fn main(){
    let string1 = "-17";
    let string2 = "Tux";

    print!("{}",convert_to_int1(string1));
}

fn convert_to_int1(chaine:&str)->i32{
    let inte = chaine.parse::<i32>().unwrap();
    return inte;
}