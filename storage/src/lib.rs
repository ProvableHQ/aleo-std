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

    let mut path = PathBuf::from(std::str::from_utf8(&output).unwrap().trim());
    path.pop();

    path
}

///
/// Returns the base path for the storage files.
///
pub fn base_storage_path(dev: Option<u16>) -> PathBuf {
    // Retrieve the starting directory.
    match dev.is_some() {
        // In development mode, the ledger is stored in the root directory of the repository.
        true => workspace_dir().into(),
        // In production mode, the ledger is stored in the `~/.aleo/` directory.
        false => aleo_dir(),
    }
}

///
/// Returns the directory for accessing the ledger files from Aleo storage.
///
/// In production mode, the expected directory path is `~/.aleo/storage/ledger-{network}`.
/// In development mode, the expected directory path is `/path/to/repo/.ledger-{network}-{id}`.
///
pub fn aleo_ledger_dir(network: u16, dev: Option<u16>) -> PathBuf {
    let mut path = base_storage_path(dev);

    // Construct the path to the ledger in storage.
    match dev {
        // In development mode, the ledger files are stored in a hidden folder.
        Some(id) => {
            path.push(format!(".ledger-{}-{}", network, id));
            path
        }
        // In production mode, the ledger files are stored in a visible folder.
        None => {
            path.push("storage");
            path.push(format!("ledger-{}", network));
            path
        }
    }
}

///
/// Returns the directory for accessing the prover files from Aleo storage.
///
/// In production mode, the expected directory path is `~/.aleo/storage/prover-{network}`.
/// In development mode, the expected directory path is `/path/to/repo/.prover-{network}-{id}`.
///
pub fn aleo_prover_dir(network: u16, dev: Option<u16>) -> PathBuf {
    let mut path = base_storage_path(dev);

    // Construct the path to the prover in storage.
    match dev {
        // In development mode, the prover files are stored in a hidden folder.
        Some(id) => {
            path.push(format!(".prover-{}-{}", network, id));
            path
        }
        // In production mode, the prover files are stored in a visible folder.
        None => {
            path.push("storage");
            path.push(format!("prover-{}", network));
            path
        }
    }
}

///
/// Returns the path for the primary-related BFT files.
///
/// In production mode, the expected directory path is `~/.aleo/storage/bft-{network}/primary`.
/// In development mode, the expected directory path is `path/to/repo/.bft-storage-{network}/primary-{id}`.
///
pub fn aleo_bft_primary_dir(network: u16, dev: Option<u16>) -> PathBuf {
    let mut path = base_storage_path(dev);

    // Construct the path to the ledger in storage.
    match dev {
        Some(id) => {
            path.push(format!(".bft-storage-{network}"));
            path.push(format!("primary-{id}"));
        }

        None => {
            path.push("storage");
            path.push(format!("bft-{network}"));
            path.push("primary");
        }
    }

    path
}

///
/// Returns the path for the worker-related BFT files.
///
/// In production mode, the expected directory path is `~/.aleo/storage/bft-{network}/worker-{worker_id}`.
/// In development mode, the expected directory path is `path/to/repo/.bft-storage-{network}/worker-{primary_id}-{worker_id}`.
///
pub fn aleo_bft_worker_dir(network: u16, worker_id: u32, dev: Option<u16>) -> PathBuf {
    // Retrieve the starting directory.
    let mut path = base_storage_path(dev);

    // Construct the path to the ledger in storage.
    match dev {
        Some(primary_id) => {
            path.push(format!(".bft-storage-{network}"));
            path.push(format!("worker-{primary_id}-{worker_id}"));
        }

        None => {
            path.push("storage");
            path.push(format!("bft-{network}"));
            path.push(format!("worker-{worker_id}"));
        }
    }

    path
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
