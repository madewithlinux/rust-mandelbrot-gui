# notes.md

# references/cool links
https://github.com/irh/rust-hot-reloading
https://old.reddit.com/r/rust/comments/hbvv46/oxibrot_an_interactive_mandelbrot_fractal_renderer/
  https://gitlab.com/Gutawer/oxibrot/

https://github.com/anirudhb/reloady
  https://gist.github.com/xieyubo/6820491646f9d01980a7eadb16564ddf
https://docs.rs/live-reload/latest/live_reload/
https://docs.rs/dynamic_reload/latest/dynamic_reload/

https://docs.rs/polars/latest/polars/

# TODO
- [x] use rayon
- [x] split this cargo project into separate crates for host app, fractal worker, colorizer, and shared code
- [x] use image FFI view thingy to set pixels in buffer
- [x] implement UI features
  - [x] window resize
  - [x] zoom
  - [x] resize canvas
- [ ] improve data structure used
  - [x] maybe polars? no, it's too slow
- [ ] use https://docs.rs/cglue/latest/cglue/ and https://docs.rs/libloading/latest/libloading/ for loading dynamic lib
- [ ] use apache arrow for IPC?
- [ ] use some data frame lib?
  * https://github.com/apache/arrow-rs
  * https://docs.rs/polars/latest/polars/



