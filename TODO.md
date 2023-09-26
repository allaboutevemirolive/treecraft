
## stack-based approach

The stack-based approach is potentially faster and more memory-efficient than the original recursive code for several reasons:

1. **Reduced Function Call Overhead**: In the original recursive code, each recursive function call adds a new stack frame to the call stack. This incurs some overhead in terms of memory and CPU cycles. In the stack-based approach, you manage the stack explicitly, avoiding the cost of function calls and stack frame creation and destruction.

2. **Tail Call Optimization**: Rust does not perform tail call optimization (TCO) for recursive functions by default, which means that each recursive call consumes stack space. In contrast, the stack-based approach doesn't rely on recursive calls and, therefore, doesn't suffer from stack space growth.

3. **Control over Memory**: By using an explicit stack (`VecDeque`), you have more control over memory allocation and deallocation. In the original code, the recursive function calls manage the stack implicitly, potentially leading to unnecessary memory allocations and deallocations as the call stack grows and shrinks.

4. **Parallelization Potential**: The stack-based approach can be more amenable to parallelization, especially if you use multiple threads to process different parts of the directory tree concurrently. This can lead to significant performance improvements on multi-core processors.

5. **Predictable Memory Usage**: In the stack-based approach, memory usage is more predictable and does not depend on the depth of the directory structure. This can prevent stack overflow errors in cases where the directory structure is deeply nested.

6. **Customization**: The stack-based approach allows for more fine-grained control over the traversal process. You can implement custom sorting logic and other optimizations tailored to your specific requirements.

While the stack-based approach can offer these advantages, it's essential to keep in mind that the actual performance improvement may vary depending on the specific characteristics of the directory structure and the efficiency of the sorting and processing logic. As with any optimization, it's a good practice to measure the performance of both approaches with representative data to confirm the improvements.



## OUTLINE

```rs
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::{format::*, metada::*, total::*};

#[derive(Debug)]
enum MyError {
    Io(io::Error),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let directory_path = "/home/nemesis/Documents/Github/my_repo";

    let mut totals = Totals::new();
    let treestructureformatter = TreeStructureFormatter::new();

    // Measure execution time
    let start_time = Instant::now();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    read_directory_stack(
        Path::new(directory_path),
        &treestructureformatter,
        &mut handle,
        &mut totals,
    )?;

    let milli_seconds = start_time.elapsed();
    let seconds =
        milli_seconds.as_secs() as f64 + milli_seconds.subsec_nanos() as f64 / 1_000_000_000.0;

    println!();
    println!("Times Processing  : {:?}s", seconds);
    println!("Total Folders     : {}", totals.dirs);
    println!("Total Files       : {}", totals.files);
    println!("Total Size        : {} bytes", totals.size);
    println!();

    Ok(())
}

struct StackItem {
    path: PathBuf,
    depth: i32,
}

fn read_directory_stack(
    root_path: &Path,
    treestructureformatter: &TreeStructureFormatter,
    output: &mut dyn Write,
    totals: &mut Totals,
) -> Result<(), MyError> {
    let mut stack: VecDeque<StackItem> = VecDeque::new();
    stack.push_back(StackItem {
        path: root_path.to_path_buf(),
        depth: 1,
    });

    while let Some(item) = stack.pop_back() {
        let entries: Vec<_> = fs::read_dir(&item.path)?.collect();

        // Custom sorting logic here
        entries.sort_unstable_by_key(|entry| entry.as_ref().unwrap().file_name());

        for (index, entry) in entries.iter().enumerate() {
            let info = FileInfo::new(&entry.as_ref().unwrap(), item.depth)?;

            treestructureformatter.print_directory_structure(
                index < entries.len() - 1,
                item.depth - 1,
                output,
            )?;

            writeln!(output, "{}", info.name)?;

            if info.file_type.is_dir() {
                totals.dirs += 1;
                stack.push_back(StackItem {
                    path: info.path.clone(),
                    depth: item.depth + 1,
                });
            } else {
                totals.files += 1;
            }

            totals.size += info.size;
        }
    }

    Ok(())
}
```