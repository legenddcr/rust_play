extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;

fn init_env_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();

    info!("env_logger initialized.");
}

fn init_pretty_env_logger() {
    pretty_env_logger::init();
    // pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Trace);

    // use chrono::Local;
    // use std::io::Write;

    // pretty_env_logger::formatted_timed_builder()
    //     .format(|buf, record| {
    //         writeln!(
    //             buf,
    //             "{} {} [{}] {}",
    //             Local::now().format("%Y-%m-%d %H:%M:%S"),
    //             record.level(),
    //             record.module_path().unwrap_or("<unnamed>"),
    //             &record.args()
    //         )
    //     })
    //     .init();

    info!("env_logger initialized.");
}

// RUST_LOG=DEBUG cargo run -p log_test
// RUST_LOG=HELLO=DEBUG cargo run -p log_test
fn main() {
    // init_env_logger();
    init_pretty_env_logger();

    trace!("a trace example");
    debug!("deboogging");
    info!(target: "HELLO", "such information");
    warn!("o_O");
    error!("boom");
}
