use std::io;
use std::string::String;
mod command;
mod constants;
mod menu;
mod task;
mod task_list;

use crate::command::{
    AddTaskCommand, CompleteTaskCommand, DeleteTaskCommand, ExitCommand, FindTaskByKeywordCommand,
    ListTasksCommand,
};
use crate::constants::FILE_NAME;
use crate::menu::{Menu, MenuOption};
use crate::task_list::TaskList;
use std::io::{BufRead, Write};

fn main() {
    let mut task_list = TaskList::from_file(&FILE_NAME);

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
        ));

    let mut buffer = String::new();

    loop {
        menu.display();
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        let selected_option: usize = buffer.trim().parse().unwrap();

        menu.execute_command(selected_option, &mut task_list)
    }
}
