// Copyright (C) 2019-2021 Aleo Systems Inc.
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

/// An enum to define the operating mode of the Aleo node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageMode {
    /// The production mode is used for running a node on the Aleo mainnet.
    Production,
    /// The development mode is used for running a node on a local network.
    Development(u16),
    /// The custom mode is used for running a node on custom configurations.
    Custom(PathBuf),
}

impl From<Option<u16>> for StorageMode {
    fn from(dev: Option<u16>) -> Self {
        match dev {
            Some(id) => StorageMode::Development(id),
            None => StorageMode::Production,
        }
    }
}

impl From<u16> for StorageMode {
    fn from(id: u16) -> Self {
        StorageMode::Development(id)
    }
}

impl From<PathBuf> for StorageMode {
    fn from(path: PathBuf) -> Self {
        StorageMode::Custom(path)
    }
}

impl StorageMode {
    /// Returns the development ID if the mode is development.
    fn dev(&self) -> Option<u16> {
        match self {
            StorageMode::Production => None,
            StorageMode::Development(id) => Some(*id),
            StorageMode::Custom(_) => None,
        }
    }
}

///
/// Returns the directory for accessing resources from Aleo storage.
/// The expected directory path to be returned is `~/.aleo/`.
///
pub fn aleo_dir() -> PathBuf {
    // Locate the home directory as the starting point.
    // If called on a non-standard OS, use the repository directory.
    let mut path = match home_dir() {
        Some(home) => home,
        None => PathBuf::from(env!("CARGO_MANIFEST_DIR")),
    };
    // Append the Aleo directory to the path.
    path.push(ALEO_DIRECTORY);
    path
}

///
/// Returns the directory for accessing the ledger files from Aleo storage.
///
/// In production mode, the expected directory path is `~/.aleo/storage/ledger-{network}`.
/// In development mode, the expected directory path is `/path/to/repo/.ledger-{network}-{id}`.
/// In custom mode, the expected directory path is `/path/to/custom`.
///
pub fn aleo_ledger_dir(network: u16, mode: StorageMode) -> PathBuf {
    // Construct the path to the ledger in storage.
    match mode {
        // In production mode, the ledger is stored in the `~/.aleo/` directory.
        StorageMode::Production => {
            let mut path = aleo_dir();
            path.push("storage");
            path.push(format!("ledger-{}", network));
            path
        }
        // In development mode, the ledger files are stored in a hidden folder in the repository root directory.
        StorageMode::Development(id) => {
            let mut path = match std::env::current_dir() {
                Ok(current_dir) => current_dir,
                _ => PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            };
            path.push(format!(".ledger-{}-{}", network, id));
            path
        }
        // In custom mode, the ledger files are stored in the given directory path.
        StorageMode::Custom(path) => path,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aleo_dir() {
        println!("{:?} exists: {:?}", aleo_dir(), aleo_dir().exists());
    }

    #[test]
    fn test_aleo_ledger_dir() {
        println!(
            "{:?} exists: {:?}",
            aleo_ledger_dir(2, StorageMode::Production),
            aleo_ledger_dir(2, StorageMode::Production).exists()
        );
    }
}
