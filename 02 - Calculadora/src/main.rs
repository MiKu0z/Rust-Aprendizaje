use std::io;
fn main() {
    loop {
        let mut opcion: String = String::new();

        println!(" ----- CALCULADORA -----");
        println!("1. Sumar \n2. Restar \n3. Salir");
        println!("Elige una opción:");
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer");
        let opcion: u8 = opcion.trim().parse().expect("No es una opción");

        if (opcion == 1) || (opcion == 2){
            let resultado: i32 = ingresar_datos(opcion);
            println!("El resultado es: {}", resultado);
        } else if opcion == 3 {
            println!("Hasta la proxima!");
            break;
        }
    }
}

fn ingresar_datos(opcion: u8) -> i32{
     let mut valor_1: String = String::new();
     let mut valor_2: String = String::new();
     println!("Valor 1:");
     io::stdin()
        .read_line(&mut valor_1)
        .expect("Error al leer");
    println!("Valor 2:");
    io::stdin()
        .read_line(&mut valor_2)
        .expect("Error al leer");
    let valor_1: i32 = valor_1.trim().parse().expect("El valor escrito no es numero!");
    let valor_2: i32 = valor_2.trim().parse().expect("El valor escrito no es numero!");

    if opcion == 1 {
        valor_1 + valor_2
    } else if opcion == 2 {
        valor_1 - valor_2
    } else {
        0
    }
}
