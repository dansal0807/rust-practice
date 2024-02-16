struct User {
    username: String,
    email: String,
    ativo: bool
}

fn main() {

    let pessoa = User {username: String::from("Daniel"), email: String::from("daniel@gmail.com"), ativo: true};

    println!("meu nome eh {}, meu email eh {} e eh {}", pessoa.username, pessoa.email, pessoa.ativo)

}