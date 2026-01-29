use crate::task::Task;
use std::io;
use std::string::String;
mod task;
mod task_list;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use uuid::Uuid;

const FILE_NAME: &str = "tasks.jsonl";

fn handle_add_task(tasks: &mut Vec<Task>) {
    let mut buffer = String::new();
    println!("Task title:");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    let task = Task {
        id: Uuid::new_v4().to_string(),
        title: String::from(buffer.trim()),
        is_completed: false,
    };
    tasks.push(task);
    println!("Added a new task: {:?}", tasks.last().unwrap().title)
}

fn handle_save_tasks(tasks: &Vec<Task>) {
    let mut file = File::options().create(true).write(true).open(&FILE_NAME).unwrap();
    for task in tasks {
        let serialized_task = serde_json::to_string(&task).unwrap();
        file.write_all(format!("{}\n", serialized_task).as_bytes())
            .unwrap();
    }
    println!("Saved tasks to: {:?}", FILE_NAME);
}

fn read_tasks() -> Vec<Task> {
    let file = File::open(&FILE_NAME).unwrap();
    let mut reader = BufReader::new(file);
    let mut tasks: Vec<Task> = Vec::new();
    for line in reader.lines() {
        let task:Task = serde_json::from_str(line.unwrap().as_str()).unwrap();
        tasks.push(task)
    }
    tasks
}

fn handle_exit(tasks: &Vec<Task>) {
    let mut buffer = String::new();
    println!("Would you like to save your current work(y/n)?");
    io::stdin().read_line(&mut buffer).unwrap();
    let input = buffer.trim().to_lowercase();
    loop {
        match input.as_str() {
            "y" => {
                handle_save_tasks(tasks);
                break;
            }
            "n" => {
                break;
            }
            _ => {
                println!("Invalid option. Choose y or n.")
            }
        }
    }

    println!("Finished the program!");
}

fn print_options() {
    println!("0: Exit");
    println!("1: Add new task");
    println!("2: Save tasks list");
}
fn print_menu() {
    println!("\n=== To-Do CLI Menu ===");
    print_options();
    print!("Enter your choice: \n");
}

fn main() {
    let mut tasks: Vec<Task> = read_tasks();

    let mut buffer = String::new();

    loop {
        print_menu();
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let input = buffer.trim();
        match input {
            "0" => {
                handle_exit(&mut tasks);
                break;
            }
            "1" => {
                handle_add_task(&mut tasks);
            }
            "2" => {
                handle_save_tasks(&mut tasks);
            }
            _ => {
                println!("Invalid option one of:");
                print_options();
            }
        }
    }
}
