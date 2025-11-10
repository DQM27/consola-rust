use std::io;

fn main() {
   
   saludar_usuario();

}

fn saludar_usuario(){
    println!("Cual es tu nombre");

    let mut nombre = String::new();

    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer el nombre");

    let nombre = nombre.trim();

    println! ("Hola {} Eres una zorra Bienvenido a Rust.", nombre);
}