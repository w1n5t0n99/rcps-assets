use rcps_assets::configuration::get_configuration;
use rcps_assets::telemetry::{get_subscriber, init_subscriber};
use tracing::subscriber;

fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("rcps-assets".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    println!("{:?}", configuration);

    Ok(())
}
