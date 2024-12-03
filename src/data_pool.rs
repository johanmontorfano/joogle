use std::{collections::VecDeque, sync::{Arc, Mutex}};

/// A `DataPool` is a set of data that is accessed for read/write by multiple
/// threads. The goal of this struct is to make this data available quickly 
/// while allowing complex operations.
/// This is highly useful for threaded bots.
pub struct DataPool<T> {
    dataset: Arc<Mutex<VecDeque<T>>>
} 

impl DataPool<String> {
    pub fn init() -> Self {
        Self { dataset: Arc::new(Mutex::new(VecDeque::new())) }
    }
    
    pub fn add_batch(&mut self, batch: Vec<String>) {
        self.dataset.lock().unwrap().append(&mut VecDeque::from(batch));
    }

    #[allow(dead_code)]
    pub fn add_item(&mut self, item: String) {
        self.dataset.lock().unwrap().push_back(item);
    }

    pub fn get_next(&mut self) -> Option<String> {
        self.dataset.lock().unwrap().pop_front()
    }

    pub fn get_content(&self) -> Vec<String> {
        self.dataset.lock().unwrap().clone().into_iter().collect()
    }
}
