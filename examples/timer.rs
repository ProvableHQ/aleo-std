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

use aleo_std::prelude::*;

fn foo() -> u32 {
    // Start the timer.
    let timer = timer!("Arithmetic");

    // Insert expensive operation here
    let x = 1 + 1;

    // Print the elapsed time up to this point.
    lap!(timer);

    // Insert expensive operation here
    let y = 1 + 1;

    // Print the total time elapsed.
    finish!(timer);

    x + y
}

fn main() {
    foo();
}
