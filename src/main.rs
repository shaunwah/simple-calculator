use std::io;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Input something!");

        if input.contains("quit") {
            break;
        }

        let result = calculate(&input);
        let output = match result {
            Some(v) => v.to_string(),
            None => "Invalid input!".to_string(),
        };

        println!("{}", output);
    }
}

fn calculate(s: &str) -> Option<f32> {
    let input: Vec<&str> = s.split_whitespace().collect();

    if input.len() == 3 {
        let a: f32 = input.first().unwrap().parse().unwrap(); // TODO error handling
        let b: f32 = input.last().unwrap().parse().unwrap();
        let sign: char = input.get(1).unwrap().parse().unwrap();

        match sign {
            '+' => Some(a + b),
            '-' => Some(a - b),
            '*' => Some(a * b),
            '/' => Some(a / b),
            _ => None,
        }
    } else {
        None
    }
}
