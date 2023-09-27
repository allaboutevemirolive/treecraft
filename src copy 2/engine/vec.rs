pub struct FileBuffer {
    data: Vec<i32>,
}

impl FileBuffer {
    // Initialize the file buffer with an empty vector
    pub fn new() -> Self {
        FileBuffer {
            data: Vec::new(),
        }
    }

    // Push an element to the vector
    pub fn push(&mut self, element: i32) {
        self.data.push(element);
    }

    // Pop an element from the vector
    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop()
    }
}

// fn main() {
//     let mut buffer = FileBuffer::new();
//     buffer.push(42);
//     buffer.push(65);
//     buffer.push(78);
//     let popped_value = buffer.pop();

//     match popped_value {
//         Some(value) => println!("Popped: {}", value),
//         None => println!("The vector is empty."),
//     }

//     // Access and print the remaining vector through the buffer instance
//     println!("{:?}", buffer.data);
// }
