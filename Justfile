# Justfile


build-libs:
    cargo build --release --all-features -p mandelbrot_f64
    cargo build --release --all-features -p mandelbrot_f64_new
    cargo build --release --all-features -p color_luma_basic

build-gui:
    cargo build -p rust-mandelbrot-gui --release

build-all: build-libs build-gui

gui:
    clear -x
    just build-gui
    target/release/rust-mandelbrot-gui \
        --fractal-lib target/release/libmandelbrot_f64_new.so \
        --color-lib   target/release/libcolor_luma_basic.so

gui-debug:
    clear -x
    cargo build -p rust-mandelbrot-gui
    RUST_BACKTRACE=1 target/debug/rust-mandelbrot-gui \
        --fractal-lib target/release/libmandelbrot_f64_new.so \
        --color-lib   target/release/libcolor_luma_basic.so

check:
    cargo check --release


list-lib-symbols:
    nm -gD --defined-only target/release/*.so