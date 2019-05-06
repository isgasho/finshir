# Contributing
Since Finshir is a free (in sense of freedom) kind of software, you are always welcome to contribute! Please look through our [code of conduct](https://github.com/Gymmasssorla/finshir/blob/master/CODE_OF_CONDUCT.md) and the liberal [GPLv3 license](https://github.com/Gymmasssorla/finshir/blob/master/LICENSE), under which the product is distributed.

## Environment setup
To setup your development environment for contribution, you need to [install the Rust toolchain](https://www.rust-lang.org/tools/install) using one convenient command below:

```bash
# Setup all the rust programming language toolchain
curl https://sh.rustup.rs -sSf | sh
```

We use the [IntelliJ Rust](https://intellij-rust.github.io/) integrated development environment for comfortable development process. Just install it from the link above (or download [this plugin](https://plugins.jetbrains.com/plugin/8182-rust)) and open the cloned repository using it.

## Building and testing
As it should be in correct projects, all the building and testing procedures are performed by [Travis CI](https://travis-ci.com/Gymmasssorla/finshir). But, of course, you can do it by yourself using the following commands:

```bash
$ cargo build --verbose --all
$ cargo test --verbose --all
```

## Where to go?
 - **[Issues](https://github.com/Gymmasssorla/finshir/issues)** are meant for reporting found bugs and new functionality suggestions. Discussions are welcome too, and I will try to answer you in near future.
 
 - **[Pulls](https://github.com/Gymmasssorla/finshir/pulls)** are meant for implementing new functionality and fixing bugs. Note that other people can criticize your code, and you should answer them. Also don't forget to run `cargo fmt` before pushing any changes.
