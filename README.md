# crossgen
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Cross compilation template generator. Automated version of
[trust](https://github.com/japaric/trust).

- [Documentation][8]
- [Crates.io][2]
- [Releases][9]

## Usage
```txt
crossgen 0.1.0
Cross compilation template generator

USAGE:
    crossgen [FLAGS] [OPTIONS] [dir]

FLAGS:
    -h, --help         Prints help information
    -P, --pretty       Enable pretty printing.
    -V, --version      Prints version information
    -v, --verbosity    Pass many times for more log output

OPTIONS:
    -n, --name <name>    Project name. Defaults to target directory name

ARGS:
    <dir>    Target directory [default: .]
```

## Installation
```sh
$ cargo add crossgen
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/crossgen.svg?style=flat-square
[2]: https://crates.io/crates/crossgen
[3]: https://img.shields.io/travis/yoshuawuyts/crossgen.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/crossgen
[5]: https://img.shields.io/crates/d/crossgen.svg?style=flat-square
[6]: https://crates.io/crates/crossgen
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/crossgen
[9]: https://github.com/yoshuawuyts/crossgen/releases
