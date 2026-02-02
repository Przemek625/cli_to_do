use crate::task::Task;

pub struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    pub fn get_by_id(&self, id: &str) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id == id)
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
            None => false
        }
    }

    pub fn all(&self) -> &Vec<Task> {
        &self.tasks
    }
}
