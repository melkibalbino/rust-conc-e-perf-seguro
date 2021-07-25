fn main() {
    let bol1: bool = true;
    let bol2: bool = false;
    println!("bol1 == bol2 -> {}", bol1 == bol2);
    println!("bol1.eq(bol2) -> {}", bol1.eq(&bol2));
    println!("bol1 != bol2 -> {}", bol1 != bol2);
    println!("bol1.ne(bol2) -> {}\n", bol1.ne(&bol2));

    let char1: char = 'a';
    let char2: char = 'b';
    println!("char1 == char2 -> {}", char1 == char2);
    println!("char1.eq(char2) -> {}", char1.eq(&char2));
    println!("char1 != char2 -> {}", char1 != char2);
    println!("char1.ne(char2) -> {}\n", char1.ne(&char2));

    let int1: i32 = 1;
    let int2: i32 = 2;
    println!("int1 == int2 -> {}", int1 == int2);
    println!("int1.eq(int2) -> {}", int1.eq(&int2));
    println!("int1 != int2 -> {}", int1 != int2);
    println!("int1.ne(int2) -> {}\n", int1.ne(&int2));

    let float1: f32 = 1.3;
    let float2: f32 = 2.4;
    println!("float1 == float2 -> {}", float1 == float2);
    println!("float1.eq(float2) -> {}", float1.eq(&float2));
    println!("float1 != float2 -> {}", float1 != float2);
    println!("float1.ne(float2) -> {}\n", float1.ne(&float2));

    let str1: &'static str = "Marcelo";
    let str2: &'static str = "Castellani";
    println!("str1 == str2 -> {}", str1 == str2);
    println!("str1.eq(str2) -> {}", str1.eq(str2));
    println!("str1 != str2 -> {}", str1 != str2);
    println!("str1.ne(str2) -> {}\n", str1.ne(str2))
}
