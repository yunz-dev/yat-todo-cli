struct TodoItem {
    importance: u8,
    size: u8,
    title: String,
    content: String,
    date: i32, // TODO: change to actual date time with chrono package later
}

impl TodoItem {
    //TODO: Update with chrono package 
    fn new(importance: u8, size: u8, title: &str, content: &str, date: i32) -> TodoItem {
        TodoItem {
            importance,
            size,
            title: title.to_string(),
            content: content.to_string(),
            date,
        }
    }

    //for testing purposes
    fn list_details(&self) {
        println!("DETAILS:");
        println!("Importance: {}", self.importance);
        println!("Size: {}", self.size);
        println!("Title: {}", self.title);
        println!("Content: {}", self.content);
        println!("Date: {}", self.date);
    }
}


fn main() {
    let test1 = TodoItem::new(1,2,"title","Content",123123);
    test1.list_details();
}
