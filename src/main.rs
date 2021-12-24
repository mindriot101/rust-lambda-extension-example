use lambda_extension::{Error, NextEvent, Extension};
use std::{pin::Pin, future::{ready, Future}};

#[derive(Default)]
struct MyExtension {
    num_events: usize,
}

impl Extension for MyExtension {
    type Fut = Pin<Box<dyn Future<Output = Result<(), Error>>>>;

    fn call(&mut self, event: NextEvent) -> Self::Fut {
        match event {
            NextEvent::Shutdown(_event) => {
                println!("Shutdown handled {} events", self.num_events);
            }
            NextEvent::Invoke(event) => {
                self.num_events += 1;
                println!("Invoke {:?}", event);
            }
        }
        Box::pin(ready(Ok(())))
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_extension::run(MyExtension::default()).await
}
