# Notes on Rust Language

### Variable Declaration
```rust
let x = 5; // Immutable variable x
let mut x = 5; // mutable variable x
let mut x: i32 = 5; // explicit typing
```

### Printing formats
```rust
println!("{}", int_var) // print func only takes formatted strings
println!("Struct: {:?}", my_struct);
println!("Struct (pretty):\n{:#?}", my_struct); // pretty print a struct
println!("{1} is from {0} and {1} likes to {2}.", "Canada", "Alice", "code"); // Positional Formatting
println!("Pi is roughly {pi:.precision$}", pi=3.141592, precision=2); // Fixed number of decimal places
println!("{number:>width$}", number=7, width=6); // Right-align text with spaces
println!("{number:0>width$}", number=7, width=3); // Pad numbers with zeros
println!("{name} likes {activity}.", name="Dave", activity="to swim");
```

```Rust
 #[allow(dead_code)]
fn main() {

    #[derive(Debug)]
    struct ClassRoom{
    teacher: String,
    best_student: String,
    num_students: i32,
    class_avg: f32,
    }

  #[allow(dead_code)]
  let class = ClassRoom {
    teacher: String::from("Mrs Teach"),
    best_student: String::from("Nico Bico"),
    num_students: 30,
    class_avg: 80.5
  };

  // Tuple Structs
  struct Student(i32, f32);

  // enums
  
#[allow(dead_code)]
  enum AppState {
    Loading,
    Content(String),
    Error(String)
  }
  // enums are compatible with Match statements, which are similar to Switch Case in Cpp
  // They basically ensure all variants are covered when an enum is initialized. I think?

  let state = AppState::Loading;

  match state{
    AppState::Loading => println!("Application is loading..."),
    AppState::Content(data) => println!("Content available: {}", data),
    AppState::Error(message) => println!("Error: {}", message)
}


}
```