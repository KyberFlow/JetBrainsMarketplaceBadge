mod file_writer;
mod generate;

use crate::file_writer::save_svg;
use crate::generate::GenerateInfo;
use quick_xml::de::from_str;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::env;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
struct IdeaPlugin {
    version: String,
    vendor: String,
    name: String,
    #[serde(rename = "@downloads")]
    downloads: u64,
}

#[derive(Debug, Deserialize)]
struct Category {
    #[serde(rename = "idea-plugin")]
    plugins: Vec<IdeaPlugin>,
}

#[derive(Debug, Deserialize)]
struct PluginRepository {
    category: Category,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let key = "MARKETPLACE_ID";
    let svg_output_name = "intellij_marketplace_badge.svg";
    let svg_style = "default_badge";

    let marketplace_id: u16 = match env::var(key) {
        Ok(val) => {
            tracing::debug!(value = %val, "Read environment variable");
            match val.parse::<u16>() {
                Ok(id) => id,
                Err(_) => {
                    tracing::error!(value = %val, "Environment variable {} is not a valid number", key);
                    return Ok(());
                }
            }
        }
        Err(_) => {
            tracing::error!("Environment variable {} not set", key);
            return Ok(());
        }
    };

    tracing::debug!(marketplace_id, "Using marketplace ID");

    let url = Url::parse_with_params(
        "https://plugins.jetbrains.com/plugins/list",
        &[("pluginId", marketplace_id.to_string())],
    )?;

    let resp = reqwest::get(url).await;

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            tracing::error!(error = ?e, "Failed to fetch plugin info");
            return Ok(());
        }
    };

    if !resp.status().is_success() {
        tracing::error!(status = ?resp.status(), "Request failed");
        return Ok(());
    }

    let body = match resp.text().await {
        Ok(text) => text,
        Err(e) => {
            tracing::error!(error = ?e, "Failed to read response body");
            return Ok(());
        }
    };

    tracing::trace!(body);

    let plugin_repro: PluginRepository = match from_str(&body) {
        Ok(info) => info,
        Err(e) => {
            let plugin_info: PluginRepository = match from_str(&body) {
                Ok(info) => info,
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to parse XML response");
                    return Ok(());
                }
            };
            tracing::error!(error = ?e, "Failed to parse XML response");
            return Ok(());
        }
    };

    let latest_version = plugin_repro.category.plugins.first();

    let idea_plugin = match latest_version {
        Some(plugin) => plugin,
        None => {
            tracing::error!("No Plugin found in the repository info.");
            return Ok(());
        }
    };

    let generate_info = GenerateInfo {
        version: idea_plugin.version.clone(),
        vendor: idea_plugin.vendor.clone(),
        downloads: idea_plugin.downloads.clone().to_string(),
        name: idea_plugin.name.clone(),
    };

    let renderedSvg = generate::generate(&generate_info, svg_style);

    tracing::trace!(renderedSvg);
    save_svg(renderedSvg, svg_output_name);

    Ok(())
}
