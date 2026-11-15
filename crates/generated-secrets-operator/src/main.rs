// SPDX-License-Identifier: Apache-2.0

//! `GeneratedSecret` operator entry point.

use clap::Parser;
use generated_secrets_operator::config::Config;
use generated_secrets_operator::logging::init_logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();
    config.validate()?;
    init_logging(&config.log_level, config.log_format)?;
    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        ?config,
        "generated-secrets-operator starting"
    );
    let client = kube::Client::try_default()
        .await
        .map_err(|e| anyhow::anyhow!("cannot build the Kubernetes client: {e}"))?;
    generated_secrets_operator::run(
        config,
        client,
        generated_secrets_operator::shutdown_signal(),
    )
    .await
}
