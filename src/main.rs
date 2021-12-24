use lambda_extension::{extension_fn, Error, NextEvent};

async fn log_extension(event: NextEvent) -> Result<(), Error> {
    match event {
        NextEvent::Shutdown(event) => {
            println!("Shutdown {:?}", event);
        }
        NextEvent::Invoke(event) => {
            println!("Invoke {:?}", event);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = extension_fn(log_extension);
    lambda_extension::run(func).await
}
