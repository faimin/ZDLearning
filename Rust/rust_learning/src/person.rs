
pub trait Protocol {
    fn like_what(&self) -> &str;

    fn hate_what(&self) -> &str { 
        "宫斗剧"
    }
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: i32,
}

impl Person {
    pub fn say(&self, msg: &str) {
        println!("{} say {}", self.name, msg);
    }
}

impl Protocol for Person {
    fn like_what(&self) -> &str {
        "热血动漫"
    }

    fn hate_what(&self) -> &str {
        "后宫"
    }
}

