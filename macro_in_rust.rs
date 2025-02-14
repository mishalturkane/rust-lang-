macro_rules! log {
    // Match the `info` level
    ($level:ident, $message:expr) => {
        println!("[{}]: {}", stringify!($level), $message);
    };
}

fn main() {
    log!(info, "Application started");
    log!(warn, "This is a warning");
    log!(error, "An error occurred");
}
