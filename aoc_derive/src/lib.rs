use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input};

#[proc_macro_attribute]
pub fn aoc_main(
  _args: TokenStream,
  item: TokenStream,
) -> TokenStream {
    let solve_fn = parse_macro_input!(item as syn::ItemFn);

    let solve_fn_identifier = solve_fn.sig.ident.clone();

    quote! {
        #solve_fn

        fn main() {
            let input_file = std::env::args().nth(1).expect("Expect input file as first argument");
            let solution: Solution = #solve_fn_identifier(Input::new(&input_file)).into();
            println!("{}", solution);
            solution.copy_to_clipboard();
        }
    }.into()
}
