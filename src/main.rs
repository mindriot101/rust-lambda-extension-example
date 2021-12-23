use lambda_extension::{extension_fn, Error, NextEvent};
use simple_logger::SimpleLogger;
use log::LevelFilter;
use tracing::info;

async fn log_extension(event: NextEvent) -> Result<(), Error> {
    match event {
        NextEvent::Shutdown(event) => {
            info!("Shutdown {:?}", event);
        }
        NextEvent::Invoke(event) => {
            info!("Invoke {:?}", event);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = extension_fn(log_extension);
    lambda_extension::run(func).await
}
