use itertools::Itertools;
use miette::IntoDiagnostic;
use rattler_conda_types::ParseStrictness::Lenient;
use rattler_conda_types::{Channel, MatchSpec};
use serde::Deserialize;
use std::str::FromStr;
use std::{io::BufRead, path::Path, sync::Arc};

use crate::config::Config;

#[derive(Deserialize, Debug, Clone)]
pub struct PipEnvFile {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    channels: Vec<String>,
    dependencies: Vec<PipEnvDep>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PipEnvDep {
    Pip { pip: Vec<String> },
}