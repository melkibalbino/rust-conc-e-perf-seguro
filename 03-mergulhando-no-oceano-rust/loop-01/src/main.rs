fn main() {
    let mut a = 0;
    loop { // semelhante ao while true
        // Para cancelar e so usar o comando CTRL + c
        // Se não estiver usando o break
        println!("Running forever...");
        a += 1;
        if a >= 10 {break} // loop usando break pode ser trocado pelo while facilmente
    }
}
