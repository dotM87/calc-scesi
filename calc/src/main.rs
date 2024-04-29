use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Uso: {} <operacion> <numero1> <numero2>", args[0]);
        println!("Operaciones válidas: sum, rest, mult, div");
        return;
    }

    let operacion = &args[1];
    let num1: f64 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El segundo argumento no es un número válido.");
            return;
        },
    };
    let num2: f64 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: El tercer argumento no es un número válido.");
            return;
        },
    };

    let resultado = match operacion.as_str() {
        "sum" => sumar(num1, num2),
        "res" => restar(num1, num2),
        "mul" => multiplicar(num1, num2),
        "div" => dividir(num1, num2),
        _ => {
            println!("Error: Operación no válida.");
            return;
        },
    };

    mostrar_resultado(operacion, num1, num2, resultado);
}

fn sumar(a: f64, b: f64) -> f64 {
    a + b
}

fn restar(a: f64, b: f64) -> f64 {
    a - b
}

fn multiplicar(a: f64, b: f64) -> f64 {
    a * b
}

fn dividir(a: f64, b: f64) -> f64 {
    if b != 0.0 {
        a / b
    } else {
        println!("Error: división por cero");
        f64::NAN
    }
}

fn mostrar_resultado(operacion: &str, num1: f64, num2: f64, resultado: f64) {
    println!("__________________");
    println!("| OPERACION: {} |", operacion.to_uppercase());
    println!("|----------------|");
    println!("| {:.0} \t| {:.0} \t |", num1, num2);
    println!("|----------------|");
    println!("| RESULTADO      |");
    println!("|----------------|");
    println!("| {:.0}  \t\t |", resultado);
    println!("------------------");
}
