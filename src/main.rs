use crate::task::Task;
use std::io;
use std::string::String;
mod task;
mod task_list;

use crate::task_list::TaskList;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
const FILE_NAME: &str = "tasks.jsonl";

fn handle_add_task(task_list: &mut TaskList) {
    let mut buffer = String::new();
    println!("Task title:");
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    let title = String::from(buffer.trim());
    let task = Task::new(&title);
    task_list.add_task(task);
    println!("Added a new task")
}

fn handle_delete_task(task_list: &mut TaskList) {
    println!("What is the task`s id?");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Exception occurred");
    let task_id = buffer.trim();
    if (task_list.remove_task(&task_id)) {
        println!("Task: {:?} has been deleted", task_id)
    } else {
        println!("Task: {:?} is not on the list", task_id)
    }
}

fn handle_save_task_list(task_list: &TaskList) {
    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&FILE_NAME)
        .unwrap();
    for task in task_list.all() {
        let serialized_task = serde_json::to_string(&task).unwrap();
        file.write_all(format!("{}\n", serialized_task).as_bytes())
            .unwrap();
    }
    println!("Saved task_list to: {:?}", FILE_NAME);
}

fn load_task_list_from_file() -> TaskList {
    let file = File::open(&FILE_NAME).unwrap();
    let mut reader = BufReader::new(file);
    let mut task_list = TaskList::new();
    for line in reader.lines() {
        let task: Task = serde_json::from_str(line.unwrap().as_str()).unwrap();
        task_list.add_task(task)
    }
    task_list
}

fn handle_exit(task_list: &TaskList) {
    let mut buffer = String::new();
    println!("Would you like to save your current work(y/n)?");
    io::stdin().read_line(&mut buffer).unwrap();
    let input = buffer.trim().to_lowercase();
    loop {
        match input.as_str() {
            "y" => {
                handle_save_task_list(task_list);
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

fn handle_list_tasks(task_list: &TaskList) {
    for task in task_list.all() {
        println!("{}", task)
    }
}

fn handle_task_completed(task_list: &mut TaskList) -> bool {
    println!("What is the task` if that you have completed?");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Error occurred");
    let task_id = buffer.trim();
    let task = task_list.get_by_id(task_id);
    match task {
        None => false,
        Some(t) => {
            t.is_completed = true;
            println!("task {} has been completed", task_id);
            true
        }
    }
}

fn handle_list_completed_tasks(task_list: &mut TaskList) {
    for task in task_list.list_completed() {
        println!("{}", task)
    }
}

fn handle_find_tasks_by_keyword(task_list: &TaskList) {
    println!("keyword:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let keyword = buffer.trim();
    let filtered_tasks = task_list.find_by_keyword(keyword);
    if filtered_tasks.is_empty() {
        println!("There are no matching results for the query: {}", keyword)
    } else {
        for task in filtered_tasks {
            println!("{}", task);
        }
    }
}

fn print_options() {
    println!("0: Exit");
    println!("1: Add new task");
    println!("2: Save task_list list");
    println!("3: Delete task");
    println!("4: List tasks");
    println!("5: List completed tasks");
    println!("6: Set task completed");
    println!("7: List task matching keyword")
}
fn print_menu() {
    println!("\n=== To-Do CLI Menu ===");
    print_options();
    print!("Enter your choice: \n");
}

fn main() {
    let mut task_list = load_task_list_from_file();

    let mut buffer = String::new();

    loop {
        print_menu();
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let input = buffer.trim();
        match input {
            "0" => {
                handle_exit(&mut task_list);
                break;
            }
            "1" => {
                handle_add_task(&mut task_list);
            }
            "2" => {
                handle_save_task_list(&mut task_list);
            }
            "3" => {
                handle_delete_task(&mut task_list);
            }
            "4" => {
                handle_list_tasks(&mut task_list);
            }
            "5" => {
                handle_list_completed_tasks(&mut task_list);
            }
            "6" => {
                handle_task_completed(&mut task_list);
            },
            "7" => {
                handle_find_tasks_by_keyword(&mut task_list);
            }
            _ => {
                println!("Invalid option one of:");
                print_options();
            }
        }
    }
}
