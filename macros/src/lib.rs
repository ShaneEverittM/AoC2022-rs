extern crate proc_macro;

use proc_macro::TokenStream;
use std::ops::RangeInclusive;

use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitInt, Token};

struct Days {
    lower: u32,
    upper: u32,
}

impl Parse for Days {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lower = input.parse::<LitInt>()?.base10_parse::<u32>()?;
        input.parse::<Token![,]>()?;
        let upper = input.parse::<LitInt>()?.base10_parse::<u32>()?;
        Ok(Days { lower, upper })
    }
}

#[proc_macro]
pub fn benchmark_days(input: TokenStream) -> TokenStream {
    let days = parse_macro_input!(input as Days);

    let days = RangeInclusive::new(days.lower, days.upper);
    let mut part1_bench_idents = Vec::new();
    let mut mod_idents = Vec::new();

    let mut part2_bench_idents = Vec::new();
    for day in days.clone() {
        part1_bench_idents.push(format_ident!("bench_day{}_part1", day));
        part2_bench_idents.push(format_ident!("bench_day{}_part2", day));
        mod_idents.push(format_ident!("day{}", day));
    }

    let tokens = quote! {
        #(
            fn #part1_bench_idents(c: &mut criterion::Criterion) { c.bench_function(concat!("Day ", #days ," P1"), |b| b.iter(#mod_idents::part1)); }
            fn #part2_bench_idents(c: &mut criterion::Criterion) { c.bench_function(concat!("Day ", #days ," P1"), |b| b.iter(#mod_idents::part2)); }
        )*

        criterion::criterion_group!(
                benches,
                #(
                    #part1_bench_idents,
                    #part2_bench_idents
                ),*
        );

        criterion::criterion_main!(benches);
    };

    tokens.into()
}
