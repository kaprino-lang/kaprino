use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct DictionaryItem<'ctx, T> {
    content: &'ctx T,
    id: &'ctx str,
    scope_id: &'ctx str
}

impl<'ctx, T> DictionaryItem<'ctx, T> {
    pub fn new(content: &'ctx T, id: &'ctx str, scope_id: &'ctx str) -> DictionaryItem<'ctx, T> {
        DictionaryItem { content, id, scope_id }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dictionary<'ctx, T> {
    contents: Vec<DictionaryItem<'ctx, T>>,
    scope: VecDeque<&'ctx str>
}

impl<'ctx, T> Dictionary<'ctx, T> {
    pub fn new() -> Dictionary<'ctx, T> {
        Dictionary { contents: Vec::new(), scope: VecDeque::new() }
    }

    pub fn add(&mut self, id: &'ctx str, content: &'ctx T) {
        let content_with_scope = DictionaryItem::new(
            content,
            id,
            self.get_current_scope()
        );
        self.contents.push(content_with_scope);
    }

    pub fn add_scope(&mut self, scope: &'ctx str) {
        self.scope.push_back(scope);
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

    fn get_current_scope(&self) -> &'ctx str {
        match self.scope.back() {
            Some(scope_id) => scope_id,
            None => "__global"
        }
    }
}
