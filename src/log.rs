use colored::*;

pub fn warn(message: &str) {
    println!("[{}] {}", "WARN".yellow().bold(), message)
}

pub fn error<E: std::fmt::Debug>(message: &str, err: E) {
    println!(
        "[{}] {} {:?}",
        "ERROR".white().on_red().bold(),
        message,
        err
    )
}

// Alternative to dbg!
// it will only print if ENV=debug
// macro_rules! debug {
//     ($e:expr) => {
//         if std::env::var("ENV").unwrap_or_else(|_| "debug".to_owned()) == "debug" {
//             dbg!($e);
//         }
//     };
// }
