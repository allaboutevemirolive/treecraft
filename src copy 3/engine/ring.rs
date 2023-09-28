pub struct RingBuffer {
    data: Vec<Option<i32>>,
    capacity: usize,
    size: usize,
    front: usize,
    rear: usize,
}

impl RingBuffer {
    // Initialize the ring buffer with a given capacity
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            data: vec![None; capacity],
            capacity,
            size: 0,
            front: 0,
            rear: 0,
        }
    }

    // Push an element to the ring buffer
    pub fn push(&mut self, element: i32) {
        if self.size == self.capacity {
            // If the buffer is full, remove the oldest element (LIFO)
            self.front = (self.front + 1) % self.capacity;
            self.size -= 1;
        }

        let insert_index = (self.front + self.size) % self.capacity;
        self.data[insert_index] = Some(element);
        self.size += 1;
    }

    // Pop an element from the ring buffer
    pub fn pop_lifo(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let index = (self.front + self.size - 1) % self.capacity;
            let element = self.data[index].take();
            self.size -= 1;
            element
        }
    }

    // Check if the ring buffer is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

// fn main() {
//     let mut buffer = RingBuffer::new(5); // Create a ring buffer with a capacity of 3
//     buffer.push(42);
//     buffer.push(65);
//     buffer.push(78);
//     buffer.push(5432);
//     buffer.push(989);
//     let popped_value = buffer.pop_lifo();

//     match popped_value {
//         Some(value) => println!("Popped: {}", value),
//         None => println!("The ring buffer is empty."),
//     }

//     // Access and print the remaining elements in the ring buffer
//     for i in 0..buffer.size {
//         let index = (buffer.front + i) % buffer.capacity;
//         if let Some(value) = buffer.data[index] {
//             println!("Remaining Element {}: {}", i + 1, value);
//         }
//     }
// }
