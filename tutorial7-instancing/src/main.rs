pub fn main() {
    env_logger::init();
    let (event_loop, window) = tutorial7::prepare_window();
    pollster::block_on(tutorial7::run(event_loop, window));
}
