
## 2023-10-23

1. If use default path ".", check for the literal syntax for full path

2. Distinct between output on terminal or in file

3. Distinct between "Current Directory" and "Target Directory"

```rs
write!(
    handler,
    "\nCurrent Directory: {}{}{}\n\n",
    "\x1b[1;33m",
    flags.dir_path.to_string_lossy(),
    "\x1b[0m"
)?;
```

## 2023-10-30

1. Applied more idiomatic rust

2. Minimalist output interface

3. Code refactoring

4. Bug fixed