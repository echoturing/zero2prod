use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use zero_to_production::configuration::get_configuration;
use zero_to_production::startup;
use zero_to_production::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero-to-production".into(), "info".into(),
                                    std::io::stdout);
    init_subscriber(subscriber);


    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    let connection_pool = PgPoolOptions::new().connect_lazy_with(
        configuration.database.with_db());
    startup::run(listener, connection_pool)?.await
}