

1.  Create Crate to handle ANSI color.

Why?

Currently, ANSI color only works with terminal, if the same output were produce on a text file, it will generate something like this,

```txt
│   ├── README.md
│   ├── SECURITY.md
│   ├── [92msrc[0m
│   │   ├── all.bash
│   │   ├── all.bat
│   │   ├── all.rc
│   │   ├── [92marchive[0m
│   │   │   ├── [92mtar[0m
│   │   │   │   ├── common.go
```