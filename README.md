# geany-init
Create a simple [Geany](https://www.geany.org/) project in the current directory

## Options

```
Usage: geany-init [OPTIONS] <LANGUAGE>

Arguments:
  <LANGUAGE>
          The programming language of the project

          Possible values:
          - rust: specialized for the rust language
          - any:  generic

Options:
  -f, --force
          Force overwrite if a project file already exists

  -h, --help
          Print help information (use `-h` for a summary)
```

## Installation

### From sources
Follow these instructions to compile `geany-init` (requires [rust](https://www.rust-lang.org/) installed).

1\. Clone the project 
 
 ```
 https://github.com/geckoblu/geany-init.git && cd geany-init
 ```
 
2\. Build the project
 
 ```
 cargo build --release
 ```
 
3\. Once complete, the binary will be located at

```
target/release/geany-init
```

4\. Copy the binary in one of the directories listed in your $PATH
