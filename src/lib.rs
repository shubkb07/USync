use chrono::Local;

pub fn get_status_message(name: &str) -> String {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    format!("Hello, {name}! USync is running at {now}.")
}

pub fn add_numbers(a: f64, b: f64) -> f64 {
    a + b
}

#[derive(Debug, Clone)]
pub struct ClipboardHistory {
    max_items: usize,
    items: Vec<String>,
}

impl ClipboardHistory {
    pub fn new(max_items: usize) -> Self {
        Self {
            max_items,
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, text: &str) {
        let value = text.trim();
        if value.is_empty() {
            return;
        }

        if let Some(pos) = self.items.iter().position(|x| x == value) {
            self.items.remove(pos);
        }

        self.items.insert(0, value.to_string());
        self.items.truncate(self.max_items);
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add_numbers(2.0, 3.0), 5.0);
    }

    #[test]
    fn status_contains_name() {
        let msg = get_status_message("Alice");
        assert!(msg.contains("Alice"));
        assert!(msg.contains("USync is running"));
    }

    #[test]
    fn clipboard_history_unique_recency() {
        let mut history = ClipboardHistory::new(3);
        history.add("one");
        history.add("two");
        history.add("one");
        history.add("three");
        history.add("four");
        assert_eq!(history.items(), &["four", "three", "one"]);
    }
}
