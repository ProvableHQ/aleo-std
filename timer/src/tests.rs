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

use super::*;

#[test]
fn test_timer_finish() {
    let timer = timer!("Hello");
    finish!(timer);
}

#[test]
fn test_timer_lap_finish() {
    let timer = timer!("Hello");
    lap!(timer);
    finish!(timer);
}

#[test]
fn test_timer_timer_finish_finish() {
    let hello = timer!("Hello");
    let world = timer!("World");
    finish!(world);
    finish!(hello);
}

#[test]
fn test_timer_timer_lap_finish_finish() {
    let hello = timer!("Hello World");
    let world = timer!("Testing");
    lap!(hello);
    finish!(world);
    finish!(hello);
}
