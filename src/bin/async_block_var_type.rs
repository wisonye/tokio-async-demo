use std::future::Future;
use std::pin::Pin;

//
//
//
async fn async_variable_type_example() {
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
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = async_variable_type_example().await;

    Ok(())
}
