#![deny(warnings)]
#![feature(assert_matches)]

#[cfg(all(
    not(feature = "private"),
    any(feature = "gateway", feature = "gateway_objects")
))]
pub mod gateway;

#[cfg(all(
    feature = "private",
    any(feature = "gateway", feature = "gateway_objects")
))]
mod gateway;

#[cfg(all(
    not(feature = "private"),
    any(feature = "api", feature = "api_objects")
))]
pub mod api;

#[cfg(all(feature = "private", any(feature = "api", feature = "api_objects")))]
mod api;

/// Calculates the size of an array at compile time.
///
/// Usage:
/// ```rust
/// macro_rules! foo {
///     ($($variant:ident),+) => {
///         {
///             let variants: [Foo; ::datrope::array_size!($(Foo::$variant,)+)] = [$(Foo::$variant,)+];
///             variants
///         }
///     }
/// }
/// enum Foo {
///     Hello,
///     World,
/// }
/// let all_variants = foo!(Hello, World);
/// assert_eq!(2, all_variants.len());
/// ```
#[macro_export]
macro_rules! array_size {
    () => { 0 };
    ($head:expr, $($tail:expr,)*) => {
        1 + $crate::array_size!($($tail,)*)
    };
}

/// HashSet for an enum with support for serializing to and deserializing from an integer.
///
/// Usage:
/// ```rust
/// datrope::flags!(your_flags {
///     Foo = 1 << 1,
///     Bar = 1 << 2,
/// });
/// let flags = datrope::flags!(your_flags(Foo | Bar));
/// ```
///
/// If `isize` doesn't work for your type, you can specify the size yourself:
/// ```rust
/// datrope::flags!(your_flags: i8 {
///     Foo = 1 << 1,
///     Bar = 1 << 2,
/// });
/// let flags = datrope::flags!(your_flags(Foo | Bar));
/// ```
#[macro_export]
macro_rules! flags {
    ( $ident:ident { $($types:ident = $values:expr),+ $(,)? } ) => {
        $crate::flags!($ident:isize { $($types = $values),+ });
    };
    ( $ident:ident:$repr:ty { $($types:ident = $values:expr),+ $(,)? } ) => {
        pub mod $ident {
            type Repr = $repr;
            #[cfg(feature = "serde")]
            use ::serde::{Deserialize, Serialize};
            #[allow(dead_code)]
            #[derive(Eq, PartialEq, Hash)]
            #[cfg_attr(any(feature = "clone", feature = "serde"), derive(Clone))]
            #[cfg_attr(feature = "debug", derive(Debug))]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            #[repr($repr)]
            pub enum Flag {
                $($types = $values),+
            }

            impl Flag {
                fn discriminant(&self) -> $repr {
                    match self {
                        $(Self::$types => $values),+
                    }
                }

                fn iter() -> [Flag; $crate::array_size!($(Self::$types,)+)] {
                    [
                        $(Self::$types),+
                    ]
                }
            }

            #[allow(dead_code)]
            #[derive(Eq, PartialEq)]
            #[cfg_attr(any(feature = "clone", feature = "serde"), derive(Clone))]
            #[cfg_attr(feature = "debug", derive(Debug))]
            #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
            #[cfg_attr(feature = "serde", serde(from = "Repr"))]
            #[cfg_attr(feature = "serde", serde(into = "Repr"))]
            pub struct Flags(::std::collections::HashSet<Flag>);

            impl Flags {
                #[allow(dead_code)]
                pub fn new(flags: impl Iterator<Item = Flag>) -> Self {
                    Flags(::std::collections::HashSet::from_iter(flags))
                }
            }

            impl From<Flags> for Repr {
                fn from(value: Flags) -> Self {
                    value.0.iter().map(|flag| flag.discriminant()).sum()
                }
            }

            impl From<Repr> for Flags {
                fn from(value: Repr) -> Self {
                    let mut flags = vec![];
                    for flag in Flag::iter() {
                        if value & flag.discriminant() == flag.discriminant() {
                            flags.push(flag);
                        }
                    }
                    Flags::new(flags.into_iter())
                }
            }
        }
    };
    ( $ident:ident( $( $types:ident)|* $(|)? ) ) => {
        $ident::Flags::new([$($ident::Flag::$types),*].into_iter())
    };
}
