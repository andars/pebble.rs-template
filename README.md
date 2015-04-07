##Pebble.rs Template

This is a template application demonstrating the use of
[Pebble.rs](https://github.com/andars/pebble.rs). It includes the modified
`wscript` and a build script that performs the necessary tasks to link in the
`pebble` crate. This is very hacky but it works and I haven't been able to
figure out any better solutions that actually work.

In order to build, clone [pebble.rs](https://github.com/andars/pebble.rs) to
somewhere on your computer and modify the `path` in Cargo.toml as appropriate.

Don't be fooled by the Cargo.toml, building is actually accomplished with
`./build.sh` rather than `cargo build`. Cargo is only used to build
dependencies.

*This is very much a work in progress*
