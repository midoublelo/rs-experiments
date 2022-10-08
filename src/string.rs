pub fn vowel_printer(name: &str) {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    for letter in name.chars() {
        if vowels.contains(&letter) {
            println!("{}", letter);
        }
    }
}