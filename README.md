# Custom Shell

A simple POSIX-compliant shell implemented in Rust.

## Description

This project is a custom shell that provides a command-line interface for executing commands and managing processes. It supports basic shell functionalities such as running external programs, handling builtin commands, and parsing command arguments.

The shell is implemented in Rust, following POSIX standards, and aims to provide a minimal yet functional shell experience.

## Features

- Execute external programs with arguments
- Support for builtin commands:
  - `exit`: Exit the shell with an optional exit code
  - `echo`: Print the specified arguments to the console
  - `type`: Display information about the specified command
- Dynamic determination of builtin commands and external programs
- Error handling and reporting for command execution failures
- POSIX-compliant argument parsing and command execution

## Getting Started

### Prerequisites

- Rust programming language (version 1.x.x)

### Installation

1. Clone the repository:

   ```shell
   git clone https://github.com/Delvoid/SeaShell
   ```

2. Change to the project directory:

   ```shell
   cd SeaShell
   ```

3. Build the project:
   ```shell
   cargo build --release
   ```

### Usage

1. Run the shell:

   ```shell
   ./target/release/SeaShell
   ```

2. Enter commands at the shell prompt (`$`).

3. To exit the shell, use the `exit` command or press `Ctrl+D`.

## Examples

```shell
$ echo Hello, World!
Hello, World!

$ type echo
echo is a shell builtin

$ type ls
ls is /bin/ls

$ ls -l /usr/bin
total 14192
-rwxr-xr-x 1 root root   31416 Feb 17  2022 '['
-rwxr-xr-x 1 root root   59736 Jan 27  2022 'a2p'
-rwxr-xr-x 1 root root   14328 Mar 27  2022 'acpi'
...

$ cd /usr
$ pwd
/usr

$ cd ./local/bin
$ pwd
/usr/local/bin

$ cd ../../
$ pwd
/usr

$ exit
```
