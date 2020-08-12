use std::thread::sleep;
use std::time::Duration;
use paris::Logger;

fn main() {
    let mut log = Logger::new();
    log.info("Here's some info");
    log.error("Here's an error");
    log.loading("Loading?");
    sleep(Duration::from_secs(3));
    log.done().success("All done.");
}
