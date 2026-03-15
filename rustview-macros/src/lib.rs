use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Marks a function as a RustView application entry point.
///
/// The function must accept `&mut Ui` as its only parameter.
///
/// # Example
/// ```ignore
/// #[rustview::app]
/// fn my_app(ui: &mut Ui) {
///     ui.write("Hello, world!");
/// }
/// ```
#[proc_macro_attribute]
pub fn app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_vis = &input.vis;

    let expanded = quote! {
        #fn_vis fn #fn_name(ui: &mut rustview::ui::Ui) {
            #fn_block
        }
    };

    TokenStream::from(expanded)
}

/// Caches the return value of a function based on its arguments.
///
/// The cache is keyed on function name + serialized arguments using FNV-1a hashing.
/// Cache is per-process, shared across sessions.
/// Maximum 128 entries per function with LRU eviction (configurable).
///
/// # Requirements
/// - Arguments must implement `Hash + Eq + Clone`
/// - Return type must implement `Clone`
///
/// # Example
/// ```ignore
/// #[rustview::cached]
/// fn expensive_computation(n: i64) -> Vec<f64> {
///     (0..n).map(|i| (i as f64).sqrt()).collect()
/// }
/// ```
#[proc_macro_attribute]
pub fn cached(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_vis = &input.vis;
    let fn_sig = &input.sig;
    let fn_block = &input.block;
    let fn_output = &fn_sig.output;
    let fn_inputs = &fn_sig.inputs;

    let fn_name_str = fn_name.to_string();

    // Extract parameter names for cache key construction
    let param_names: Vec<_> = fn_inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat_ident) = pat_type.pat.as_ref() {
                    return Some(pat_ident.ident.clone());
                }
            }
            None
        })
        .collect();

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            use std::hash::{Hash, Hasher};

            let mut hasher = fnv::FnvHasher::default();
            #fn_name_str.hash(&mut hasher);
            #( #param_names.hash(&mut hasher); )*
            let cache_key = hasher.finish();

            // Check global cache
            if let Some(cached) = rustview::cache::get_cached::<_>(#fn_name_str, cache_key) {
                return cached;
            }

            let result = (|| #fn_block)();

            rustview::cache::insert_cached(#fn_name_str, cache_key, result.clone());

            result
        }
    };

    TokenStream::from(expanded)
}
