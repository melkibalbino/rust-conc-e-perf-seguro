trait User {
    // Temos um construtor onde vamos receber um nome de 
    // usuário (login)
    fn new(username: &'static str) -> Self;

    // Retorna o login definido em new
    fn username(&self) -> &'static str;

    // Logar-se no sistema
    fn login(&self) -> &'static str;

    // Deslogar-se no sistema
    fn logout(&self) -> &'static str;

    // Verificar se está logado
    fn is_logged_in(&self) -> bool {
        false
    }
}

#[derive(Debug)]
struct Admin { username: &'static str }
struct Operador { username: &'static str }
struct BasicUser { username: &'static str }

impl User for Admin {
    fn new(username: &'static str) -> Admin {
        Admin { username: username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Usuário do tipo ADMIN entrou no sistema"
    }


    fn logout(&self) -> &'static str {
        "Usuário do tipo ADMIN saiu no sistema"
    }
}

impl User for Operador {
    fn new(username: &'static str) -> Operador {
        Operador { username: username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Usuário do tipo OPERADOR entrou no sistema"
    }


    fn logout(&self) -> &'static str {
        "Usuário do tipo OPERADOR saiu no sistema"
    }
}

impl User for BasicUser {
    fn new(username: &'static str) -> BasicUser {
        BasicUser { username: username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Usuário do tipo BASICO entrou no sistema"
    }


    fn logout(&self) -> &'static str {
        "Usuário do tipo BASICO saiu no sistema"
    }
}

fn main() {
    let admin: Admin = User::new("Corleone");
    println!("Bem-Vindo usuario {}", admin.username());
    println!("{}", admin.login());
    println!("{}", admin.logout());
    println!("{:?}", admin);

    let operador: Operador = User::new("pessoa qualquer 01");
    println!("Bem-Vindo usuario {}", operador.username());
    println!("{}", operador.login());
    println!("{}", operador.logout());

    let basic_user: BasicUser = User::new("pessoa qualquer 02");
    println!("Bem-Vindo usuario {}", basic_user.username());
    println!("{}", basic_user.login());
    println!("{}", basic_user.logout())
}
