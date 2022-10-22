use std::io::stdin;

fn main() {
    println!("Hello, world!");
    let input = read_var() as u8;
    let mut mass = read_var();
    match input {
        1 => {println!("{mass}");}
        2 => {mass = mass * 0.000001}
        3 => {mass = mass * 0.001;}
        4 => {mass = mass * 1000.0;}
        5 => {mass = mass * 100.0;}
        _ => {println!("bfebfef")}
    }
    println!("{mass}");
}

fn read_var() -> f64 {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки
        break x;
    }
}