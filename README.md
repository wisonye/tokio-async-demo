# Tokio async/.await demo

The `Tokio` library is the most widely used `RUST async` runtime, any it
designed for **IO-bound** applications where each individual task spends
most of its time waiting for IO.

But `Tokio` is not good for a couple of scenarios, read the `When not
to use Tokio` section from [here](https://tokio.rs/tokio/tutorial).

</br>


### Async block or async function returns `Future<T>` trait.

```bash
cargo watch -c --exec "run --bin async_function_body"
```

</br>


### Async block return type

Async block returns `Future<T>` trait, return type is `impl Future<Output = String>`.

But if you have to write return type with `impl Future<Output = String>`,
it doesn't work, you need to declare like this!!!

```rust
let my_future_2: Pin<Box<dyn Future<Output = String>>> = Box::pin(async {
    "Result from 'My Future'_2.".to_string()
});
```

</br>

```bash
cargo watch -c --exec "run --bin async_block_var_type"
```

</br>

### Async task run concurrently

`tokio::task::spawn()` accept a `Future<T>` to spawn a `Task` (a light weight,
non-blocking unit of execution or say `green thread`).

```rust
//
// Spawn with an async block which produces a `Future<T>`.
//
// The `move` keyword is optional just in case you need to change the ownership
// for the captured variable.
//
tokio::task::spawn(async move {
    // ...
})
```

After spawning tasks and calling each `task::JoinHandle.await`, all of
the spawned tasks will run concurrently in tokio runtime-created threads.

But the `.await` result returns the value in order!!!

```bash
cargo watch -c --exec "run --bin basic_async_task"


# Task 5 is running......
# Task 4 is running......
# Task 9 is running......
# Task 8 is running......
# Task 1 is running......
# Task 3 is running......
# Task 7 is running......
# Task 2 is running......
# Task 6 is running......
# Task 1 is done.
# Task 2 is done.
# Task 3 is done.
# Task 4 is done.
# Task 5 is done.
# Task 6 is done.
# Task 7 is done.
# Task 8 is done.
# Task 9 is done.
```

</br>

Because `tokio::task::spawn()` accept a `Future<T>`, that's why you can pass an
async function and call it to produce a `Future<T>` for getting the reusable
async logic.

```rust
async fn reusable_async_fn() -> String {
    format!("")
}

tokio::task::spawn(reusable_async_fn())
```
</br>

### About `Runtime` and `features`

An `async fn` is used as we want to enter an asynchronous context. However,
asynchronous functions must be executed by a **`runtime`**. The runtime
contains the asynchronous task scheduler, provides evented I/O, timers, etc.
The `runtime` does not automatically start, so the main function needs to
start it.

The `#[tokio::main]` function is a macro. It does the following transformations:

- It transforms the `async fn main()` into a synchronous `fn main()`.

- It wraps the entire function body into an async block that produces a `Future<T>`.

- It initializes a runtime instance and executes the async main function.

</br>


```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(">>> start.");

    Ok(())
}
```

</br>

For example, the sample code above will be expanded as the following code if
you run `cargo expand --bin runtime_expand`:

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;

//
// It transforms the `async fn main()` into a synchronous `fn main()`.
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    // It wraps the entire function body into an async block that produces a `Future<T>`.
    //
    let body = async {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&[">>> start.\n"], &[]));
        Ok(())
    };

    //
    // It initializes a runtime instance and executes the async main function.
    //
    #[allow(clippy::expect_used)]
    // Builder with the multi thread scheduler selected.
    tokio::runtime::Builder::new_multi_thread()
        //
        // Enables both I/O and time drivers.
        // Doing this is a shorthand for calling `enable_io` and `enable_time`
        // individually.
        // If additional components are added to Tokio in the future,
        // `enable_all` will include these future components.
        //
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        //
        // Execute the future, blocking the current thread (main thread )
        // until completion
        //
        .block_on(body)
}
```

</br>

Pay attention that you need to configure the `tokio` with the following features
in `Cargo.toml` before you can run the above code:

```toml
# Enables the heavier, multi-threaded, work-stealing scheduler.
# Enables #[tokio::main] and #[tokio::test] macros.
# Enables tokio::time types and allows the schedulers to enable the built in timer.
tokio = { version = "1", features = [ "rt-multi-thread", "macros", "time" ] }
```

</br>


