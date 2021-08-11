// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Module containing the node configuration.

use crate::cli::NodeCliArgs;

use bee_logger::{LoggerConfig, LoggerConfigBuilder, LOGGER_STDOUT_NAME};

use serde::Deserialize;
use thiserror::Error;

use std::{fs, path::Path};

/// Default path for the node configuration file.
pub const DEFAULT_NODE_CONFIG_PATH: &str = "./config.toml";

/// Errors occurring while parsing the node configuration file.
#[derive(Debug, Error)]
pub enum Error {
    /// Reading the node configuration file failed.
    #[error("reading the node configuration file failed: {0}")]
    ConfigFileReadFailed(#[from] std::io::Error),
    /// Deserializing the node configuration file failed.
    #[error("deserializing the node configuration file failed: {0}")]
    ConfigFileDeserializationFailed(#[from] toml::de::Error),
}

/// Builder for the [`NodeConfig`].
#[derive(Default, Deserialize)]
pub struct NodeConfigBuilder {
    pub(crate) logger: Option<LoggerConfigBuilder>,
}

impl NodeConfigBuilder {
    /// Creates a [`NodeConfigBuilder`] from a config file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        match fs::read_to_string(path) {
            Ok(toml) => toml::from_str::<Self>(&toml).map_err(Error::ConfigFileDeserializationFailed),
            Err(e) => Err(Error::ConfigFileReadFailed(e)),
        }
    }

    /// Applies a [`NodeCliArgs`] to the [`NodeConfigBuilder`].
    pub fn with_cli_args(mut self, args: NodeCliArgs) -> Self {
        if let Some(log_level) = args.log_level {
            if self.logger.is_none() {
                self.logger = Some(LoggerConfigBuilder::default());
            }
            // Unwrapping is fine because the logger is set to Some if it was None.
            self.logger.as_mut().unwrap().level(LOGGER_STDOUT_NAME, log_level);
        }
        self
    }

    /// Finished the [`NodeConfigBuilder`] into a [`NodeConfig`].
    pub fn finish(self) -> NodeConfig {
        NodeConfig {
            logger: self.logger.unwrap_or_default().finish(),
        }
    }
}

/// Node configuration.
pub struct NodeConfig {
    /// Logger configuration.
    pub logger: LoggerConfig,
}

impl Clone for NodeConfig {
    fn clone(&self) -> Self {
        Self {
            logger: self.logger.clone(),
        }
    }
}