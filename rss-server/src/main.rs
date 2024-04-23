use std::io;


#[tokio::main]
async fn main() {
    let mut guess = String::new();

    match  io::stdin().read_line(&mut guess) {
        Ok(n) => {
            println!("{n} is read");
            println!("{guess} is read");

        } ,
        Err(e) => {
            println!("{e} is thrown");
        }
    }
}
