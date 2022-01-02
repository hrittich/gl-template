# Template: Rust + OpenGL + SDL2

[This template](src/main.rs) should get you started using OpenGL in Rust
quickly. It is a minimal template, which just depends on [sdl2] for window
management and initialization and on [gl_generator] for providing bindings to
the [OpenGL API].

To use the template it is recommended to use [cargo-generate], which is only
needed to setup your project and can be deleted afterwards. Using the template
works by executing the following commands:

    cargo install cargo-generate
    cargo generate --name my-awesome-project --git https://github.com/hrittich/gl-template

To run your program execute the following commands:

    cd my-awesome-project
    cargo run

Now, you can move on to [learning OpenGL]. Have fun!

[learning OpenGL]: https://learnopengl.com/
[sdl2]: https://crates.io/crates/sdl2
[gl_generator]: https://crates.io/crates/gl_generator
[OpenGL API]: https://www.khronos.org/registry/OpenGL-Refpages/
[cargo-generate]: https://crates.io/crates/cargo-generate
