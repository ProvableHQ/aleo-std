// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the aleo-std library.

// The aleo-std library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The aleo-std library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the aleo-std library. If not, see <https://www.gnu.org/licenses/>.

use dirs::home_dir;
use std::path::PathBuf;

/// The directory name for Aleo-related resources.
const ALEO_DIRECTORY: &str = ".aleo";

///
/// Returns the directory for accessing resources from Aleo storage.
///
/// The expected directory path to be returned is `~/.aleo/`. Falls back to the workspace directory
/// if the home directory doesn't exist or when in development mode.
///
pub fn aleo_dir(dev: Option<u16>) -> PathBuf {
    // Locate the home directory as the starting point.
    // If called on a non-standard OS, use the repository directory.
    let mut path = if dev.is_some() || home_dir().is_none() {
        workspace_dir()
    } else {
        home_dir().expect("home directory should be present")
    };

    // Append the Aleo directory to the path.
    path.push(ALEO_DIRECTORY);
    path
}

///
/// Returns the workspace path.
///
pub fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;

    // Returns the location of the toml file for the workspace.
    let mut path = PathBuf::from(String::from_utf8(output).unwrap().trim());
    // Pop the toml file from the path to get the workspace dir.
    path.pop();

    path
}

///
/// Returns the base path for the BFT committee files.
///
/// The expected paths are:
///
/// - Production: `~/.aleo/committee`.
/// - Development: `../.aleo/committee`.
///
/// Note: in development mode, the `.aleo` directory will be created in the cargo workspace as
/// opposed to the home directory, see [`aleo_dir`] for details.
pub fn aleo_base_committee_dir(dev: Option<u16>) -> PathBuf {
    let mut path = aleo_dir(dev);
    path.push("committee");
    path
}

///
/// Returns the base path for the storage files.
///
/// The expected paths are:
///
/// - Production: `~/.aleo/storage`.
/// - Development: `../.aleo/storage`.
///
/// Note: in development mode, the `.aleo` directory will be created in the cargo workspace as
/// opposed to the home directory, see [`aleo_dir`] for details.
///
pub fn aleo_base_storage_dir(dev: Option<u16>) -> PathBuf {
    let mut path = aleo_dir(dev);
    path.push("storage");
    path
}

///
/// Returns the directory for accessing the ledger files from Aleo storage.
///
/// The expected paths are:
///
/// - Production: `~/.aleo/storage/ledger-{network}`.
/// - Development: `../.aleo/storage/ledger-{network}-{dev_id}`.
///
/// Note: in development mode, the `.aleo` directory will be created in the cargo workspace as
/// opposed to the home directory, see [`aleo_dir`] for details.
///
pub fn aleo_ledger_dir(network: u16, dev: Option<u16>) -> PathBuf {
    let mut path = aleo_base_storage_dir(dev);

    // Construct the path to the ledger in storage.
    match dev {
        // In development mode, the ledger files are stored in a hidden folder.
        Some(id) => {
            path.push(format!("ledger-{}-{}", network, id));
        }
        // In production mode, the ledger files are stored in a visible folder.
        None => {
            path.push(format!("ledger-{}", network));
        }
    }

    path
}

///
/// Returns the directory for accessing the prover files from Aleo storage.
///
/// The expected paths are:
///
/// - Production: `~/.aleo/storage/prover-{network}`.
/// - Development: `../.aleo/storage/prover-{network}-{dev_id}`.
///
/// Note: in development mode, the `.aleo` directory will be created in the cargo workspace as
/// opposed to the home directory, see [`aleo_dir`] for details.
///
pub fn aleo_prover_dir(network: u16, dev: Option<u16>) -> PathBuf {
    let mut path = aleo_base_storage_dir(dev);

    // Construct the path to the prover in storage.
    match dev {
        // In development mode, the prover files are stored in a hidden folder.
        Some(id) => {
            path.push(format!("prover-{}-{}", network, id));
        }
        // In production mode, the prover files are stored in a visible folder.
        None => {
            path.push(format!("prover-{}", network));
        }
    }

    path
}

/// Returns the directory for accessing the BFT files from Aleo storage.
///
/// The expected paths are:
///
/// - Production: `~/.aleo/storage/bft-{network}`.
/// - Development: `../.aleo/storage/bft-{network}-{dev_id}`.
///
/// Note: in development mode, the `.aleo` directory will be created in the cargo workspace as
/// opposed to the home directory, see [`aleo_dir`] for details.
///
pub fn aleo_bft_dir(network: u16, dev: Option<u16>) -> PathBuf {
    let mut path = aleo_base_storage_dir(dev);

    // Construct the path to the ledger in storage.
    match dev {
        Some(id) => {
            path.push(format!("bft-{network}-{id}"));
        }

        None => {
            path.push(format!("bft-{network}"));
        }
    }

    path
}

///
/// Returns the path for the primary-related BFT files.
///
/// The expected paths are:
///
/// - Production: `~/.aleo/storage/bft-{network}/primary`.
/// - Development: `../.aleo/storage/bft-{network}-{dev_id}/primary`.
///
/// Note: in development mode, the `.aleo` directory will be created in the cargo workspace as
/// opposed to the home directory, see [`aleo_dir`] for details.
///
pub fn aleo_bft_primary_dir(network: u16, dev: Option<u16>) -> PathBuf {
    let mut path = aleo_bft_dir(network, dev);
    path.push("primary");
    path
}

///
/// Returns the path for the worker-related BFT files.
///
/// The expected paths are:
///
/// - Production: `~/.aleo/storage/bft-{network}/worker-{worker-id}`.
/// - Development: `../.aleo/storage/bft-{network}-{dev_id}/worker-{worker_id}`.
///
/// Note: in development mode, the `.aleo` directory will be created in the cargo workspace as
/// opposed to the home directory, see [`aleo_dir`] for details.
///
pub fn aleo_bft_worker_dir(network: u16, worker_id: u32, dev: Option<u16>) -> PathBuf {
    let mut path = aleo_bft_dir(network, dev);
    path.push(format!("worker-{worker_id}"));
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aleo_dir() {
        println!("{:?} exists: {:?}", aleo_dir(None), aleo_dir(None).exists());
    }

    #[test]
    fn test_aleo_ledger_dir() {
        println!(
            "{:?} exists: {:?}",
            aleo_ledger_dir(3, None),
            aleo_ledger_dir(3, None).exists()
        );
    }

    #[test]
    fn test_aleo_bft_primary_dir() {
        println!(
            "{:?} exists: {:?}",
            aleo_bft_primary_dir(3, None),
            aleo_bft_primary_dir(3, None).exists()
        );
    }

    #[test]
    fn test_aleo_bft_worker_dir() {
        println!(
            "{:?} exists: {:?}",
            aleo_bft_worker_dir(3, 0, None),
            aleo_bft_worker_dir(3, 0, None).exists()
        );
    }

    #[test]
    fn test_aleo_prover_dir() {
        println!(
            "{:?} exists: {:?}",
            aleo_prover_dir(3, None),
            aleo_prover_dir(3, None).exists()
        );
    }
}
