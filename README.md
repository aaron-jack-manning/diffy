# Diff

A Rust implementation of a (modified, simplified and colourised) version of Unix diff tool.

Run by supplying diff with two filepaths on the command line:

```
diff file1.txt file2.txt
```

To Do:
- [ ] Options for which characters leading characters to ignore (currently ignores whitespace)
- [ ] Optimizations (the current diff algorithm is naive and slow)
