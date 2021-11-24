# rbar

`rbar` is a simple program for managing blocks (modules) in the dwm statusbar.
It includes the following features:

- Support for different update intervals for different blocks
- Support for defining a block as a Rust function, of the signature `fn() ->
  String`. This is useful for performance critical blocks, or if you just want
  to write your blocks in Rust!
- Support for defining a block as the path to a script. The script will be
  executed and the content of the standard output stream used.
