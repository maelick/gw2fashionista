macro_rules! id_kind {
    ($alias:ident = $kind:ident: $value:ty, width = $width:expr $(, padding = $padding:expr)? $(, default = $default:expr)?) => {
        #[derive(Copy, Clone, Hash, PartialEq, Eq)]
        pub enum $kind {}

        impl $crate::id::IdKind for $kind {
            type Value = $value;
            const WIDTH: usize = $width;
            $(const PADDING: usize = $padding;)?
            const NAME: &'static str = stringify!($alias);
        }

        pub type $alias = $crate::id::Id<$kind>;

        $(impl Default for $alias {
            fn default() -> Self {
                Self::new($default)
            }
        })?

        impl From<$value> for $alias {
            fn from(id: $value) -> Self {
                Self::new(id)
            }
        }

        impl From<$alias> for $value {
            fn from(id: $alias) -> Self {
                id.value()
            }
        }
    };
}
pub(crate) use id_kind;
