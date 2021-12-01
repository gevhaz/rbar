# rbar

`rbar` is a simple program for managing blocks (modules) in the dwm statusbar.
It includes the following features:

- Support for different update intervals for different blocks
- Support for defining a block as a Rust function, of the signature `fn() ->
  String`. This is useful for performance critical blocks, or if you just want
  to write your blocks in Rust!
  Tip: if you want to use external scripts, just use `std::process::Command`.
