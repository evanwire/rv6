# rv6
xv6.. but in Rust

## How to build
We have to specify a bare-metal environment so that Rust doesn't know there is an underlying OS:
```
rustup target add thumbv7em-none-eabihf
```

Now we can build, specifying this bare-metal environment as our build target.
```
cargo build --target thumbv7em-none-eabihf
```

For more info, see [this article](https://os.phil-opp.com/freestanding-rust-binary/).

## Resources
Philipp Opperman's [Writing an OS in Rust](https://os.phil-opp.com/) blog.
The original [xv6 source code](https://github.com/mit-pdos/xv6-public).
