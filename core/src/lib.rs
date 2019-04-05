use frunk::{HCons, HNil};
use frunk::labelled::Transmogrifier;
use std::marker::PhantomData;

pub enum HEither<H, T> {
    Head(H),
    Tail(T),
}

pub struct Variant<K, T> {
    pub key: &'static str,
    pub value: T,
    pub name_type_holder: PhantomData<K>,
}

#[macro_export]
macro_rules! variant {
    // No name provided and type is a tuple
    (($($repeated: ty),*), $value: expr) => {
        $crate::variant!( ($($repeated),*), $value, concat!( $(stringify!($repeated)),* ) )
    };
    // No name provided and type is a tuple, but with trailing commas
    (($($repeated: ty,)*), $value: expr) => {
        $crate::variant!( ($($repeated),*), $value )
    };
    // We are provided any type, with no stable name
    ($name_type: ty, $value: expr) => {
        $crate::variant!( $name_type, $value, stringify!($name_type) )
    };
    // We are provided any type, with a stable name
    ($name_type: ty, $value: expr, $name: expr) => {
        $crate::Variant::<$name_type,_> {
            key: $name,
            value: $value,
            name_type_holder: std::marker::PhantomData,
        }
    }
}

impl<TargetHead, TargetTail, SourceHead, SourceTail, HeadIndices, TailIndices, Key>
    Transmogrifier<HEither<Variant<Key, TargetHead>, TargetTail>, HCons<HeadIndices, TailIndices>>
    for HEither<Variant<Key, SourceHead>, SourceTail>
where
    SourceHead: Transmogrifier<TargetHead, HeadIndices>,
    SourceTail: Transmogrifier<TargetTail, TailIndices>,
{
    #[inline(always)]
    fn transmogrify(self) -> HEither<Variant<Key, TargetHead>, TargetTail> {
        match self {
            HEither::Head(Variant { value: h, key: k, .. }) => HEither::Head(variant!(Key, h.transmogrify(), k)),
            HEither::Tail(t) => HEither::Tail(t.transmogrify()),
        }
    }
}

pub enum Void {}

impl Transmogrifier<Void, HNil> for Void {
    fn transmogrify(self) -> Void {
        match self {}
    }
}
