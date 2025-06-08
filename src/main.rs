use t3_clone::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "t3_clone".into(),
        "info,t3_clone=info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    Ok(())
}
