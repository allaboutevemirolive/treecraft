# TreeCraft v0.2.4 (23 October 2023)

TreeCraft is a command-line utility written in pure Rust that helps you visualize directory structures in ASCII format on your terminal or save them to a text file.

<div>
  <img src="https://github.com/allaboutevemirolive/treecraft/blob/v14/icon/treecraft.png" />
</div>

## Command Line Options

TreeCraft offers several command line options:

- `-tf`: Output the tree view to a text file.
- `-ci`: Sort filenames with case insensitivity or lowercase.
- `-cs`: Sort filenames.
- `-no`: Do not sort.
- `-xt`: Sort based on file's extension.
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

Download the [treecraft binary](https://github.com/allaboutevemirolive/treecraft/releases/tag/treecraft) and follow these steps:

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


# Set it as `alias` in Linux (Tested in Debian)

Here's how you can add the alias for your Rust binary at the end of your `.bashrc` file:

1. Open your `.bashrc` file for editing:

```bash
nano ~/.bashrc
```

2. Scroll to the end of the file and add the following line:

```bash
alias trie="/path/to/your/project/target/release/your_binary_name"
```

Replace "/path/to/your/project" with the actual path to your project directory and "your_binary_name" with the name of your Rust binary.

3. Save and exit the text editor (for nano, press Ctrl+O, then Enter to save, and Ctrl+X to exit).

4. After saving the changes, apply the changes to your current terminal session by running:

```bash
source ~/.bashrc
```

Now, when you open a new terminal session, you should be able to use the "trie" alias to execute your Rust binary.

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
./treecraft /home/nemesis/Documents/Github/Focus/lang
```

TreeCraft will then display the tree-view on your terminal screen, allowing you to conveniently explore your directory structure.

Example Output:
```
nemesis@nemesis:~/Documents/Github/Focus/util/treecraft/target/release$ ./treecraft

Current Directory: /home/nemesis/Documents/Github/Focus/util/treecraft/target/release
Target Directory : /home/nemesis/Documents/Github/Focus/util/treecraft/target/release

Target Directory Structure:

release
-------
├── .cargo-lock
├── .fingerprint
│   └── treecraft-52fb4a3e910503fe
│       ├── bin-treecraft
│       ├── bin-treecraft.json
│       ├── dep-bin-treecraft
│       └── invoked.timestamp
├── Output.txt
├── build
├── deps
│   ├── treecraft-52fb4a3e910503fe
│   └── treecraft-52fb4a3e910503fe.d
├── examples
├── incremental
├── treecraft
└── treecraft.d

Statistics:

Processing Time   : 0.000660894 seconds
Total Directories : 6
Total Files       : 10
Total Items       : 16
Total Size        : 0.01 GB (9,571,573 bytes)
```


## Licensing

TreeCraft is released under the [MIT License](LICENSE). You are free to use, modify, and distribute this software in accordance with the terms of the license.