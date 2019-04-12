# pwg

[![Build Status](https://travis-ci.org/dirkeinecke/pwg.svg?branch=master)](https://travis-ci.org/dirkeinecke/pwg)
[![Crate](https://img.shields.io/crates/v/pwg.svg)](https://crates.io/crates/pwg)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/pwg.svg)](https://crates.io/crates/pwg)
[![API](https://docs.rs/pwg/badge.svg)](https://docs.rs/pwg)
![License](https://img.shields.io/crates/l/pwg.svg)
[![Gitter](https://badges.gitter.im/pwg-rs/community.svg)](https://gitter.im/pwg-rs/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

With `pwg` you can easily create random passwords. You can specify how long the password should be and whether it should also contain capital letters, numbers and special characters.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
pwg = "0.1"
```

Add this to your `*.rs` file:

```rust
extern crate pwg;
```

Now you can proceed as follows:

### Example 1

```rust
/*
  A password with 20 characters is generated.
  It contains lowercase letters (default),
  uppercase letters, numbers and symbols.
*/
let password = pwg::new(20, &["uppercase", "numbers", "symbols"]);
println!("{}", password);
```

### Example 2

```rust
/* A password with 10 characters (lower case letters) is generated. */
let password = pwg::new(10, &[]);
println!("{}", password);
```

### Example 3

```rust
/*
  A password with 20 characters is generated.
  It contains lowercase letters (default) and uppercase letters.
*/
let password = pwg::new(20, &["uppercase"]);
println!("{}", password);
```

### Example 4

```rust
/*
  A password with 20 characters is generated.
  It contains lowercase letters (default) and numbers.
*/
let password = pwg::new(20, &["numbers"]);
println!("{}", password);
```

### Example 5

```rust
/*
  A password with 20 characters is generated.
  It contains lowercase letters (default) and symbols.
*/
let password = pwg::new(20, &["symbols"]);
println!("{}", password);
```

## Versions

A detailed [changelog](CHANGELOG.md) is available.

## Getting help

If you have questions or problems with `pwg`, then we are happy to respond to [GitHub issues](https://github.com/dirkeinecke/pwg/issues/new) or come chat with us on our [Gitter channel](https://gitter.im/pwg-rs/community) - if you have any questions about the project, or just want to say hi!

## License

`pwg` is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.
