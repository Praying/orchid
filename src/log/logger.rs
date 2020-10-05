pub use log::{error, info, warn};
pub use log4rs;
pub use log4rs::config::ConfigBuilder;
pub use log4rs::Error;

pub fn setup_logging() -> Result<(), Error> {
    match log4rs::init_file("src/log/log4rs.toml", Default::default()) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("log4rs init failed!,{}", e);
            Err(e)
        }
    }
}

#[test]
fn test_log() {
    match setup_logging() {
        Ok(()) => {
            info!("booting up");
            error!("booting up error");
            warn!("booting up warn");
        }
        Err(e) => panic!("failed to init log2rs, err: {}", e),
    }
}
