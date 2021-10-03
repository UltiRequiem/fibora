# Fibora

[![GitMoji](https://img.shields.io/badge/Gitmoji-%F0%9F%8E%A8%20-FFDD67.svg)](https://gitmoji.dev)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
![Lines Of Code](https://img.shields.io/tokei/lines/github.com/UltiRequiem/fibora?color=blue&label=Total%20Lines)

Port of [fibonacci-deno](https://github.com/UltiRequiem/fibonacci-deno) for
[Rust](https://rust-lang.org).

> Utilities for the Fibonacci Number and Sequence.

## Usage

### CLI Tool

This project contains a cli tool, it is made to give an example of use mainly.

#### Install it

```bash
cargo install fibora
```

#### Usage

- Get the Nth Fibonacci Number:

```bash
$ fibora --number 3
2
```

- Get an Array with the first N numbers of the Fibonacci Sequence:

```bash
$ fibora --sequence 3
[ 0, 1, 1 ]
```

Or use a binary from
[releases](https://github.com/UltiRequiem/fibora/releases/latest).

### Related

- [UltiRequiem/fibonacci](https://github.com/UltiRequiem/fibonacci): This but in
  Golang
- [UltiRequiem/fibonacci-deno](https://github.com/UltiRequiem/fibonacci-deno):
  This but in Javascript using Deno

### License

This project is licensed under the [MIT License](./LICENSE.md).
