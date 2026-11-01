use std::io;

fn main() {
    let mut n: String = String::new();
    let mut sucesion: i32 = 0;
    let mut num_anterior: i32 = 1;

    println!(" ----- SECUENCIA  FIBONACCI -----");
    println!("Digite la cantidad de numeros a recorrer:");
    io::stdin()
        .read_line(&mut n)
        .expect("Error al leer");
    let n: u32 = n.trim().parse().expect("El valor elegido no es positivo/numero!");

    println!("\nAqui tienes la cantidad de numeros elegida:");
    for _i in 0..n {
        print!("{} ", sucesion);
        let num_siguiente = sucesion + num_anterior;
        sucesion = num_anterior;
        num_anterior = num_siguiente;
    }
}
