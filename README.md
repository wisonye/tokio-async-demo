# Tokio async/.await demo


### Async block or async function returns `Future<T>` trait.

    ```bash
    cargo watch -c --exec "run --bin async_function_body"
    ```

    </br>


### Async block return type

    Asyn block returns `Future<T>` trait, return type is `impl Future<Output = String>`.

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


