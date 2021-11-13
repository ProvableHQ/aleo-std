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

/// Returns the directory for accessing resources from Aleo storage.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aleo_dir() {
        println!("{:?} exists: {:?}", aleo_dir(), aleo_dir().exists());
    }
}
