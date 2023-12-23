# TreeCraft

> UPDATE: TREECRAFT CURRENTLY IN FREEZE SHIPPING AND WILL BE GOING RAPID DEVELOPMENT AND BREAKING CHANGES.

TreeCraft is a pure Rust command-line utility that allows user to see directory structures in ASCII format on terminal or save them to a text file. This project is meant to be a `drop-in replacement` for [tree](https://github.com/Old-Man-Programmer/tree.git) (see the link) that is written in the C language. This project aims to be non-bloated and more functional than the existing [tree](https://github.com/Old-Man-Programmer/tree.git).


What should you expect?

- A `drop-in replacement` for [tree](https://github.com/Old-Man-Programmer/tree.git).
- Fast and efficient for most platforms.
- Respect `.gitignore` and handle hidden files gracefully. (plan)
- Minimalist and more functional than the existing [tree](https://github.com/Old-Man-Programmer/tree.git).


What shouldn't you expect?

- You shouldn't expect to see any other existing [GNU utilities's related](https://github.com/coreutils/coreutils.git), as they already fit for their purpose and didn't fit for this project.


Example output:

```
nemesis@nemesis:~/Documents/Github/Focus/util/treecraft/target/release$ trie

 release
    .
    ├── build
    ├── deps
    │   ├── treecraft-52fb4a3e910503fe
    │   └── treecraft-52fb4a3e910503fe.d
    ├── examples
    ├── incremental
    └── treecraft.d


 Insights:
    .
    ├── Processing Time      : 0.00037471 seconds
    ├── Visible Dirs         : 4
    ├── Visible Files        : 3
    ├── *Hidden Dirs/Files   : 2
    ├── Total Items(excl.*)  : 7
    └── Total Size           : 0.00 GB (4,809,968 bytes)
```

## Command Line Options

TreeCraft offers several command line options:

- `-out` : Output the tree view to a text file.
- `-ci`  : Sort filenames with case insensitivity or lowercase.
- `-cs`  : Sort filenames.
- `-no`  : Do not sort.
- `-xt`  : Sort based on file's extension.
- `-help`: Display usage information and exit.


# Recommended Usage

### 1. Compile with `cargo build --release` (Recommended)

To ensure optimal performance, compile TreeCraft with the following command, which generates highly efficient code tailored to your platform:

```bash
cargo build --release
```

### 2. Usage from Source Code

1. Clone this repository or download the source code.

2. Navigate to the `src` folder.

3. Initialize the project with Cargo by running:

   ```bash
   cargo init
   ```

4. Generate an ASCII tree view with the following command:

   ```bash
   cargo run "filepath" "flag"
   ```

   For example:

   ```bash
   cargo run /home/nemesis/Documents/Github/Focus/lang -tf
   ```

### 3. Usage of the `treecraft` Binary

Download the treecraft binary and follow these steps:

1. Place the `treecraft` binary in the directory where you want to generate a tree view.

2. Open your terminal and navigate to the folder containing the `treecraft` binary:

   ```bash
   cd "filepath"
   ```

3. Make the binary executable with the following command:

   ```bash
   chmod +x treecraft
   ```

4. Generate an ASCII tree view in the terminal by running:

   ```bash
   ./treecraft "filepath"
   ```

   Replace `"filepath"` with the path to the directory you want to visualize.


# Create a Linux alias (tested in Debian).

1. Open your `.bashrc` file for editing by running this command:

   ```bash
   nano ~/.bashrc
   ```

2. Go to the end of the file and add the following line, replacing the placeholders with your actual information:

   ```bash
   alias tr="/path/to/your/project/target/release/your_binary_name"
   ```

   - Replace "/path/to/your/project" with the actual path to your project directory.
   - Replace "your_binary_name" with the name of your Rust binary.

3. Save your changes and exit the text editor (in nano, press Ctrl+O to save and Ctrl+X to exit).

4. Apply the changes to your current terminal session by running:

   ```bash
   source ~/.bashrc
   ```

Now, when you open a new terminal session, you can use the "trie" alias to run your Rust binary.

# Examples of TreeCraft Usage

Here are two examples of how to use TreeCraft to visualize directory structures:

### Example 1: Generate a Text File

To create an ASCII tree-view and save it to a text file, utilize the `-tf` flag like this:

```bash
./treecraft /home/nemesis/Documents/Github/Focus/lang -tf
```

This will generate an `output.txt` file in the same directory as your specified target folder.

### Example 2: Terminal Tree-View

For an ASCII tree-view directly on the terminal, use this command:

```bash
./treecraft /home/nemesis/Documents/Github/Focus/util/treecraft/target/release
```

TreeCraft will then display the tree-view on your terminal screen, allowing you to conveniently explore your directory structure.

## Contributing

See [CONTRIBUTING.md](https://github.com/allaboutevemirolive/treecraft/blob/main/CONTRIBUTING.md).

## Licensing

TreeCraft is released under the [MIT License](https://github.com/allaboutevemirolive/treecraft/blob/main/LICENSE.md). You are free to use, modify, and distribute this software in accordance with the terms of the license.