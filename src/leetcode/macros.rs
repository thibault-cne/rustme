#[macro_export]
macro_rules! style {
    { $($key:literal: $value:expr),* } => {
        vec![
            $(
                ($key.to_string(), $value.to_string())
            ),*
        ]
    };
    { $($key:literal: $value:expr),*, } => {
        vec![
            $(
                ($key.to_string(), $value.to_string())
            ),*
        ]
    };
}

#[macro_export]
macro_rules! attribute {
    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        attribute_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        attribute_internal_vec![$($elems),*]
    };

    // Next element is `null`.
    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        attribute!(@array [$($elems,)* attribute!(null)] $($rest)*)
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        attribute!(@array [$($elems,)* attribute!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        attribute!(@array [$($elems,)* attribute!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        attribute!(@array [$($elems,)* attribute!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        attribute!(@array [$($elems,)* attribute!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        attribute!(@array [$($elems,)* attribute!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        attribute!(@array [$($elems,)* attribute!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        attribute!(@array [$($elems,)*] $($rest)*)
    };

    // Done
    (@map $object:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@map $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        attribute!(@map $object () ($($rest)*) ($($rest)*));
    };

    // Insert the current entry followed by trailing comma.
    (@map $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+).into(), $value);
        attribute!(@map $object () ($($rest)*) ($($rest)*));
    };

    // Insert the last entry without trailing comma.
    (@map $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+).into(), $value);
    };

    // Next value is an array.
    (@map $object:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        attribute!(@map $object [$($key)+] (attribute!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@map $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
        attribute!(@map $object [$($key)+] (attribute!({$($map)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@map $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        attribute!(@map $object [$($key)+] (attribute!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@map $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        attribute!(@map $object [$($key)+] (attribute!($value)));
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@map $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        attribute!(@map $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@map $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        attribute!(@map $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    ([]) => {
        Attribute::from(vec![])
    };

    ([ $($tt:tt)+ ]) => {
        Attribute::from(attribute!(@array [] $($tt)+))
    };

    ({}) => {
        Attribute::from(HashMap::<String, Attribute>::new())
    };

    ({ $($tt:tt)+ }) => {
        {
            let mut map = std::collections::HashMap::<String, Attribute>::new();
            attribute!(@map map () ($($tt)+) ($($tt)+));
            map
        }
    };

    ($other:expr) => {
        Attribute::from($other)
    }
}
