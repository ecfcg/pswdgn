# pswdgn

Password Generator for Command Line

# Usage

```
USAGE:
    pswdgn.exe [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -e, --is_easy
                     Use easy to identify characters.
    -V, --version    Prints version information

OPTIONS:
    -l, --length <LENGTH>              Length of generated password string.
                                       Default length is 8.
                                       Minimum length is 8.
                                       Maximum length is 255.
    -u, --usable <USABLE CHARACTER>    The category of characters to be used for the generated password.
                                           l : Lower case alphabets.
                                           u : Upper case alphabets.
                                           n : Numbers.
                                           s : Symbols.
```

# LICENSE

pswdgn is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See the LICENSE-APACHE and LICENSE-MIT files in this repository for more information.

