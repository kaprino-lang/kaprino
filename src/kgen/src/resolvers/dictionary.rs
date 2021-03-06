use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
pub struct DictionaryItem<T> {
    content: T,
    id: String,
    scope_id: String
}

impl<T> DictionaryItem<T> {
    pub fn new(content: T, id: String, scope_id: String) -> DictionaryItem<T> {
        DictionaryItem { content, id, scope_id }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dictionary<T> {
    contents: Vec<DictionaryItem<T>>,
    scope: VecDeque<String>
}

impl<T> Dictionary<T> {
    pub fn new() -> Dictionary<T> {
        Dictionary { contents: Vec::new(), scope: VecDeque::new() }
    }

    pub fn add(&mut self, id: &str, content: T) {
        let content_with_scope = DictionaryItem::new(
            content,
            id.to_string(),
            self.get_current_scope()
        );
        self.contents.push(content_with_scope);
    }

    pub fn add_scope(&mut self, scope: &str) {
        self.scope.push_back(scope.to_string());
    }

    pub fn remove_scope(&mut self) {
        let scope_name = self.get_current_scope();
        self.contents.retain(|item| item.scope_id != scope_name);
        self.scope.pop_back();
    }

    pub fn find(&self, id: &str) -> Option<&T> {
        let content = self.contents
            .iter()
            .find(|item| item.id == id);
        match content {
            Some(item) => Some(&item.content),
            None => None
        }
    }

    pub fn find_mut(&mut self, id: &str) -> Option<&mut T> {
        let content = self.contents
            .iter_mut()
            .find(|item| item.id == id);
        match content {
            Some(item) => Some(&mut item.content),
            None => None
        }
    }

    fn get_current_scope(&self) -> String {
        match self.scope.back() {
            Some(scope_id) => scope_id.clone(),
            None => "__global".to_string()
        }
    }
}
