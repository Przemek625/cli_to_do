use std::io;
use std::string::String;
mod command;
mod constants;
mod menu;
mod task;
mod task_list;

use crate::command::{
    AddTaskCommand, CompleteTaskCommand, DeleteTaskCommand, ExitCommand, FindTaskByKeywordCommand,
    ListTasksCommand, SaveTaskList,
};
use crate::constants::FILE_NAME;
use crate::menu::{Menu, MenuOption};
use crate::task_list::TaskList;
use config::Config;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    file_name: String,
}

fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("default"))
        .add_source(config::File::with_name("config").required(false))
        .build()
        .expect("Failed to load config");

    let app_config: AppConfig = config.try_deserialize().expect("Failed to load App Config");

    println!("app_config: {:?}", app_config);

    let mut task_list = TaskList::from_file(&app_config.file_name);

    let mut menu = Menu::new();
    menu.add_option(MenuOption::new("Exit", Box::new(ExitCommand)))
        .add_option(MenuOption::new("Add Task", Box::new(AddTaskCommand)))
        .add_option(MenuOption::new("Delete Task", Box::new(DeleteTaskCommand)))
        .add_option(MenuOption::new("List Tasks", Box::new(ListTasksCommand)))
        .add_option(MenuOption::new(
            "Complete Task",
            Box::new(CompleteTaskCommand),
        ))
        .add_option(MenuOption::new(
            "Find By KeyWord Task",
            Box::new(FindTaskByKeywordCommand),
        ))
        .add_option(MenuOption::new("Save Tasks", Box::new(SaveTaskList)));

    let mut buffer = String::new();

    loop {
        menu.display();
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let selected_option: usize = buffer.trim().parse().unwrap();

        menu.execute_command(selected_option, &mut task_list)
    }
}
