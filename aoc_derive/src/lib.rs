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
            let solution = #solve_fn_identifier(Input::new(&input_file));

            println!("Solutions:");
            println!("Part 1: {}", solution.part1);
            println!("Part 2: {}", solution.part2);

            let solution_to_copy = if solution.part2.is_empty() {
                solution.part1
            } else {
                solution.part2
            };

            std::process::Command::new("bash")
                .arg("-c")
                .arg(format!("echo {} | xclip -r -selection clipboard", solution_to_copy))
                .spawn()
                .expect("Failed to copy solution to clipboard");
        }
    }.into()
}
