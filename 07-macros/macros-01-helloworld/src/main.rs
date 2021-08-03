macro_rules! hello {
    () => { println!("Hello, world!"); }
}
fn hello() {
    println!("Hello, world!");
}

fn main() {
    // usando "nm nome_do_arquivo | grep nome_do_arquivo" no terminal
    // é mostrado que cada função retorna um simbolo mesmo apos copilado e o macro não
    // o chamado do macro é subistituido por codigo antes de ser gerado o binario
    hello!();
    hello()
}
