
# Nitro Generator (In Rust)
[![🔥 Continuous Integration](https://github.com/lnxcz/nitro-generator/actions/workflows/ci.yml/badge.svg?branch=dev)](https://github.com/lnxcz/nitro-generator/actions/workflows/ci.yml)
[![Release](https://github.com/lnxcz/nitro-generator/actions/workflows/release.yml/badge.svg?branch=dev)](https://github.com/lnxcz/nitro-generator/actions/workflows/release.yml)

This generator is revork of my last python one. It has a simple interface that makes it easy to use, and it is also very fast and efficient (maybe not).

## How to use

It is very easy to use. Simply run the program and follow the prompts. It will ask you for the name of the file you want to create, and then it will generate and check the codes in multiple threads.

## Contributing

If you would like to contribute, please fork the repository and make your changes in a new branch. Once you have made your changes, simply create a pull request and I will review and merge your changes.

## Releases

Every so often, I will create a release, it will include binaries and checksums for all major os (Theese binaries are build with github actions, so no need to worry)  


## Building from source

If you would like to build this from source, simply clone the repository and run the following command:

```
cargo build --release
```

This will generate a release binary in the `target/release` directory.
