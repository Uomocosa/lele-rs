#[derive(Debug, Clone)]
struct Person {
    name: String,
}

impl From<&str> for Person {
    fn from(name_slice: &str) -> Self {
        Person {
            name: name_slice.to_string(),
        }
    }
}

// --- The Solution ---
// This function accepts any type `T`...
// ...as long as `T` implements `Into<Person>`.
fn greet_flexible<T>(source: T)
where
    T: Into<Person>,
{
    // We call .into() *inside* the function.
    // This is the "automatic" part for the caller.
    let person: Person = source.into();
    println!("Hello, {}!", person.name);
}

fn main() {
    // We can call it with a &str!
    // Why? Because we implemented `From<&str> for Person`,
    // Rust automatically gave `&str` an `Into<Person>` impl.
    greet_flexible("Alice");

    // We can still call it with a Person!
    // Why? Rust provides a trivial `Into<Person> for Person`.
    let bob = Person::from("Bob");
    greet_flexible(bob);
}