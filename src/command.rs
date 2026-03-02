use crate::constants::FILE_NAME;
use crate::task::Task;
use crate::task_list::TaskList;
use std::io;
use std::process::exit;

pub trait Command {
    fn execute(&mut self, task_list: &mut TaskList);
}

pub struct ExitCommand;

impl Command for ExitCommand {
    fn execute(&mut self, task_list: &mut TaskList) {
        let mut buffer = String::new();
        println!("Would you like to save your current work(y/n)?");
        io::stdin().read_line(&mut buffer).unwrap();
        let input = buffer.trim().to_lowercase();
        loop {
            match input.as_str() {
                "y" => {
                    task_list.save_to_file();
                    exit(0);
                }
                "n" => {
                    println!("Finished the program!");
                    exit(0);
                }
                _ => {
                    println!("Invalid option. Choose y or n.")
                }
            }
        }
    }
}

pub struct AddTaskCommand;

impl Command for AddTaskCommand {
    fn execute(&mut self, task_list: &mut TaskList) {
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
}

pub struct DeleteTaskCommand;

impl Command for DeleteTaskCommand {
    fn execute(&mut self, task_list: &mut TaskList) {
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
}

pub struct ListTasksCommand;

impl Command for ListTasksCommand {
    fn execute(&mut self, task_list: &mut TaskList) {
        for task in task_list.all() {
            println!("{}", task)
        }
    }
}

pub struct CompleteTaskCommand;

impl Command for CompleteTaskCommand {
    fn execute(&mut self, task_list: &mut TaskList) {
        println!("What is the task` if that you have completed?");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Error occurred");
        let task_id = buffer.trim();
        let task = task_list.get_by_id(task_id);
        match task {
            None => (),
            Some(task) => {
                task.is_completed = true;
                println!("task {} has been completed", task_id);
            }
        }
    }
}

pub struct FindTaskByKeywordCommand;

impl Command for FindTaskByKeywordCommand {
    fn execute(&mut self, task_list: &mut TaskList) {
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
}

pub struct SaveTaskList;

impl Command for SaveTaskList {
    fn execute(&mut self, task_list: &mut TaskList) {
        task_list.save_to_file()
    }
}
