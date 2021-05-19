# npm-dep-check-rust

Helps you find unused dependencies in Nodejs applications. 

:warning: Might not work well with **webpack** based applications. 

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

### sample command

```
./npm-dep-check-rust --src=./<path-to-source> --package=./<path-to-package.json>
```

**Sample output**

```cmd
* "shortid"
Executed in: 12.322209ms
```
