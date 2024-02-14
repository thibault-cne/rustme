use core::font::Font;

macro_rules! fonts {
    ($($const:ident => $type:ident);*;) => {
        $(
            pub const $const: Font = Font::$type;
        )*

        pub const ALL_FONTS: &[Font] = &[$($const),*];
    };
}

fonts!(
    BALOO_2 => Baloo2;
    FORMULA_1 => Formula1;
);
