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

            println!("Solutions:");
            println!("Part 1: {}", solution.part1.clone().unwrap_or("N/A".to_string()));
            println!("Part 2: {}", solution.part2.clone().unwrap_or("N/A".to_string()));

            let solution_to_copy = if solution.part2.is_none() {
                solution.part1
            } else {
                solution.part2
            };

            if let Some(solution_to_copy) = solution_to_copy {
                std::process::Command::new("bash")
                    .arg("-c")
                    .arg(format!("echo {} | xclip -r -selection clipboard", solution_to_copy))
                    .spawn()
                    .expect("Failed to copy solution to clipboard");
            }
        }
    }.into()
}
