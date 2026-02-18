use crate::command::Command;
use crate::task_list::TaskList;

pub struct MenuOption<'a> {
    name: &'a str,
    command: Box<dyn Command + 'a>,
}

impl<'a> MenuOption<'a> {
    pub fn new(name: &'a str, command: Box<dyn Command + 'a>) -> Self {
        MenuOption { name, command }
    }
}

pub struct Menu<'a> {
    pub options: Vec<MenuOption<'a>>,
}

impl<'a> Menu<'a> {
    pub fn new() -> Self {
        Menu {
            options: Vec::new(),
        }
    }

    pub fn add_option(&mut self, menu_option: MenuOption<'a>) -> &mut Self {
        self.options.push(menu_option);
        self
    }

    pub fn display(&self) {
        for (index, option) in self.options.iter().enumerate() {
            println!(
                "{index}: {option_name}",
                index = index,
                option_name = option.name
            )
        }
    }

    pub fn execute_command(&mut self, option_index: usize, task_list: &mut TaskList) {
        if let Some(option) = self.options.get_mut(option_index) {
            option.command.execute(task_list);
        } else {
            println!(
                "Such option: {} does not exist. Choose option in range: [0..{}]",
                option_index,
                self.options.len() - 1
            );
        }
    }
}
