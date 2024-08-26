use std::error::Error;

use rusqlite::{params, Connection};

use crate::task::{Task, TaskStatus};

pub struct TaskManager {
    connection: Connection,
}

impl TaskManager {
    pub fn new() -> Result<TaskManager, Box<dyn Error>> {
        let connection_result = Connection::open("appdata.db");
        match connection_result {
            Ok(connection) => {
                // initializing the tasks table if not present
                let _ = connection.execute("CREATE TABLE IF NOT EXISTS tasks ('title' text, 'description' text, )", params![]);
                Ok(TaskManager {
                    connection
                })
            },
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn add_new_task(&mut self, task: Task) -> Result<(), Box<dyn Error>> {
        let mut preparred_statement = self.connection.prepare("INSERT INTO tasks (title, description, status) VALUES (?1, ?2, ?3)")?;
        let result = preparred_statement.execute(params![
            task.get_title(),
            task.get_description(),
            task.get_status()
        ])
            .map(|_| ())
            .map_err(|e| Box::new(e));

        Ok(result?)
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut preparred_statement = self.connection.prepare("SELECT * FROM tasks")?;
        let result = preparred_statement.query([])
            .map(|mut rows| {
                let mut tasks = vec![];
                loop {
                    let row = rows.next().unwrap();
                    match row {
                        Some(content) => {
                            let title: String = content.get("title").unwrap();
                            let description: String = content.get("description").unwrap();
                            let status: TaskStatus = content.get("status").unwrap();
                            
                            let mut task = Task::from(&title, Some(&description));
                            task.set_status(&status);

                            tasks.push(task);
                        }
                        None => break
                    }
                }
                tasks
            });

        Ok(result?)
    }

    pub fn get_task(&self, title: &str) -> Result<Task, Box<dyn Error>> {
        let task = self.get_all_tasks()?.into_iter().find(|task| task.get_title() == title);
        match task {
            Some(task) => Ok(task),
            None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Task not found"))),
        }
    }

    pub fn delete_task(&mut self, title: &str) {
        let mut preparred_statement = self.connection.prepare("DELETE * FROM tasks WHERE title=1?").unwrap();
        let _ = preparred_statement.execute(params![title]);
    }
}

#[cfg(test)]
mod tests {
    use super::TaskManager;


    #[test]
    fn test_new_taskmanager() {
        let manager_result = TaskManager::new();
        assert!(manager_result.is_ok())
    }

    #[test]
    fn test_add_new_task() {
        
    }

    #[test]
    fn test_add_two_tasks_with_same_title() {
        
    }

    #[test]
    fn test_get_all_tasks() {
        
    }

    #[test]
    fn test_get_task_by_title() {
        
    }

    #[test]
    fn test_delete_task() {
        
    }
}
