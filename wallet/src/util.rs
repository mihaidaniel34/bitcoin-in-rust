use anyhow::Result;
use std::panic;
use std::path::PathBuf;
use tracing::*;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use crate::core::{Config, Core, FeeConfig, FeeType, Recipient};

pub fn setup_tracing() -> Result<()> {
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "logs",
        "wallet.log",
    );
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(file_appender))
        .with(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::TRACE.into()),
        )
        .init();
    Ok(())
}

pub fn setup_panic_hook() {
    panic::set_hook(Box::new(move |panic_info| {
        let backtrace = std::backtrace::Backtrace::force_capture();
        error!("Application panicked!");
        error!("Panic info: {:?}", panic_info);
        error!("Backtrace: {:?}", backtrace);
    }))
}

pub fn generate_dummy_config(path: &PathBuf) -> Result<()> {
    let dummy_config = Config {
        my_keys: vec![],
        contacts: vec![
            Recipient {
                name: "Billy".to_string(),
                key: PathBuf::from("billy.pub.pem"),
            },
            Recipient {
                name: "Lukas".to_string(),
                key: PathBuf::from("lukas.pub.pem"),
            },
        ],
        default_node: "127.0.0.1:9000".to_string(),
        fee_config: FeeConfig {
            feetype: FeeType::Percent,
            value: 0.1,
        },
    };
    let config_str = toml::to_string(&dummy_config)?;
    std::fs::write(path, config_str)?;
    println!("Dummy config generated at: {}", path.display());
    Ok(())
}

pub fn sats_to_btc(sats: u64) -> String {
    let btc = sats as f64 / 100_000_000.0;
    format!("{} BTC", btc)
}

pub fn big_mode_btc(core: &Core) -> String {
    text_to_ascii_art::convert(sats_to_btc(core.get_balance())).unwrap()
}