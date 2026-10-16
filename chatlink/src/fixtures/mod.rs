pub use rstest_reuse;
pub use rstest_reuse::template;

use crate::ChatLinkType;
pub(crate) use macros::*;

pub mod achievements;
pub mod coins;
pub mod items;
pub mod maplinks;
pub mod npctext;
pub mod outfits;
pub mod recipes;
pub mod skills;
pub mod traits;
pub mod travel;
pub mod wardrobe;
pub mod wardrobe_skins;

pub const ALL: &[ChatLinkFixture] = fixture_slice!(
    coins,
    items,
    npctext,
    maplinks,
    skills,
    traits,
    recipes,
    wardrobe_skins,
    outfits,
    achievements,
    wardrobe,
    travel
);
pub const ALL_TEMPLATES: &[ChatLinkFixture] = fixture_slice!(wardrobe, travel);

#[derive(Clone, Copy, Debug)]
/// Represents a chat link test fixture
pub struct ChatLinkFixture {
    pub name: &'static str,
    pub link_type: ChatLinkType,
    pub link: &'static str,
}

impl ChatLinkFixture {
    const fn new(name: &'static str, link_type: ChatLinkType, link: &'static str) -> Self {
        ChatLinkFixture {
            name,
            link_type,
            link,
        }
    }

    pub fn wrap(&self) -> String {
        format!("[&{}]", self.link)
    }
}

fixture_cases! { all_fixtures: coins, items, npctext, maplinks, skills, traits, recipes, wardrobe_skins, outfits, achievements, wardrobe, travel }
fixture_cases! { template_fixtures: wardrobe, travel }

pub(crate) mod macros {
    /// Declares the fixture consts, ALL_TEMPLATES, and a `fixture_list!` callback macro.
    macro_rules! fixtures {
        ($d:tt $module:ident, $kind:expr; $( $name:ident = ($label:expr, $link:expr) ),* $(,)?) => {
            $( pub const $name: ChatLinkFixture = ChatLinkFixture { name: $label, link_type: $kind, link: $link }; )*
            pub const ALL: &[ChatLinkFixture] = &[$($name),*];

            macro_rules! fixture_list {
                (@collect [$d($d acc:path,)*] $d($d rest:tt)*) => {
                    with_fixtures! { @collect [$d($d acc,)* $($crate::fixtures::$module::$name,)*] $d($d rest)* }
                };
            }
            pub(crate) use fixture_list;
        };
    }
    pub(crate) use fixtures;

    macro_rules! with_fixtures {
        // public entry point: `emitter(extra args): module, module, ...`
        ($emit:ident $( ( $($args:tt)* ) )?: $($m:ident),* $(,)?) => {
            with_fixtures! { @collect [] { $emit $($($args)*)? }; $($m)* }
        };
        // internal rules
        (@collect [$($acc:path,)*] { $emit:ident $($args:tt)* };) => {
            $emit! { [$($acc),*] $($args)* }
        };
        (@collect [$($acc:path,)*] $emit:tt; $m:ident $($rest:ident)*) => {
            $m::fixture_list! { @collect [$($acc,)*] $emit; $($rest)* }
        };
    }
    pub(crate) use with_fixtures;

    macro_rules! as_slice { ([$($f:path),*]) => { &[$($f),*] }; }
    pub(crate) use as_slice;

    macro_rules! as_template {
        ([$($f:path),*] $name:ident) => {
            #[template]
            #[export]
            #[rstest]
            $( #[case($f)] )*
            pub fn $name(#[case] fixture: ChatLinkFixture) {}
        };
    }
    pub(crate) use as_template;

    macro_rules! fixture_slice { ($($m:ident),* $(,)?) => { with_fixtures! { as_slice: $($m),* } }; }
    pub(crate) use fixture_slice;

    macro_rules! fixture_cases { ($name:ident: $($m:ident),* $(,)?) => { with_fixtures! { as_template($name): $($m),* } }; }
    pub(crate) use fixture_cases;
}
