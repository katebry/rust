// defining a struct to hold a reference using the lifetime annotation
#[derive(Debug)]
struct ImportantExcept<'a> {
    part: &'a str,
}

impl<'a> ImportantExcept<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcept<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcept {
        part: first_sentence,
    };

    println!("{:?}", i);
}
