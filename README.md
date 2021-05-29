# npm-dep-check-rust

Helps you find unused dependencies in Nodejs applications. 

:warning: Might not work well with **webpack** based applications. 

## Installation 

Goto [release](https://github.com/saiumesh535/npm-dep-chek-rust/releases) and download latest version from assets, or build steps:

- need rust, see <https://rustup.rs/>
- `git clone https://github.com/saiumesh535/npm-dep-chek-rust.git`
- `cd npm-dep-chek-rust`
- `cargo install --path .`

## Usage

```cmd
npm-dep-check-rust --help
```

```cmd
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --package <package>    path to package.json
    -s, --src <source>         path to source directory
```

## Sample command

```cmd
./npm-dep-check-rust --src=./<path-to-source> --package=./<path-to-package.json>
```

Sample output:

```cmd
* "shortid"
Executed in: 12.322209ms
```
