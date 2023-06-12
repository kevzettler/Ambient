use std::path::Path;

use anyhow::Context;

pub async fn migrate_pipeline(path: &Path) -> anyhow::Result<()> {
    let s = tokio::fs::read_to_string(path)
        .await
        .context("Error reading json pipeline file")?;

    let de = &mut serde_json::de::Deserializer::from_str(&s);

    let value: serde_json::Value = serde_path_to_error::deserialize(de)
        .with_context(|| format!("Error deserializing json pipeline file {:?}", path))?;

    tracing::info!(?value, "Deserialized json pipeline file");

    let toml = toml::to_string_pretty(&value).context("Error serializing json to toml")?;

    Ok(())
}
