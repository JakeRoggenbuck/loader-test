use hyper_minimal_loaders::{Count, Counter};
use std::{thread, time};

fn main() {
    let mut counter = Count::default();

    loop {
        counter.show();
        counter.tick();

        if counter.finished() {
            break;
        }

        thread::sleep(time::Duration::from_millis(100));
    }
}
