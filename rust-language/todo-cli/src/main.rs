use std::collections::HashMap; // 引入 HashMap 哈希表

struct Todo {
    map: HashMap<String, bool>, // 当前任务的活动状态
}

impl Todo {
    // 插入任务
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }

        std::fs::write("db.txt", content)
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    println!("{} {}", action, item);

    let mut todo = Todo {
        map: HashMap::new(),
    };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
}
