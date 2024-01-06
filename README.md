# RTD 📖

RTD (i.e. RustyToDo) is a simple command-line TODO tool written in Rust.

English | [简体中文](./README_CN.md)

## Features ✨

- Multiple task lists can be created.
- User-friendly command-line interface.
- Uses JSON files for storage.

## Installation ⚙️

Ensure that you have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
$ git clone https://github.com/ikka0426/rtd.git
$ cd rtd
$ cargo build --release
```

## Usage 💻

### Selecting a todo list

```bash
./rtd use <TODOLIST>
```

### Adding a task

```bash
./rtd add <EVENT>
```

### Modifying the completion status of a task

```bash
./rtd ch [-r] <ID>
```

Use the following command to learn more about usage:

```bash
./rtd help
```

## Contribution 👥

If you would like to contribute to RTD, feel free to raise issues or initiate pull requests.

## Copyright and License 📝

RTD is released under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Author

<a href="https://github.com/ikka0426"><img alt="Static Badge" src="https://img.shields.io/badge/github-ikka0426-green"></a>
