// Copyright (c) 2019-2026 Provable Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:

// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(feature = "cpu")]
pub use aleo_std_cpu::{Cpu, get_cpu};
pub use aleo_std_profiler::*;
#[cfg(feature = "storage")]
pub use aleo_std_storage::{StorageMode, aleo_dir, aleo_ledger_dir};

pub mod prelude {
    #[cfg(feature = "cpu")]
    pub use aleo_std_cpu::{Cpu, get_cpu};
    pub use aleo_std_profiler::*;
    #[cfg(feature = "storage")]
    pub use aleo_std_storage::{StorageMode, aleo_dir, aleo_ledger_dir};
    pub use aleo_std_time::time;
    pub use aleo_std_timed::timed;
    pub use aleo_std_timer::{finish, lap, timer};
}
