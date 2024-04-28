# Simple Hash Check

This is a simple command-line tool for checking file hashes.

## Features

- Supports MD5, SHA1, SHA256, and CRC32 hash algorithms.
- Calculates and compares hash values of files.
- Accepts both absolute and relative file paths.

## Usage
```bash
$ shc
Usage: shc <filename> [<checksum1> <checksum2> ...]
```
- `<filename>`: The name of the file to check.
- `<checksum1> <checksum2> ...`: Optional checksum values to compare against.

## Examples

- Check the hash values of a file:
```bash
$ shc shc.exe
Checking file: shc.exe
MD5     : 7c48f13796c83b0b49c77aeee8f53b37
SHA1    : 5c6f48a5f653d824b3e04e85a6c554f00f56bf47
SHA256  : c8e97bff2c0e563d9deb2ae2568aa6e2e652c16d8f14deed3f1fc3778439c4d0
CRC32   : 48826480
```

```bash
$ shc shc.exe 7C48F13796C83B0B49C77AEEE8F53B37
Checking file: shc.exe
MD5     : 7c48f13796c83b0b49c77aeee8f53b37 # This line is green
SHA1    : 5c6f48a5f653d824b3e04e85a6c554f00f56bf47
SHA256  : c8e97bff2c0e563d9deb2ae2568aa6e2e652c16d8f14deed3f1fc3778439c4d0
CRC32   : 48826480
```

## License

This project is licensed under the [MIT License](https://github.com/VictorModi/shc/blob/master/LICENSE). Feel free to use it for testing purposes or any other purpose you see fit! :)
