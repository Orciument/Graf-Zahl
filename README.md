# Graf-Zahl

A CLI Tool in Rust for Calculating the Lines of Code in a Codebase
---------------------
![239758977-0da30056-8472-4e42-a352-91fbf3290fe1](https://github.com/Orciument/Graf-Zahl/assets/67759477/0486fcb2-a03f-45d5-8659-d2680cb7a175)
![239758948-e5c25015-333e-4e49-9f1f-0935acdf4b7b](https://github.com/Orciument/Graf-Zahl/assets/67759477/7e721faf-283c-4c24-b5da-0e4f11a81dff)
---------------------

Graf-Zahl allows you to count the lines of code, or text in a specific directory. <br>
There are several count modes to choose from: 
- `Line` counts the amount of lines
- `Word` counts the amount of words
- `Char` counts the amount of chars
- `LOC` counts the amount of lines, but categorized into code, comments, and empty lines
- `Language` counts the amount of lines for each language or file type

For each of this mode you have the option to show a summary of the results, or show the results for the entire directory tree.

More information can be found by using the ``--help`` command!

# Installing

You can either install graf-zahl via the executables in the release assets. Or build the executable yourself on your
system. <br>
For that you need cargo and the rust Toolchain for your system.
You can build the application with the ``cargo build --release`` command to get a standalone executable, you can find
this in the ``/target/release/`` directory. <br>
This binary can then be placed in a folder of your choosing.

Additionally, you need to add a few config files. Where these files should be located is platform dependent:

- Windows: ``%localappdata%/graf-zahl``
- Linux: ``/etc/opt/graf-zahl``

This folder should contain two files:

- ``ignore_list.gitingore`` for which files and folders should be ignored
- ``languages.txt`` for defining details of programming languages and file types

# Configs
## ignore_list.gitignore

``ignore_list.txt`` is for blacklisting folder names that should not be counted into the final total. 
Matched are checked at each directory level, not only at the project root!

The Syntax is similar to a standard gitignore file. The file is parsed with the [ignore crate](https://crates.io/crates/ignore).

## languages.txt

``languages.txt`` is to allow the executable to understand the Syntax of the Language, namely how comments are defined.

Each language definition is a single line. All fields are seperated by a `,`, abd all Strings are enclosed with `"`. <br>
All fields are required, but you can leave the array empty, if your language doesn't have one of comment styles.

 - the first field is the file extension. Without the separating dot!
 - the second field is the Name of the Language, results with equals names are added together
 - an array of symbols starting a inline comment, inline comments are which that are automatically terminated by a line break
 - an array of symbols starting a block comment, block comments can span several lines and only end when a ending symbol is encountered
 - an array of symbols ending a block comment

Example:
```
"rs", "Rust", [ "//", "///"], ["/*"], ["*/"]
```

# Binary Targets

- `stable-x86_64-pc-windows-msvc`
- `aarch64-unknown-linux-gnu`
- `x86_64-unknown-linux-gnu`

