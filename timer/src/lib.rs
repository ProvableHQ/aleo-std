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

// With credits to PhilipDaniels/logging_timer.

//! This crate implements a straightforward timer to conveniently time code blocks.

#[cfg(test)]
mod tests;

use std::{
    fmt,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::Instant,
};

pub static NUM_INDENT: AtomicUsize = AtomicUsize::new(0);
pub const PAD_CHAR: &str = "·";

/// When this struct is dropped, it logs a message stating its name and how long
/// the execution time was. Can be used to time functions or other critical areas.
pub struct Timer<'name> {
    /// The instant, in UTC, that the timer was instantiated.
    start_time: Instant,
    /// Set by the module_path!() macro to the module where the timer is instantiated.
    module_path: &'static str,
    /// Set by the file!() macro to the name of the file where the timer is instantiated.
    file: &'static str,
    /// Set by the line!() macro to the line number where the timer is instantiated.
    line: u32,
    /// The name of the timer. Used in messages to identify it.
    name: &'name str,
    /// The level of indentation for this timer context.
    indent: usize,
    /// A flag used to suppress printing of the 'Finish' message in the drop() function
    /// It is set by the finish method.
    finished: AtomicBool,
    /// Any extra information to be logged along with the name. Unfortunately, due
    /// to the lifetimes associated with a `format_args!` invocation, this currently allocates
    /// if you use it.
    extra_info: Option<String>,
}

impl<'name> Timer<'name> {
    /// Constructs a new `Timer` that prints a 'Start' and a 'Finish' message.
    /// This method is not usually called directly, use the `timer!` macro instead.
    #[cfg(feature = "timer")]
    pub fn new(
        file: &'static str,
        module_path: &'static str,
        line: u32,
        name: &'name str,
        extra_info: Option<String>,
    ) -> Option<Self> {
        let timer = Timer {
            start_time: Instant::now(),
            module_path,
            file,
            line,
            name,
            indent: NUM_INDENT.fetch_add(0, Ordering::Relaxed),
            finished: AtomicBool::new(false),
            extra_info,
        };
        timer.print(TimerState::Start, None);
        Some(timer)
    }

    /// Constructs a new `Timer` that prints a 'Start' and a 'Finish' message.
    /// This method is not usually called directly, use the `timer!` macro instead.
    #[cfg(not(feature = "timer"))]
    pub fn new(
        file: &'static str,
        module_path: &'static str,
        line: u32,
        name: &'name str,
        extra_info: Option<String>,
    ) -> Option<Self> {
        None
    }

    /// Returns how long the timer has been running for.
    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }

    /// Outputs a log message with a target of 'TimerLap' showing the current elapsed time, but does not
    /// stop the timer. This method can be called multiple times.
    /// The message can include further information via a `format_args!` approach.
    /// This method is usually not called directly, it is easier to use the `lap!` macro.
    pub fn lap(&self, args: Option<fmt::Arguments>) {
        self.print(TimerState::Lap, args);
    }

    /// Outputs a log message with a target of 'TimerFinish' and suppresses the normal message
    /// that is output when the timer is dropped. The message can include further `format_args!`
    /// information. This method is normally called using the `finish!` macro. Calling
    /// `finish()` again will have no effect.
    pub fn finish(&self, args: Option<fmt::Arguments>) {
        if !self.finished.load(Ordering::SeqCst) {
            self.finished.store(true, Ordering::SeqCst);
            self.print(TimerState::Finish, args);
        }
    }

    fn print(&self, state: TimerState, args: Option<fmt::Arguments>) {
        match (state, self.extra_info.as_ref(), args) {
            (TimerState::Start, Some(info), Some(args)) => {
                self.format(state, format_args!("{}, {}, {}", self.name, info, args))
            }
            (TimerState::Start, Some(info), None) => self.format(state, format_args!("{}, {}", self.name, info)),
            (TimerState::Start, None, Some(args)) => self.format(state, format_args!("{}, {}", self.name, args)),
            (TimerState::Start, None, None) => self.format(state, format_args!("{}", self.name)),

            (_, Some(info), Some(args)) => self.format(
                state,
                format_args!("{}, elapsed={:?}, {}, {}", self.name, self.elapsed(), info, args),
            ),
            (_, Some(info), None) => self.format(
                state,
                format_args!("{}, elapsed={:?}, {}", self.name, self.elapsed(), info),
            ),
            (_, None, Some(args)) => self.format(
                state,
                format_args!("{}, elapsed={:?}, {}", self.name, self.elapsed(), args),
            ),
            (_, None, None) => self.format(state, format_args!("{}, elapsed={:?}", self.name, self.elapsed())),
        };
    }

    fn format(&self, target: TimerState, args: fmt::Arguments) {
        let target = match target {
            TimerState::Start => "Starting",
            TimerState::Lap => "Lap",
            TimerState::Finish => "Finished",
        };

        println!(
            "[{} {} L{}] {} {}",
            self.module_path, self.file, self.line, target, args
        );
    }
}

impl<'a> Drop for Timer<'a> {
    /// Drops the timer, outputting a log message with a target of `Finish`
    /// if the `finish` method has not yet been called.
    fn drop(&mut self) {
        self.finish(None);
    }
}

#[derive(Debug, Copy, Clone)]
enum TimerState {
    Start,
    Lap,
    Finish,
}

/// Initializes a timer that logs a start and finish message.
///
/// # Examples
/// Note that when specifying the log level you must use a semi-colon as a
/// separator, this is to ensure disambiguous parsing of the macro arguments.
///
/// ```
/// use aleo_std_timer::timer;
///
/// let _tmr1 = timer!("FIND_FILES");
/// let _tmr2 = timer!("FIND_FILES", "Found {} files", 42);
/// ```
#[macro_export]
macro_rules! timer {
    ($name:expr) => {
        {
            $crate::Timer::new(
                file!(),
                module_path!(),
                line!(),
                $name,
                None,
                )
        }
    };

    ($name:expr, $format:tt) => {
        {
            $crate::Timer::new(
                file!(),
                module_path!(),
                line!(),
                $name,
                Some(format!($format)),
                )
        }
    };

    ($name:expr, $format:tt, $($arg:expr),*) => {
        {
            $crate::Timer::new(
                file!(),
                module_path!(),
                line!(),
                $name,
                Some(format!($format, $($arg), *)),
                )
        }
    };
}

/// Makes an existing timer output an 'lap' message.
/// Can be called multiple times.
#[macro_export]
macro_rules! lap {
    ($timer:expr) => ({
        if let Some(ref timer) = $timer {
            timer.lap(None);
        }
    });

    ($timer:expr, $format:tt) => ({
        if let Some(ref timer) = $timer {
            timer.lap(Some(format_args!($format)))
        }
    });

    ($timer:expr, $format:tt, $($arg:expr),*) => ({
        if let Some(ref timer) = $timer {
            timer.lap(Some(format_args!($format, $($arg), *)))
        }
    })
}

/// Makes an existing timer output a 'finished' message and suppresses
/// the normal drop message.
/// Only the first call has any effect, subsequent calls will be ignored.
#[macro_export]
macro_rules! finish {
    ($timer:expr) => ({
        if let Some(ref timer) = $timer {
            timer.finish(None)
        }
    });

    ($timer:expr, $format:tt) => ({
        if let Some(ref timer) = $timer {
            timer.finish(Some(format_args!($format)))
        }
    });

    ($timer:expr, $format:tt, $($arg:expr),*) => ({
        if let Some(ref timer) = $timer {
            timer.finish(Some(format_args!($format, $($arg), *)))
        }
    })
}
