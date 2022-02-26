# notes.md

# references/cool links

## fractals
https://soma-arc.net/projects

## code
https://github.com/irh/rust-hot-reloading
https://old.reddit.com/r/rust/comments/hbvv46/oxibrot_an_interactive_mandelbrot_fractal_renderer/
  https://gitlab.com/Gutawer/oxibrot/

https://github.com/anirudhb/reloady
  https://gist.github.com/xieyubo/6820491646f9d01980a7eadb16564ddf
https://docs.rs/live-reload/latest/live_reload/
https://docs.rs/dynamic_reload/latest/dynamic_reload/
https://nullderef.com/blog/plugin-abi-stable/

https://docs.rs/pixels/latest/pixels/
  https://github.com/parasyte/pixels/tree/main/examples/minimal-sdl2
https://docs.rs/minifb/latest/minifb/

https://docs.rs/imgui/0.8.0/imgui/
  https://docs.rs/imgui-ext/0.3.0/imgui_ext/

https://docs.rs/polars/latest/polars/


https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/#a-change-to-the-vertices
https://github.com/gfx-rs/wgpu
https://github.com/gpuweb/gpuweb
  https://gpuweb.github.io/gpuweb/
  https://gpuweb.github.io/gpuweb/wgsl/#integer-types



# TODO
- [x] use rayon
- [x] split this cargo project into separate crates for host app, fractal worker, colorizer, and shared code
- [x] use image FFI view thingy to set pixels in buffer
- [x] implement UI features
  - [x] window resize
  - [x] zoom
  - [x] resize canvas
- [x] switch to minifb to see if that's faster
  * it's great but it doesn't look like there's any easy way to add a gui...
- [x] improve data structure used
  - [x] maybe polars? no, it's too slow
  - [x] come on, at least use a 2d grid lol
- [x] make a stable abi
- [x] load fractal impl dynamically
- [x] separate fractal and colorizer functions
- [x] use gpu shader for canvas offset+zoom
- [x] use gpu shader to map prev rendered buffer to temp data in new position
- [x] rewrite the fractal worker thread to not use a grid buf, and only update the pixel buffer as needed
- [x] remove the old mandelbrot func



- [ ] save/load config from json file
- [ ] be able to reload the dynamic library
  - [ ] use https://docs.rs/libloading/latest/libloading/ for loading dynamic lib?




# compile time

```bash
cargo bloat -n 10
cargo bloat --crates -n 10
cargo bloat --release -n 10
cargo bloat --release --crates -n 10
```

https://matklad.github.io/2021/09/04/fast-rust-builds.html

https://github.com/dtolnay/cargo-llvm-lines
```bash
cd gui/
cargo llvm-lines --release --bin rust-mandelbrot-gui | head -n 12
cargo llvm-lines --bin rust-mandelbrot-gui | head -n 12
```

