pub fn main() {
    env_logger::init();
    let (event_loop, window) = tutorial8::prepare_window();
    pollster::block_on(tutorial8::run(event_loop, window));
}
