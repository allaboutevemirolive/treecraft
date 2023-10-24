


## 2023-10-23

1. If use default path ".", check for the literal syntax for full path

2. Distinct between output on terminal or in file

3. Distinct between "Current Directory" and "Target Directory"

```rs
write!(
    output_handler,
    "\nCurrent Directory: {}{}{}\n\n",
    "\x1b[1;33m",
    flags.dirname.to_string_lossy(),
    "\x1b[0m"
)?;
```