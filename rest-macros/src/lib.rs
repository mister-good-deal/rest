use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Item, ItemFn, ItemMod, parse_macro_input,
    visit_mut::{self, VisitMut},
};

/// Registers a function to be run once before any test in the current module
///
/// Example:
/// ```
/// use rest::prelude::*;
///
/// #[before_all]
/// fn setup_once() {
///     // Initialize test environment once for all tests
/// }
/// ```
#[proc_macro_attribute]
pub fn before_all(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    // Create a unique registration function name based on the function name
    let register_fn_name = syn::Ident::new(&format!("__register_before_all_fixture_{}", fn_name), fn_name.span());

    let output = quote! {
        #input_fn

        // We use ctor to register the function at runtime
        #[ctor::ctor]
        fn #register_fn_name() {
            rest::backend::fixtures::register_before_all(
                module_path!(),
                Box::new(|| #fn_name())
            );
        }
    };

    TokenStream::from(output)
}

/// Registers a function to be run once after all tests in the current module
///
/// Example:
/// ```
/// use rest::prelude::*;
///
/// #[after_all]
/// fn teardown_once() {
///     // Clean up test environment after all tests
/// }
/// ```
#[proc_macro_attribute]
pub fn after_all(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    // Create a unique registration function name based on the function name
    let register_fn_name = syn::Ident::new(&format!("__register_after_all_fixture_{}", fn_name), fn_name.span());

    let output = quote! {
        #input_fn

        // We use ctor to register the function at runtime
        #[ctor::ctor]
        fn #register_fn_name() {
            rest::backend::fixtures::register_after_all(
                module_path!(),
                Box::new(|| #fn_name())
            );
        }
    };

    TokenStream::from(output)
}

/// Registers a function to be run before each test in the current module
///
/// Example:
/// ```
/// use rest::prelude::*;
///
/// #[setup]
/// fn setup() {
///     // Initialize test environment
/// }
/// ```
#[proc_macro_attribute]
pub fn setup(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    // Create a unique registration function name based on the function name
    let register_fn_name = syn::Ident::new(&format!("__register_setup_fixture_{}", fn_name), fn_name.span());

    let output = quote! {
        #input_fn

        // We use ctor to register the function at runtime
        #[ctor::ctor]
        fn #register_fn_name() {
            rest::backend::fixtures::register_setup(
                module_path!(),
                Box::new(|| #fn_name())
            );
        }
    };

    TokenStream::from(output)
}

/// Registers a function to be run after each test in the current module
///
/// Example:
/// ```
/// use rest::prelude::*;
///
/// #[tear_down]
/// fn tear_down() {
///     // Clean up test environment
/// }
/// ```
#[proc_macro_attribute]
pub fn tear_down(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    // Create a unique registration function name based on the function name
    let register_fn_name = syn::Ident::new(&format!("__register_teardown_fixture_{}", fn_name), fn_name.span());

    let output = quote! {
        #input_fn

        // We use ctor to register the function at runtime
        #[ctor::ctor]
        fn #register_fn_name() {
            rest::backend::fixtures::register_teardown(
                module_path!(),
                Box::new(|| #fn_name())
            );
        }
    };

    TokenStream::from(output)
}

/// Runs a function with setup and teardown fixtures from the current module
///
/// Example:
/// ```
/// use rest::prelude::*;
///
/// #[with_fixtures]
/// fn test_something() {
///     // Test code here
///     expect!(2 + 2).to_equal(4);
/// }
/// ```
#[proc_macro_attribute]
pub fn with_fixtures(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let vis = &input_fn.vis; // Preserve visibility
    let attrs = &input_fn.attrs; // Preserve attributes
    let sig = &input_fn.sig; // Get function signature

    // Generate a unique internal name for the real implementation
    let impl_name = syn::Ident::new(&format!("__{}_impl", fn_name), fn_name.span());

    let output = quote! {
        // Define the implementation function with a private name
        fn #impl_name() #fn_body

        // Create the public function with fixtures
        #(#attrs)*
        #vis #sig {
            // Get the current module path - critical for finding the right fixtures
            let module_path = module_path!();

            rest::backend::fixtures::run_test_with_fixtures(
                module_path,
                std::panic::AssertUnwindSafe(|| #impl_name())
            );
        }
    };

    TokenStream::from(output)
}

/// A struct to visit all functions in a module and add the with_fixtures attribute to test functions
struct TestFunctionVisitor {}

impl VisitMut for TestFunctionVisitor {
    fn visit_item_fn_mut(&mut self, node: &mut ItemFn) {
        // First check if this is a test function (has #[test] attribute)
        let is_test = node.attrs.iter().any(|attr| attr.path().is_ident("test"));

        // Check if it already has the with_fixtures attribute
        let already_has_fixtures = node.attrs.iter().any(|attr| attr.path().is_ident("with_fixtures"));

        // Only add the with_fixtures attribute if this is a test function and doesn't already have it
        if is_test && !already_has_fixtures {
            // Create the with_fixtures attribute
            let with_fixtures_attr: Attribute = syn::parse_quote!(#[with_fixtures]);

            // Add it to the function's attributes
            node.attrs.push(with_fixtures_attr);
        }

        // Continue visiting the function's items
        visit_mut::visit_item_fn_mut(self, node);
    }
}

/// Runs all test functions in a module with setup and teardown fixtures
///
/// Example:
/// ```
/// use rest::prelude::*;
///
/// #[with_fixtures_module]
/// mod test_module {
///     #[setup]
///     fn setup() {
///         // Initialize test environment
///     }
///     
///     #[tear_down]
///     fn tear_down() {
///         // Clean up test environment
///     }
///     
///     fn test_something() {
///         // Test code - will automatically run with fixtures
///         expect!(2 + 2).to_equal(4);
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn with_fixtures_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_mod = parse_macro_input!(item as ItemMod);

    // Only process if we have a defined module body
    if let Some((_, items)) = &mut input_mod.content {
        // Visit all items in the module
        let mut visitor = TestFunctionVisitor {};
        for item in items.iter_mut() {
            // Check if the item is a function
            if let Item::Fn(func) = item {
                visitor.visit_item_fn_mut(func);
            }

            // Recursively process nested modules as well
            if let Item::Mod(nested_mod) = item
                && let Some((_, nested_items)) = &mut nested_mod.content
            {
                for nested_item in nested_items.iter_mut() {
                    if let Item::Fn(func) = nested_item {
                        visitor.visit_item_fn_mut(func);
                    }
                }
            }
        }
    }

    // Convert back to token stream
    TokenStream::from(quote! {
        #input_mod
    })
}
