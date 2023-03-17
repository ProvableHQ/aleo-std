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

// With credits to PhilipDaniels/logging_timer.

#[cfg(feature = "time")]
use quote::quote;

#[cfg(feature = "time")]
const DEFAULT_LEVEL: &str = "debug";
#[cfg(feature = "time")]
const DEFAULT_NAME_PATTERN: &str = "{}";

#[cfg(feature = "time")]
fn extract_literal(token_tree: &proc_macro::TokenTree) -> String {
    let s = match token_tree {
        proc_macro::TokenTree::Literal(literal) => literal.to_string(),
        _ => panic!(
            "Invalid argument. Specify at most two string literal arguments, for log level and name pattern, in that order."
        ),
    };

    // String literals seem to come through including their double quotes. Trim them off.
    let s = s.trim().trim_matches('"').trim().to_string();
    s
}

// We also allow 'Never' to mean disable timer instrumentation
// altogether. Any casing is allowed.
#[cfg(feature = "time")]
fn get_log_level_and_name_pattern(metadata: proc_macro::TokenStream) -> (String, String) {
    // Grab everything into a vector and filter out any intervening punctuation
    // (commas come through as TokenTree::Punct(_)).
    let macro_args: Vec<proc_macro::TokenTree> = metadata
        .into_iter()
        .filter(|token| matches!(token, proc_macro::TokenTree::Literal(_)))
        .collect();

    if macro_args.is_empty() {
        return (DEFAULT_LEVEL.to_string(), DEFAULT_NAME_PATTERN.to_string());
    }

    if macro_args.len() > 2 {
        panic!("Specify at most two string literal arguments, for log level and name pattern");
    }

    let first_arg = extract_literal(&macro_args[0]);

    if first_arg.contains("{}") && macro_args.len() == 2 {
        panic!("Invalid first argument. Specify the log level as the first argument and the pattern as the second.");
    }

    let first_arg_lower = first_arg.to_ascii_lowercase();
    if macro_args.len() == 1 {
        match first_arg_lower.as_str() {
            "error" | "warn" | "info" | "debug" | "trace" | "never" => {
                // User specified a valid log level as their only argument.
                return (first_arg_lower, DEFAULT_NAME_PATTERN.to_string());
            }
            _ => {
                // User specified something that doesn't look like a log-level.
                // It may be a pattern with "{}", or it may just be a string.
                // In any case, consider it to be the pattern and return it
                // n.b. the original, not the lowered version.
                return (DEFAULT_LEVEL.to_string(), first_arg);
            }
        }
    }

    // We have two arguments. We are stricter on the first now, we require
    // that to be a valid log level.
    match first_arg_lower.as_str() {
        "error" | "warn" | "info" | "debug" | "trace" | "never" => {
            let mut second_arg = extract_literal(&macro_args[1]);
            if second_arg.is_empty() {
                second_arg += DEFAULT_NAME_PATTERN;
            }

            (first_arg_lower, second_arg.to_string())
        }
        _ => {
            panic!("Invalid first argument. Specify the log level as the first argument and the pattern as the second.")
        }
    }
}

#[cfg(feature = "time")]
fn get_timer_name(name_pattern: &str, function_name: &str) -> String {
    let function_name_with_parenthesis = format!("{}()", function_name);
    name_pattern.replacen("{}", &function_name_with_parenthesis, 1)
}

/// Instruments the function with an `timer!`, which logs two messages, one at the start
/// of the function and one at the end of execution stating the elapsed time.
///
/// The attribute accepts two string literals as arguments. The first is the log level,
/// valid values of which are "error", "warn", "info", "debug", "trace" or "never".
/// The default value is "debug". "never" can be used to temporarily disable instrumentation
/// of the function without deleting the attribute.
///
/// The second argument is the function name pattern. The pattern is helpful to
/// disambiguate functions when you have many functions in the same module with the same
/// name: `new` might occur many times on different structs, for example. In the pattern,
/// "{}" will be replaced with the name of the function.
///
/// Examples:
///     #[time]                                 // Use default print level
///     #[time("FirstStruct::{}")]              // Prints "FirstStruct::new()"
///     #[time("never")]                        // Turn off instrumentation at compile time
#[cfg(feature = "time")]
#[proc_macro_attribute]
pub fn time(metadata: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (level, name_pattern) = get_log_level_and_name_pattern(metadata);

    if level != "never" {
        let input_fn: syn::ItemFn = syn::parse_macro_input!(input as syn::ItemFn);
        let visibility = input_fn.vis;
        let ident = input_fn.sig.ident;
        let inputs = input_fn.sig.inputs;
        let output = input_fn.sig.output;
        let generics = &input_fn.sig.generics;
        let where_clause = &input_fn.sig.generics.where_clause;
        let block = input_fn.block;

        let timer_name = get_timer_name(&name_pattern, &ident.to_string());

        (quote!(
            #visibility fn #ident #generics (#inputs) #output #where_clause {
                let _tmr = ::aleo_std::prelude::timer!(#timer_name);
                #block
            }
        ))
        .into()
    } else {
        input
    }
}

#[cfg(not(feature = "time"))]
#[proc_macro_attribute]
pub fn time(_metadata: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}
