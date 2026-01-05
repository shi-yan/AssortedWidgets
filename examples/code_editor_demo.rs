// Code Editor Demo
//
// Demonstrates the code editor widget with:
// - Syntax highlighting (Rust)
// - Line numbers in gutter
// - Current line highlighting
// - Minimap
// - Code folding (Phase 3.5)

use assorted_widgets::Application;
use assorted_widgets::widgets::CodeEditor;

fn main() {
    Application::launch(|app| {
        app.spawn_window("Code Editor Demo", 1200.0, 800.0, |window| {
            // Create code editor with sample Rust code
            let sample_code = r#"// Sample Rust Code
fn main() {
    println!("Hello, world!");

    let numbers = vec![1, 2, 3, 4, 5];

    for num in numbers {
        if num % 2 == 0 {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    }

    let result = calculate_sum(10, 20);
    println!("Sum: {}", result);
}

fn calculate_sum(a: i32, b: i32) -> i32 {
    let sum = a + b;
    sum
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) {
        println!("Hello, I'm {} and I'm {} years old!", self.name, self.age);
    }
}

// Comments are highlighted in green
/* Multi-line comments
   are also supported */

fn test_strings() {
    let s1 = "This is a string";
    let s2 = 'c';  // character literal
    let number = 42;
    let hex = 0xFF;
    let float = 3.14;
}

// Test very long line for text wrapping - this line intentionally exceeds the typical editor width to verify that text wrapping works correctly when lines are extremely long and contain lots of text that should wrap to multiple visual lines in the editor
fn very_long_function_name_to_test_horizontal_scrolling_and_text_wrapping(parameter1: String, parameter2: i32, parameter3: Vec<String>, parameter4: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("This is a very long line that should definitely wrap or require horizontal scrolling depending on the wrap mode configuration of the code editor widget");
}
"#;

            let editor = CodeEditor::with_text(sample_code);

            // Update gutter width for the number of lines
            window.set_main_widget(editor);
        });
    });
}
