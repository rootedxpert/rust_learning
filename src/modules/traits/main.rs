trait Summarize {
    fn summarize(&self) -> String;
}

struct NewsLetter {
    email_address: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summarize for NewsLetter {
    fn summarize(&self) -> String {
        format!("{}.{}", self.content, self.email_address)
    }
}

impl Summarize for Tweet {
    fn summarize(&self) -> String {
        format!("{}.{}", self.content, self.username)
    }
}

fn send_letter() -> impl Summarize {
    NewsLetter {
        content: "test content".to_string(),
        email_address: "me@rootedxpert.dev".to_string(),
    }
}

fn traits_1() {
    let tweet1 = Tweet {
        content: "hello from this world".to_string(),
        username: "RootedXpert".to_string(),
    };
    let tweet1_summary = tweet1.summarize();
    println!("{:?}", tweet1_summary);
    let news_letter1 = NewsLetter {
        content: "hello from this world 2".to_string(),
        email_address: "me@rootedxpert.dev".to_string(),
    };
    let new_letter_summary = news_letter1.summarize();
    println!("{:?}", new_letter_summary);
}

pub fn traits() {
    println!("");
    println!("Implement Traits");
    traits_1();
    send_letter().summarize();
    println!("")
}
