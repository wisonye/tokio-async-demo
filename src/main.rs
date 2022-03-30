use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::sleep;
// use std::sync::mpsc;

//
// Async function returns `Future<T>`
//
async fn send_request() -> String {
    sleep(Duration::from_secs(2)).await;

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
        sleep(Duration::from_secs(2)).await;

        "Request 2 done, here is the data: ABC".to_string()
    }
}

//
//
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(">>> start.");

    //
    // Asyn block returns Future, return type is `impl Future<Output = String>`
    //
    let my_future = async {
        println!("My Future is running...");
        println!("My Future is Done.");

        "Result from 'My Future'.".to_string()
    };

    //
    // But if you have to write return type with `impl Future<Output = String>`,
    // it doesn't work, you need to declare like this!!!
    //
    let my_future_2: Pin<Box<dyn Future<Output = String>>> = Box::pin(async {
        println!("My Future_2 is running...");
        println!("My Future_2 is Done.");

        "Result from 'My Future'_2.".to_string()
    });

    println!("my_future: {}", &my_future.await);
    println!("my_future_2: {}", &my_future_2.await);

    // let will_take_some_times = tokio::task::spawn(async move {
    //     sleep(Duration::from_secs(3)).await;
    //     "Async task done.".to_string()
    // }).await.unwrap();

    // println!("will_take_some_times: {:#?}", &will_take_some_times);

    // let data = send_request().await;

    // println!(">>> before done.");

    // println!("data: {:#?}", &data);

    // println!(">>> Done.");

    Ok(())
}
