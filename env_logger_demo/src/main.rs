#[macro_use]
extern crate log;
extern crate chrono;
use env_logger::Env;
use std::io::Write;

fn main() {
    use chrono::Local;
    // Initialize logging.
    env_logger::Builder::from_env(
        Env::default()
           .default_filter_or("env_logger_demo=info")
           .default_write_style_or("never"),
    )
    .format(|buf, record| {
        writeln!(
            buf,
            "[{} {}] [{}:{}] - {}",
            Local::now().format("%Y-%m-%d %H:%M:%S%z"),
            record.level(),
            record.module_path().unwrap_or("<unnamed>"),
            record.line().unwrap_or(0),
            record.args()
        )
    })
    .init();
    info!("call_server 服务开始运行...");
    println!("Hello, world!");
}
