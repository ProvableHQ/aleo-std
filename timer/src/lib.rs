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

use std::{
    fmt,
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

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
    /// A flag used to suppress printing of the 'Finish' message in the drop() function
    /// It is set by the finish method.
    finished: AtomicBool,
    /// Any extra information to be logged along with the name. Unfortunately, due
    /// to the lifetimes associated with a `format_args!` invocation, this currently allocates
    /// if you use it.
    extra_info: Option<String>,
}

impl<'name> Timer<'name> {
    /// Constructs a new `Timer` that prints only a 'TimerFinish' message.
    /// This method is not usually called directly, use the `timer!` macro instead.
    pub fn new(
        file: &'static str,
        module_path: &'static str,
        line: u32,
        name: &'name str,
        extra_info: Option<String>,
    ) -> Option<Self> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "timer")] {
                Some(Timer {
                    start_time: Instant::now(),
                    module_path,
                    file,
                    line,
                    name,
                    finished: AtomicBool::new(false),
                    extra_info
                })
            } else {
                None
            }
        }
    }

    /// Constructs a new `Timer` that prints a 'TimerStart' and a 'TimerFinish' message.
    /// This method is not usually called directly, use the `stimer!` macro instead.
    pub fn new_with_start_message(
        file: &'static str,
        module_path: &'static str,
        line: u32,
        name: &'name str,
        extra_info: Option<String>,
    ) -> Option<Self> {
        cfg_if::cfg_if! {
            if #[cfg(feature = "timer")] {
                let timer = Self::new(file, module_path, line, name, extra_info).unwrap();
                timer.print(TimerTarget::Start, None);
                Some(timer)
            } else {
                None
            }
        }
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
        self.print(TimerTarget::Lap, args);
    }

    /// Outputs a log message with a target of 'TimerFinish' and suppresses the normal message
    /// that is output when the timer is dropped. The message can include further `format_args!`
    /// information. This method is normally called using the `finish!` macro. Calling
    /// `finish()` again will have no effect.
    pub fn finish(&self, args: Option<fmt::Arguments>) {
        if !self.finished.load(Ordering::SeqCst) {
            self.finished.store(true, Ordering::SeqCst);
            self.print(TimerTarget::Finish, args);
        }
    }

    fn print(&self, target: TimerTarget, args: Option<fmt::Arguments>) {
        match (target, self.extra_info.as_ref(), args) {
            (TimerTarget::Start, Some(info), Some(args)) => {
                self.format(target, format_args!("{}, {}, {}", self.name, info, args))
            }
            (TimerTarget::Start, Some(info), None) => self.format(target, format_args!("{}, {}", self.name, info)),
            (TimerTarget::Start, None, Some(args)) => self.format(target, format_args!("{}, {}", self.name, args)),
            (TimerTarget::Start, None, None) => self.format(target, format_args!("{}", self.name)),

            (_, Some(info), Some(args)) => self.format(
                target,
                format_args!("{}, elapsed={:?}, {}, {}", self.name, self.elapsed(), info, args),
            ),
            (_, Some(info), None) => self.format(
                target,
                format_args!("{}, elapsed={:?}, {}", self.name, self.elapsed(), info),
            ),
            (_, None, Some(args)) => self.format(
                target,
                format_args!("{}, elapsed={:?}, {}", self.name, self.elapsed(), args),
            ),
            (_, None, None) => self.format(target, format_args!("{}, elapsed={:?}", self.name, self.elapsed())),
        };
    }

    fn format(&self, target: TimerTarget, args: fmt::Arguments) {
        let target = match target {
            TimerTarget::Start => "Starting",
            TimerTarget::Lap => "Lap",
            TimerTarget::Finish => "Finished",
        };

        println!(
            "[{} {} L{}] {} {}",
            self.module_path, self.file, self.line, target, args
        );
    }
}

impl<'a> Drop for Timer<'a> {
    /// Drops the timer, outputting a log message with a target of `TimerFinish`
    /// if the `finish` method has not yet been called.
    fn drop(&mut self) {
        self.finish(None);
    }
}

#[derive(Debug, Copy, Clone)]
enum TimerTarget {
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
/// ```norun
///
/// use logging_timer::{stime, time, stimer, timer};
///
/// let _tmr1 = timer!("FIND_FILES");
/// let _tmr2 = timer!("FIND_FILES", "Found {} files", 42);
/// ```
#[macro_export]
macro_rules! timer {
    ($name:expr) => {
        {
            $crate::Timer::new_with_start_message(
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
            $crate::Timer::new_with_start_message(
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
            $crate::Timer::new_with_start_message(
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
