use t3_clone::{
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber(
        "t3_clone".into(),
        "info,t3_clone=info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read config");

    Ok(())
}
