pub fn main() {
    env_logger::init();
    let (event_loop, window) = tutorial6::prepare_window();
    pollster::block_on(tutorial6::run(event_loop, window));
}
