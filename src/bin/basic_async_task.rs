use std::time::Duration;
use tokio::time::sleep;

//
//
//
async fn run_some_tasks() {
    let mut task_list: Vec<tokio::task::JoinHandle<String>> = Vec::new();

    for index in 1..10 {
        task_list.push(tokio::task::spawn(async move {
            println!("Task {} is running......", index);
            sleep(Duration::from_secs(1)).await;
            format!("Task {} is done.", index)
        }))
    }

    for temp_task in task_list {
        //
        // After calling each `task::JoinHandle.await`, all of the spawned
        // tasks will run concurrently in tokio runtime-created threads.
        //
        // But the `.await` result returns the value in order!!!
        //
        let temp_result = temp_task.await.unwrap();
        println!("{}", temp_result);
    }
}

//
//
//
async fn reusable_async_fn(index: usize) -> String {
    println!("Task {} is running......", index);
    sleep(Duration::from_secs(1)).await;
    format!("Task {} is done.", index)
}

//
//
//
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = run_some_tasks().await;

    let mut task_list_2 = Vec::new();
    for index in 1..10 {
        task_list_2.push(tokio::task::spawn(reusable_async_fn(index)))
    }

    for temp_task in task_list_2 {
        //
        // After calling each `task::JoinHandle.await`, all of the spawned
        // tasks will run concurrently in tokio runtime-created threads.
        //
        // But the `.await` result returns the value in order!!!
        //
        let temp_result = temp_task.await.unwrap();
        println!("{}", temp_result);
    }

    Ok(())
}
