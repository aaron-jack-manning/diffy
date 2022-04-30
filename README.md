# Diffy

A file diff tool written in Rust.

Build the program by running `cargo build --release` which will create an executable at `target/release/diffy`.

Run this program by supplying it with two filepaths on the command line:

```
diffy file1.txt file2.txt
```

The algorithm used in this program is a modified version of Myers diff, using higher level abstractions to make for easier to read code, since the optimizations made to Myers diff often make understanding the algorithm on a high level harder. As such, the code here is provided for educational purposes, not in the interest of the absolute best diff algorithm.
