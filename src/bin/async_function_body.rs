use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

//
// Async function returns `Future<T>`
//
async fn send_request() -> String {
    sleep(Duration::from_secs(1)).await;

    "Request done, here is the data: ABC".to_string()
}

//
// This is the version how compiler will see when we declare an async function:
//
// Change the return type as `impl Future<Output = T>` and then wrap the entire
// function body into an `async block` which guarantees that the function body
// returns a `Future<T>`
//
fn send_request_2() -> impl Future<Output = String> {
    async {
        sleep(Duration::from_secs(1)).await;

        "Request 2 done, here is the data: ABC".to_string()
    }
}


//
//
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Future starts running ......");

    println!("'send_request' result: {}",send_request().await );
    println!("'send_request_2' result: {}",send_request_2().await );

    Ok(())
}
