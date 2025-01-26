#[macro_export]
macro_rules! sleep {
    ($millis:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($millis))
    };
}