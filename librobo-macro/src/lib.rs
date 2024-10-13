//! libroboの手続き型マクロ群クレート。

#![warn(missing_docs, rustdoc::missing_crate_level_docs)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

/// ISteeringFromSticksを実装します。
///
/// ISteeringの実装が必要です。
///
/// # Example
/// ```edition2021
/// # use robo_macro::ISteeringFromSticks;
/// #
/// #[derive(ISteeringFromSticks)]
/// # struct S;
/// #
/// # fn main() {}
/// ```
#[cfg(feature = "controller")]
#[proc_macro_derive(ISteeringFromSticks)]
pub fn derive_i_steering_from_sticks(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    #[cfg(feature = "internal")]
    let robo = quote! {
        use crate as _robo;
    };
    #[cfg(not(feature = "internal"))]
    let robo = quote! {
        extern crate robo as _robo;
    };
    let expand = quote! {
        #robo
        impl _robo::steering::ISteeringFromSticks<N> for #ident {
            /// 速度を計算する。 \[rpm]
            #[inline]
            fn calc_speed(
                steering: _robo::steering::Steering,
                pid_data: Option<&mut [_robo::steering::PIDData; N]>,
                sticks: _robo::controller::NormalizedSticks
            ) -> [i16; N] {
                <Self as _robo::steering::ISteering<N>>::calc_speed(
                    steering,
                    pid_data,
                    num::Complex::new(sticks.l[0], sticks.l[1]),
                    num::Complex::new(sticks.r[0], sticks.r[1])
                )
            }
        }
    };
    TokenStream::from(expand)
}
