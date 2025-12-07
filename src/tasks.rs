use serde::{Deserialize, Serialize};

/// Represents a single task in the system.
///
/// Each task has a title and a completion flag,
/// and it can belong to either the recurring or daily list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub completed: bool,
}

/// Central manager that owns all tasks and exposes operations on them.
///
/// The TaskManager keeps two separate collections:
/// - recurring tasks
/// - daily tasks
///
/// It also provides summary information, such as the number of
/// pending and completed tasks.
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    // Private: only TaskManager can modify these directly.
    recurring: Vec<Task>,
    daily: Vec<Task>,
}

impl TaskManager {
    /// Creates an empty `TaskManager` with no tasks.
    pub fn new() -> Self {
        Self {
            recurring: Vec::new(),
            daily: Vec::new(),
        }
    }

    /// Returns a read-only slice with all recurring tasks.
    pub fn recurring_tasks(&self) -> &[Task] {
        &self.recurring
    }

    /// Returns a read-only slice with all daily tasks.
    pub fn daily_tasks(&self) -> &[Task] {
        &self.daily
    }

    /// Returns the number of recurring tasks.
    pub fn recurring_len(&self) -> usize {
        self.recurring.len()
    }

    /// Returns the number of daily tasks.
    pub fn daily_len(&self) -> usize {
        self.daily.len()
    }

    /// Counts how many tasks (recurring + daily) are not completed yet.
    pub fn pending_count(&self) -> usize {
        let r = self.recurring.iter().filter(|t| !t.completed).count();
        let d = self.daily.iter().filter(|t| !t.completed).count();
        r + d
    }

    /// Counts how many tasks (recurring + daily) have been completed.
    pub fn completed_count(&self) -> usize {
        let r = self.recurring.iter().filter(|t| t.completed).count();
        let d = self.daily.iter().filter(|t| t.completed).count();
        r + d
    }

    /// Adds a new recurring task with the given title.
    pub fn add_recurring(&mut self, title: String) {
        self.recurring.push(Task {
            title,
            completed: false,
        });
    }

    /// Adds a new daily task with the given title.
    pub fn add_daily(&mut self, title: String) {
        self.daily.push(Task {
            title,
            completed: false,
        });
    }

    /// Marks the recurring task at the given index as completed.
    ///
    /// Returns an error if the index is out of range.
    pub fn mark_recurring_completed(&mut self, index: usize) -> Result<(), String> {
        if index >= self.recurring.len() {
            return Err("Index out of range.".to_string());
        }
        self.recurring[index].completed = true;
        Ok(())
    }

    /// Marks the daily task at the given index as completed.
    ///
    /// Returns an error if the index is out of range.
    pub fn mark_daily_completed(&mut self, index: usize) -> Result<(), String> {
        if index >= self.daily.len() {
            return Err("Index out of range.".to_string());
        }
        self.daily[index].completed = true;
        Ok(())
    }

    /// Deletes the recurring task at the given index and returns it.
    ///
    /// Returns an error if the index is out of range.
    pub fn delete_recurring(&mut self, index: usize) -> Result<Task, String> {
        if index >= self.recurring.len() {
            return Err("Index out of range.".to_string());
        }
        Ok(self.recurring.remove(index))
    }

    /// Deletes the daily task at the given index and returns it.
    ///
    /// Returns an error if the index is out of range.
    pub fn delete_daily(&mut self, index: usize) -> Result<Task, String> {
        if index >= self.daily.len() {
            return Err("Index out of range.".to_string());
        }
        Ok(self.daily.remove(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_recurring_task() {
        let mut manager = TaskManager::new();
        manager.add_recurring("Go to gym".to_string());

        assert_eq!(manager.recurring_len(), 1);
        assert_eq!(manager.pending_count(), 1);
        assert_eq!(manager.completed_count(), 0);
    }

    #[test]
    fn test_add_daily_task() {
        let mut manager = TaskManager::new();
        manager.add_daily("Doctor appointment".to_string());

        assert_eq!(manager.daily_len(), 1);
        assert_eq!(manager.pending_count(), 1);
        assert_eq!(manager.completed_count(), 0);
    }

    #[test]
    fn test_mark_recurring_task_completed() {
        let mut manager = TaskManager::new();
        manager.add_recurring("Study Rust".to_string());

        manager.mark_recurring_completed(0).unwrap();

        assert_eq!(manager.completed_count(), 1);
        assert_eq!(manager.pending_count(), 0);
    }

    #[test]
    fn test_mark_daily_task_completed() {
        let mut manager = TaskManager::new();
        manager.add_daily("Buy food".to_string());

        manager.mark_daily_completed(0).unwrap();

        assert_eq!(manager.completed_count(), 1);
        assert_eq!(manager.pending_count(), 0);
    }

    #[test]
    fn test_delete_recurring_task() {
        let mut manager = TaskManager::new();
        manager.add_recurring("Clean room".to_string());

        manager.delete_recurring(0).unwrap();

        assert_eq!(manager.recurring_len(), 0);
        assert_eq!(manager.pending_count(), 0);
        assert_eq!(manager.completed_count(), 0);
    }

    #[test]
    fn test_delete_daily_task() {
        let mut manager = TaskManager::new();
        manager.add_daily("Pick up mail".to_string());

        manager.delete_daily(0).unwrap();

        assert_eq!(manager.daily_len(), 0);
        assert_eq!(manager.pending_count(), 0);
        assert_eq!(manager.completed_count(), 0);
    }

    #[test]
    fn test_pending_and_completed_counts() {
        let mut manager = TaskManager::new();

        manager.add_recurring("A".to_string());
        manager.add_daily("B".to_string());

        assert_eq!(manager.pending_count(), 2);
        assert_eq!(manager.completed_count(), 0);

        manager.mark_daily_completed(0).unwrap();

        assert_eq!(manager.pending_count(), 1);
        assert_eq!(manager.completed_count(), 1);
    }
}
