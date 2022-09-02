Compile the code with `rustc main.rs` and run the resulting program
```
main.exe
> Hello, World!
```

Try breaking the program by renaming the main function to `main2` and attempt to build again.

This time `rustc main.rs` will report an error.

`rustc` can explain the error code. Run `rustc --explain E0601` to see the explanation.
