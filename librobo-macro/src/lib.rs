// noinspection RsUnresolvedPath
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[cfg_attr(feature = "controller", proc_macro_derive(ISteeringFromSticks))]
// noinspection RsUnresolvedPath
pub fn derive_i_steering_from_sticks(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let expand = quote! {
        impl ISteeringFromSticks for #ident {
            fn calc_speed(
                steering: Steering,
                pid_data: Option<PIDData>,
                sticks: NormalizedSticks
            ) -> Vec<i16> {
                <Self as ISteering>::calc_speed(
                    steering,
                    pid_data,
                    Complex::new(sticks.l[0], sticks.l[1]),
                    Complex::new(sticks.r[0], sticks.r[1])
                )
            }
        }
    };
    TokenStream::from(expand)
}
