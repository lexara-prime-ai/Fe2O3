fn main() {
    print_to_console();
}

fn print_to_console() {
    let num: i32 = 16;
    let target: i32 = 12;

    loop {
        
    }
}

// Define a struct Point with x and y fields. Destructure an instance of it.

// Question 4: How do you create a procedural macro in Rust?

//Answer: To create a procedural macro in Rust, you need to define a separate crate with a special attribute. Procedural macros use the proc_macro crate. Here's a simplified example of a procedural macro that generates getter methods for struct fields:

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput};

// #[proc_macro]
// pub fn generate_getters(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     let struct_name = &input.ident;
//     let fields = input.data.fields.iter().map(|field| &field.ident);

//     let gen = quote! {
//         impl #struct_name {
//             #(pub fn #fields(&self) -> &#fields::type { &self.#fields })*
//         }
//     };

//     gen.into()
// }
