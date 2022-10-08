use main_window::main_window::run;

pub mod main_window;
pub mod geometry;
pub mod game;

fn main() {
    // TODO: WASM block_on doesn't work in wasm: need to use browser's executor or
    // wasm-bindgen-futures. See https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface
    // search wasm or block_on
    pollster::block_on(run());
}
