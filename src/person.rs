
pub(crate) struct Person {
    pub(crate) name: String,
    pub(crate) age: usize,
}

pub(crate) fn print_person_info(person: &Person) {
    println!("name: {}", person.name);
    println!("age: {}", person.age);
}

