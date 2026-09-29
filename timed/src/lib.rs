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

// With credits to kardeiz/funtime.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn timed(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(mut fun) = parse::<ItemFn>(item.clone()) {
        let new_stmts = rewrite_stmts(fun.sig.ident.to_string(), &mut fun.block.stmts);
        fun.block.stmts = new_stmts;
        return quote!(#fun).into();
    }

    if let Ok(mut fun) = parse::<TraitItemMethod>(item.clone()) {
        if let Some(block) = fun.default.as_mut() {
            let new_stmts = rewrite_stmts(fun.sig.ident.to_string(), &mut block.stmts);
            block.stmts = new_stmts;
            return quote!(#fun).into();
        }
    }

    if let Ok(mut fun) = parse::<ImplItemMethod>(item) {
        let new_stmts = rewrite_stmts(fun.sig.ident.to_string(), &mut fun.block.stmts);
        fun.block.stmts = new_stmts;
        return quote!(#fun).into();
    }

    panic!("`timed` only works on functions")
}

#[cfg(feature = "timed")]
fn rewrite_stmts(name: String, stmts: &mut Vec<Stmt>) -> Vec<Stmt> {
    /// Truncates the given statement to the specified number of characters.
    fn truncate(stmt: &Stmt, len: usize) -> String {
        // Convert the statement to a string.
        let string = quote::ToTokens::to_token_stream(stmt).to_string().replace("\n", " ");
        // If the statement is too long, truncate it.
        match string.chars().count() > len {
            // Truncate the statement and append "..." to the end.
            true => string.chars().take(len).chain("...".chars()).collect::<String>(),
            // Otherwise, return the statement as-is.
            false => string,
        }
    }

    let setup: Block = parse_quote! {{
        struct Timed {
            start: std::time::Instant,
            name: &'static str,
            buffer: String,
            prev_mark: Option<std::time::Duration>,
        }

        impl Timed {
            fn new(name: &'static str) -> Self {
                use std::fmt::Write;
                let mut buffer = String::new();
                writeln!(&mut buffer, "Start: `{}`", name).unwrap();
                Timed {
                    start: std::time::Instant::now(),
                    name,
                    buffer,
                    prev_mark: None,
                }
            }

            fn mark_elapsed(&mut self, short: &str) {
                use std::fmt::Write;

                let mut elapsed = self.start.elapsed();
                if let Some(prev) = self.prev_mark.replace(elapsed) {
                    elapsed -= prev;
                }

                let elapsed = {
                    let secs = elapsed.as_secs();
                    let millis = elapsed.subsec_millis();
                    let micros = elapsed.subsec_micros() % 1000;
                    let nanos = elapsed.subsec_nanos() % 1000;
                    if secs != 0 {
                        format!("{}.{:0>3}s", secs, millis)
                    } else if millis > 0 {
                        format!("{}.{:0>3}ms", millis, micros)
                    } else if micros > 0 {
                        format!("{}.{:0>3}µs", micros, nanos)
                    } else {
                        format!("{}ns", elapsed.subsec_nanos())
                    }
                };

                writeln!(&mut self.buffer, "    {:<55} {:->25}", short, elapsed).unwrap();
            }
        }

        impl Drop for Timed {
            fn drop(&mut self) {
                use std::fmt::Write;
                writeln!(&mut self.buffer, "End: `{}` took {:?}", self.name, self.start.elapsed()).unwrap();
                print!("{}", &self.buffer);
            }
        }

        let mut timed = Timed::new(#name);

    }};

    const LENGTH: usize = 45;

    let mut new_stmts = setup.stmts;

    let last = stmts.pop();

    for (index, stmt) in stmts.drain(..).enumerate() {
        let short = truncate(&stmt, LENGTH);
        let short = format!("L{index}: {short}");

        let next_stmt = parse_quote!(timed.mark_elapsed(#short););

        new_stmts.push(stmt);
        new_stmts.push(next_stmt);
    }

    if let Some(stmt) = last {
        let short = truncate(&stmt, LENGTH);

        let new_stmt = parse_quote! {
            let return_stmt = { #stmt };
        };
        let next_stmt = parse_quote!(timed.mark_elapsed(#short););
        let return_stmt = parse_quote!(return return_stmt;);

        new_stmts.push(new_stmt);
        new_stmts.push(next_stmt);
        new_stmts.push(return_stmt);
    }

    new_stmts
}

#[cfg(not(feature = "timed"))]
fn rewrite_stmts(_name: String, stmts: &mut [Stmt]) -> Vec<Stmt> {
    stmts.to_vec()
}
