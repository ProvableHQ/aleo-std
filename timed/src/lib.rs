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
    fn truncate_stmt(stmt: &Stmt, len: usize) -> String {
        let short = format!("{}", quote::ToTokens::to_token_stream(stmt))
            .chars()
            .collect::<Vec<_>>();

        let short = if short.len() > len {
            let mut short = short[..(len - 3)].into_iter().collect::<String>();
            short.push_str("...");
            short
        } else {
            short.into_iter().collect::<String>()
        };

        short
    }

    let setup: Block = parse_quote! {{
        struct Timed {
            start: std::time::Instant,
            name: &'static str,
            buffer: String,
            prev_mark: Option<std::time::Duration>,
        }

        impl Drop for Timed {
            fn drop(&mut self) {
                use std::fmt::Write;
                writeln!(&mut self.buffer, "end: `{}` took {:?}", self.name, self.start.elapsed()).unwrap();
                print!("{}", &self.buffer);
            }
        }

        impl Timed {
            fn new(name: &'static str) -> Self {
                use std::fmt::Write;
                let mut buffer = String::new();
                writeln!(&mut buffer, "start: `{}`", name).unwrap();
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
                    elapsed = elapsed - prev;
                }
                writeln!(&mut self.buffer, "  took {:?}: `{}`", elapsed, short).unwrap();
            }
        }

        let mut timed = Timed::new(#name);

    }};

    let mut new_stmts = setup.stmts;

    let last = stmts.pop();

    for stmt in stmts.drain(..) {
        let short = truncate_stmt(&stmt, 40);

        let next_stmt = parse_quote!(timed.mark_elapsed(#short););

        new_stmts.push(stmt);
        new_stmts.push(next_stmt);
    }

    if let Some(stmt) = last {
        let short = truncate_stmt(&stmt, 40);
        let new_stmt = parse_quote! {
            let funtime_return_val = {
                #stmt
            };
        };

        let next_stmt = parse_quote!(timed.mark_elapsed(#short););
        let return_stmt = parse_quote!(return funtime_return_val;);

        new_stmts.push(new_stmt);
        new_stmts.push(next_stmt);
        new_stmts.push(return_stmt);
    }

    new_stmts
}

#[cfg(not(feature = "timed"))]
fn rewrite_stmts(_name: String, stmts: &mut Vec<Stmt>) -> Vec<Stmt> {
    stmts
}
