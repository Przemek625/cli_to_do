use std::io;

pub fn prompt_line(prompt: &str, hint: Option<&str>) -> String {
    let mut buffer = String::new();
    println!("{}", prompt);
    // hint.is_some()
    if let Some(h) = hint {
        println!("hint: {}", h);
    }

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read input");
    buffer.trim().to_owned()
}

pub fn parse_tags(tags: &str) -> Vec<String> {
    // tags.split(",").map(|tag| tag.trim().to_lowercase()).collect();
    tags.to_lowercase()
        .split(",")
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(String::from)
        .collect()
}
