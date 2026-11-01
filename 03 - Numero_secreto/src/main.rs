use std::io;

fn main() {
    let numero_secreto = 42; // Nuestro número fijo

    loop {
        println!("\nIngresa tu intento:");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la linea");

        // -------------------------------------------------------------
        // PRIMER MATCH: Convertir texto a número de forma segura
        // -------------------------------------------------------------
        let intento: i32 = match entrada.trim().parse() {
            Ok(numero) => numero, // Si el parseo funcionó, extrae `numero` y se lo guarda a `intento`
            Err(_) => {
                // Si el usuario escribió letras, entra aquí:
                println!("Por favor, ingresa un número válido (no letras).");
                continue; // Vuelve al inicio del `loop` sin seguir evaluando abajo
            }
        };

        // -------------------------------------------------------------
        // SEGUNDO MATCH: Comparar el intento con el número secreto
        // -------------------------------------------------------------
        match intento {
            n if n < numero_secreto => println!("Demasiado bajo, intenta de nuevo."),
            n if n > numero_secreto => println!("Demasiado alto, intenta de nuevo."),
            _ => {
                println!("¡Felicidades! Adivinaste el número.");
                break; // Rompe el `loop` y termina el programa
            }
        }
    }
}