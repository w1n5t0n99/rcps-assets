use rcps_assets::configuration::get_configuration;
use rcps_assets::telemetry::{get_subscriber, init_subscriber};
use rcps_assets::startup::Application;
use tracing::subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("rcps-assets".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    //println!("{:?}", configuration);

    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());

    Ok(())
}
