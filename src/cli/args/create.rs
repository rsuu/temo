use crate::data::task;
pub struct Add {}

impl Add {
    pub fn add(id: usize, name: &str) -> task::Task {
        task::Task::new(id, name)
    }
}
