use crate::task::Task;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    pub fn get_by_id(&mut self, id: &str) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.id == id)
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task)
    }

    pub fn remove_task(&mut self, id: &str) -> bool {
        let index = self.tasks.iter().position(|task| task.id == id);
        match index {
            Some(i) => {
                self.tasks.remove(i);
                true
            }
            None => false,
        }
    }

    pub fn list_completed(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.is_completed).collect()
    }

    pub fn find_by_keyword(&self, keyword: &str) -> Vec<&Task> {
        let keyword = keyword.to_lowercase();
        self.tasks
            .iter()
            .filter(|task| task.title.to_lowercase().contains(&keyword))
            .collect()
    }

    pub fn all(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn save_to_file(&self, file_name: &str) {
        let mut file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_name)
            .unwrap();
        for task in self.all() {
            let serialized_task = serde_json::to_string(&task).unwrap();
            file.write_all(format!("{}\n", serialized_task).as_bytes())
                .unwrap();
        }
        println!("Saved task_list to: {:?}", file_name);
    }

    pub fn from_file(file_name: &str) -> Self {
        let mut task_list = Self::new();
        let file = File::open(&file_name).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let task: Task = serde_json::from_str(line.unwrap().as_str()).unwrap();
            task_list.add_task(task)
        }
        task_list
    }
}

impl IntoIterator for TaskList {
    type Item = Task;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tasks.into_iter()
    }
}
