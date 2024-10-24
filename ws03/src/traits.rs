
struct Person {
    name: String,
    age: i32
}

trait Living {
    fn talk(&self);
}

impl Person {
    fn print(&self) {
        println!("person {} has age {}", self.name, self.age)
    }
}

impl Living for Person {
    fn talk(&self) {
        print!("hi!");
    }
}

struct Dog {
    age: i32
}

fn use_living(living_thing: &impl Living) {
    living_thing.talk();
}

impl Living for Dog {
    fn talk(&self) {
        print!("BARK!")
    }
}


fn example_main() {
    let p = Person {name: "hi".to_string(), age: 12};
    p.print();
    p.print();
    let dog = Dog {age: 10};
    use_living(&p);
    use_living(&p);
    use_living(&dog);
}