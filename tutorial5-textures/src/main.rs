pub fn main() {
    env_logger::init();
    let (event_loop, window) = tutorial5::prepare_window();
    pollster::block_on(tutorial5::run(event_loop, window));
}
