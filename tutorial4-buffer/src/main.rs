pub fn main() {
    env_logger::init();
    let (event_loop, window) = tutorial4::prepare_window();
    pollster::block_on(tutorial4::run(event_loop, window));
}
