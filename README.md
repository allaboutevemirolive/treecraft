# TreeCraft v0.2.1 (13 October 2023)

TreeCraft is a command-line utility written in pure Rust that helps you visualize directory structures in ASCII format on your terminal or save them to a text file.

## Command Line Options

TreeCraft offers several command line options:

- `-tf`: Output the tree view to a text file.
- `-st-fn-lc`: Sort filenames with case insensitivity or lowercase.
- `-st-fn`: Sort filenames.
- `-st-no`: Do not sort.
- `-help`: Display usage information and exit.

You can use these options to customize TreeCraft's behavior according to your preferences.

## Recommended Usage

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

5. You can now contribute to the project or modify it to suit your needs.

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
    │   │   │   └── main.zig
    │   │   ├── windows_spawn
    │   │   │   ├── build.zig
    │   │   │   ├── hello.zig
    │   │   │   └── main.zig
    │   │   └── zerolength_check
    │   │       ├── build.zig
    │   │       └── src
    │   │           └── main.zig
    │   ├── standalone.zig
    │   ├── tests.zig
    │   └── translate_c.zig
    └── tools
        ├── crc
        │   └── catalog.txt
```


## Licensing

TreeCraft is released under the [MIT License](LICENSE). You are free to use, modify, and distribute this software in accordance with the terms of the license.