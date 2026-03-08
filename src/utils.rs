use std::io;

pub fn prompt_line(prompt: &str) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input");
    buffer.trim().to_owned()
}
