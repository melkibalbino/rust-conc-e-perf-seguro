
fn is_vowel_or_consonant(c: char) -> char {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => 'v',
        'A' | 'E' | 'I' | 'O' | 'U' => 'v',
        _ => 'c'
    }
}

fn main() {
    let name: &'static str = "Marcelo";
    let mut vowel_count = 0;
    let mut consonant_count = 0;

    for a in name.chars() {
        match is_vowel_or_consonant(a) {
            'v' => vowel_count += 1,
            'c' => consonant_count += 1,
            _ => ()
        }
    }

    println!("{} has {} vowel(s) and {} consonant(s)",
            name, vowel_count, consonant_count);

    // Vinculação de match passa @ e antes o nome da variavel para receber o valor Ex:
    for a in name.chars() {
        match is_vowel_or_consonant(a) {
            r @ 'v' => println!("{}", r),
            r @ 'c' => println!("{}", r),
            _ => ()
        }
    }
}
