use std::collections::LinkedList;

pub struct FileBuffer {
    data: LinkedList<i32>,
}

impl FileBuffer {
    // Initialize the file buffer with an empty linked list
    pub fn new() -> Self {
        FileBuffer {
            data: LinkedList::new(),
        }
    }

    // Push an element to the linked list
    pub fn push(&mut self, element: i32) {
        self.data.push_back(element);
    }

    // Pop an element from the linked list
    pub fn pop(&mut self) -> Option<i32> {
        self.data.pop_back()
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
//         None => println!("The linked list is empty."),
//     }

//     // Access and print the remaining linked list through the buffer instance
//     println!("{:?}", buffer.data);
// }
