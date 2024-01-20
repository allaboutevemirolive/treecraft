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
$ tc

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

```
Usage: tc [OPTIONS]

Options:
  -d, --default         Print default layout. Etc. GNU tree's layout
  -s, --case-sensitive  Sort list in case-sensitive
  -f, --file            Printout output in a text file
  -h, --help            Print help
```


# Recommended Usage

### 1. Compile with `cargo build --release`

Compile TreeCraft with the following command, which generates highly efficient code tailored to your platform:

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
   chmod +x tc
   ```

4. Generate an ASCII tree view in the terminal by running:

   ```bash
   ./tc "filepath"
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

## Contributing

See [CONTRIBUTING.md](https://github.com/allaboutevemirolive/treecraft/blob/main/CONTRIBUTING.md).

## Licensing

TreeCraft is released under the [MIT License](https://github.com/allaboutevemirolive/treecraft/blob/main/LICENSE.md). You are free to use, modify, and distribute this software in accordance with the terms of the license.