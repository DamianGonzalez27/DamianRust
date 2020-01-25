use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Mi primer programa con Rust");
    
    let secret_number = rand::thread_rng().gen_range(1,100);

    println!("El numero secreto es: {}", secret_number);

    println!("Por favor ingresa un numero");

    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect("Fallo al leer la linea");

    println!("Tu numero es: {}", guess);
}
