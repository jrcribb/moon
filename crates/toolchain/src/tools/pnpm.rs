use crate::errors::ToolchainError;
use crate::helpers::{exec_command, get_bin_version};
use crate::tool::{PackageManager, Tool};
use crate::Toolchain;
use async_trait::async_trait;
use moon_config::workspace::PnpmConfig;
use moon_logger::{color, debug, trace};
use std::env::consts;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct PnpmTool {
    bin_path: PathBuf,

    install_dir: PathBuf,

    pub config: PnpmConfig,
}

impl PnpmTool {
    pub fn new(toolchain: &Toolchain, config: &PnpmConfig) -> Result<PnpmTool, ToolchainError> {
        let install_dir = toolchain.get_node().get_install_dir().clone();
        let mut bin_path = install_dir.clone();

        if consts::OS == "windows" {
            bin_path.push("pnpm");
        } else {
            bin_path.push("bin/pnpm");
        }

        debug!(
            target: "moon:toolchain:pnpm",
            "Creating tool at {}",
            color::file_path(&bin_path)
        );

        Ok(PnpmTool {
            bin_path,
            config: config.to_owned(),
            install_dir,
        })
    }
}

#[async_trait]
impl Tool for PnpmTool {
    fn is_downloaded(&self) -> bool {
        true
    }

    async fn download(&self, _host: Option<&str>) -> Result<(), ToolchainError> {
        trace!(
            target: "moon:toolchain:pnpm",
            "No download required as it comes bundled with Node.js"
        );

        Ok(()) // This is handled by node
    }

    async fn is_installed(&self) -> Result<bool, ToolchainError> {
        if self.bin_path.exists() {
            let version = self.get_installed_version().await?;

            if version == self.config.version {
                debug!(
                    target: "moon:toolchain:pnpm",
                    "Package has already been installed and is on the correct version",
                );

                return Ok(true);
            }

            debug!(
                target: "moon:toolchain:pnpm",
                "Package is on the wrong version ({}), attempting to reinstall",
                version
            );
        }

        Ok(false)
    }

    async fn install(&self, toolchain: &Toolchain) -> Result<(), ToolchainError> {
        debug!(
            target: "moon:toolchain:pnpm",
            "Installing package with {}",
            color::shell(&format!("npm install -g pnpm@{}", self.config.version))
        );

        toolchain
            .get_npm()
            .add_global_dep("pnpm", self.config.version.as_str())
            .await?;

        Ok(())
    }

    fn get_bin_path(&self) -> &PathBuf {
        &self.bin_path
    }

    fn get_download_path(&self) -> Option<&PathBuf> {
        None
    }

    fn get_install_dir(&self) -> &PathBuf {
        &self.install_dir
    }

    async fn get_installed_version(&self) -> Result<String, ToolchainError> {
        Ok(get_bin_version(self.get_bin_path()).await?)
    }
}

#[async_trait]
impl PackageManager for PnpmTool {
    async fn install_deps(&self, toolchain: &Toolchain) -> Result<(), ToolchainError> {
        Ok(exec_command(
            self.get_bin_path(),
            vec!["install", "--frozen-lockfile"],
            &toolchain.workspace_dir,
        )
        .await?)
    }
}