fn to_pig_latin(sentence: String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut new_words: Vec<String> = Vec::new();
    for word in sentence.split_whitespace() {
        let ch = word.chars().nth(0).unwrap();
        if vowels.contains(&ch) {
            new_words.push(format!("{word}hay"));
        } else {
            let mut tmp_word = format!("{word}{ch}ay");
            tmp_word.remove(0);
            new_words.push(tmp_word);
        }
    }
    new_words.join(" ")
}
fn main() {
    println!("{}", to_pig_latin("good morning".to_string()));
    println!("{}", to_pig_latin("first apple of the day".to_string()));
}
