use crate::task::Task;

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

    pub fn all(&self) -> &Vec<Task> {
        &self.tasks
    }
}
