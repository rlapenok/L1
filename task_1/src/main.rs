trait Action {
    fn say(&self);
}

struct Person<'a> {
    name: &'a str,
}

impl<'a> Action for Person<'a> {
    fn say(&self) {
        println!("Hello,{}", self.name);
    }
}

fn main() {
    let person = Person { name: "Foo" };
    person.say();
}
