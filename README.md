# TreeCraft v0.1.0

TreeCraft is a utility written in pure Rust that allows you to display a tree view of directories in ASCII format on the terminal or in a text file.

## Usage for source code

1. Clone this repository or download the source code.

2. Navigate to the `src` folder.

3. Run the following command to initialize the project with Cargo:

   ```
   cargo init
   ```

4. Run the following command to generate ASCII tree view

   ```
   cargo run "filepath" "flag"
   ```

   Example:

   ```
   cargo run /home/nemesis/Documents/Github/Focus/lang -tf
   ```

5. You can now contribute to the project or modify it according to your needs.


## Usage for `treecraft` binary

[treecraft's binary](https://github.com/allaboutevemirolive/treecraft/releases/tag/treecraft)

To use TreeCraft, follow these simple steps:

1. Place the `treecraft` binary in the folder where you want to generate a tree view.

2. Open your terminal and navigate to the folder where you put the `treecraft` binary with command 
   ```bash
   cd "filepath"
   ```


3. Then run the following command:

   ```bash
   chmod +x treecraft
   ```

4. Run the following command to generate an ASCII tree-view on the terminal:

   ```
   ./treecraft "filepath"
   ```

   Replace `"filepath"` with the path to the target directory you want to visualize.

### Example 1: Generate a Text File

To generate an ASCII tree-view and save it to a text file, use the `-tf` flag like this:

```
./treecraft /home/nemesis/Documents/Github/Focus/lang -tf
```

This will create an `output.txt` file in the same folder as the target directory.

### Example 2: Terminal Tree-View

To generate an ASCII tree-view directly on the terminal, use the following command:

```
./treecraft /home/nemesis/Documents/Github/Focus/lang
```

TreeCraft will display the tree-view on your terminal screen, making it easy to explore your directory structure.

Output:
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

## Command Line Options

TreeCraft supports the following command line options:

- `-tf`: Print the output in a text file.

- `-st-fn-lc`: Sort filenames with case insensitivity or lowercase.

- `-st-fn`: Sort filenames.

- `-st-no`: Do not sort.

- `-help`: Print usage information and exit.

Feel free to use these options to customize the behavior of TreeCraft according to your preferences.

## Licensing

TreeCraft is released under the [MIT License](LICENSE). You are free to use, modify, and distribute this software in accordance with the terms of the license.