pub fn main() {
    env_logger::init();
    let (event_loop, window) = tutorial1::prepare_window();
    pollster::block_on(tutorial1::run(event_loop, window));
}
