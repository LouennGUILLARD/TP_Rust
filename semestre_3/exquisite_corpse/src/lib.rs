
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn make_reponse(line: &str, text: &mut String) -> String {
    let encadrer = "***************************************\n".to_string();
    let encadrer2 = encadrer.clone();
    text.push_str(line);
    let result = format!("{}{}{}", encadrer, text, encadrer2);
    return result;
}