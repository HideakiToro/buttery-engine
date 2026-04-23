use std::collections::HashMap;

pub struct Registry<T> {
    pub to_create: HashMap<String, T>,
    pub to_delete: HashMap<String, T>,
}

impl<T> Registry<T> {
    pub fn default() -> Self {
        Self {
            to_create: HashMap::new(),
            to_delete: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.to_create.clear();
        self.to_delete.clear();
    }
}
