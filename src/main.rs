use trpl::{StreamExt, Stream, ReceiverStream};
use std::{pin::pin, time::Duration};

fn main() {
    trpl::run(async {
        // We get intervals w/ get_intervals() & try to use .merge() to combine them into one stream that produce any item from any source stream
        // This doesn't compile since the streams have different types b/c of the Timeout trait on mesages
        // We transform intervals to have a timeout
        let messages = get_messages().timeout(Duration::from_millis(200));
        //let intervals = get_intervals(); - This alone would error like stated above

        // .map() transforms the u32 to a String
        // .throttle() slows down the stream, we use it here to not overwhelm the merged stream, contrains the rate of the call
        // .timeout() matchs type of messages w/ Timeout trait
        // We use a 10 second timeout since that's big & we didn't want a timeout in the first place
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));

        // .merge() merges them together
        // .take(20) takes a certain amount of messages from the whole stream
        // pin!() makes it safe for .next() operations
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        // Iterate over the merged stream.
        // Each item is a Result<String, TimeoutError> due to .timeout()
        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

// We need to handle errors in both of these funtions since .send() could fail if the other side of the channel closes
// We've been using .unwrap() but we need to handle it properly
// The same is done in get_messages()
fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        // Create a count variable & create inf loop to sleep 1 ms every iteration, increment count, & send count to channel
        // Since it's all wrapped in the async task, it'll be cleaned up w/ the runtime
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            // Use an if let for syntax candy, break out of the loop
            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            
            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}