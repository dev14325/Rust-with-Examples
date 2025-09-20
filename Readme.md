🦀 Rust Learning Journey
Welcome to my Rust programming learning repository! This contains various projects and examples that demonstrate different Rust concepts and features.
📚 Projects Overview
🔤 Capitalize
String manipulation and character processing in Rust. Learn how to work with UTF-8 strings, iterate over characters, and transform text data.
Key Concepts:

String vs &str
Character iteration
Unicode handling
String methods

📋 Enum
Exploring Rust's powerful enum system and pattern matching. Understanding how enums differ from other languages and their advanced capabilities.
Key Concepts:

Enum definitions
Pattern matching with match
Option and Result types
Enum methods and associated functions

⚠️ Error_Handling
Comprehensive error handling strategies in Rust using Result types, panic handling, and custom error types.
Key Concepts:

Result<T, E> type
Error propagation with ? operator
Custom error types
Panic vs recoverable errors

🔧 generics
Generic programming in Rust - writing flexible, reusable code that works with multiple types while maintaining type safety.
Key Concepts:

Generic functions and structs
Type parameters
Trait bounds
Lifetime parameters

🗂️ Hashmap
Working with Rust's HashMap collection for key-value storage and efficient data lookup operations.
Key Concepts:

HashMap creation and manipulation
Ownership with collections
Entry API
Performance considerations

⚙️ Options
Deep dive into Rust's Option type - handling the presence or absence of values safely without null pointer exceptions.
Key Concepts:

Option<T> enum
Some and None variants
Combinators (map, and_then, etc.)
Avoiding null pointer errors

🧵 strings
Comprehensive string handling in Rust, covering different string types and their use cases.
Key Concepts:

String vs &str vs &'static str
String slicing and indexing
UTF-8 encoding
String concatenation and formatting

🎭 traits
Understanding Rust's trait system - defining shared behavior and implementing polymorphism in a type-safe way.
Key Concepts:

Trait definitions and implementations
Default methods
Trait objects and dynamic dispatch
Trait bounds and where clauses

📦 vector
Working with Rust's Vec<T> - dynamic arrays that can grow and shrink at runtime.
Key Concepts:

Vector creation and manipulation
Ownership and borrowing with vectors
Iteration patterns
Capacity vs length

🚀 Getting Started
Each project is a standalone Rust application. To run any project:
bashcd project-name
cargo run
To run tests (if available):
bashcargo test
🛠️ Prerequisites

Rust (latest stable version)
Cargo (comes with Rust)

📖 Learning Path
If you're new to Rust, I recommend exploring the projects in this order:

strings - Understand Rust's string types
vector - Learn about dynamic arrays
Options - Master safe null handling
Error_Handling - Robust error management
Enum - Powerful pattern matching
generics - Write flexible code
traits - Implement shared behavior
Hashmap - Efficient key-value storage
Capitalize - Practical string manipulation

🤝 Contributing
Feel free to suggest improvements or additional examples! This is a learning repository, and I welcome feedback and contributions.
📚 Resources

The Rust Programming Language Book
Rust by Example
Rustlings Exercises