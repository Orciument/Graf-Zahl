# Graf-Zahl
A CLI Tool in Rust for Calculating the Lines of Code in a Codebase
---------------------
![239758977-0da30056-8472-4e42-a352-91fbf3290fe1](https://github.com/Orciument/Graf-Zahl/assets/67759477/0486fcb2-a03f-45d5-8659-d2680cb7a175)
![239758948-e5c25015-333e-4e49-9f1f-0935acdf4b7b](https://github.com/Orciument/Graf-Zahl/assets/67759477/7e721faf-283c-4c24-b5da-0e4f11a81dff)
---------------------


# What counts as a line of Code
A line counts as a newline (Blank) if it does not contain any characters besides white spaces
If that is not the case, but it contains a Sequence of Characters that indicate it to being a comment, then it is counted as a command.
If none of the above case are found to be true, a line is considered to be code.


---------------

# Installing 
With ``cargo build --release`` a standalone executable can be generated from the Project directory; this executable can then be found under ``/target/release/``.
This binary can then be placed in a folder of your choosing.
Additionally, you need to add two Config files in ``%localappdata%/graf-zahl``, named ``ignore_list.txt`` and ``languages.txt``


## ignore_list
``ignore_list.txt`` is for blacklisting folder names that should not be counted into the final total.

### Syntax:
The Syntax is similar to a gitignore file. The file is parsed with the [ignore crate](https://crates.io/crates/ignore). 

## languages
``languages.txt`` is to allow the executable to understand the Syntax of the Language, namely how Commands are defined.
All Languages that are encountered while counting that are not in the languages file are ignored.
## Syntax
``languages.txt`` has a separate Line for each Language. Each Lines consists of three parts, each part being Space Separated from each other, and all Values being in quotation marks (``"value"``).
The first part is the Display Name of the Language, followed by the File extension of that languages Source Code File. The Third part is a List of all Character Sequences that indicate that the Line is a Comment.
The List is initiated and terminated with opening and closing brackets ``[ ]``, and all values in the list are again space separated and in quotation marks.

## Example
```
"Rust" "rs" ["//" "///"]
"Java" "java" ["//" "//*" "*//" "  * "]
"Markdown" "md" [" "]
```
