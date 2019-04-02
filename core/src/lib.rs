use frunk::{HCons, HNil, field, hlist, Hlist, hlist_pat};
use frunk::labelled::{Field, Transmogrifier, LabelledGeneric};
use frunk::labelled::chars::*;
use std::marker::PhantomData;

pub enum HEither<H, T> {
    Head(H),
    Tail(T),
}

pub struct Variant<K, T> {
    pub key: &'static str,
    pub value: T,
    name_type_holder: PhantomData<K>,
}

macro_rules! variant {
    // No name provided and type is a tuple
    (($($repeated: ty),*), $value: expr) => {
        variant!( ($($repeated),*), $value, concat!( $(stringify!($repeated)),* ) )
    };
    // No name provided and type is a tuple, but with trailing commas
    (($($repeated: ty,)*), $value: expr) => {
        variant!( ($($repeated),*), $value )
    };
    // We are provided any type, with no stable name
    ($name_type: ty, $value: expr) => {
        variant!( $name_type, $value, stringify!($name_type) )
    };
    // We are provided any type, with a stable name
    ($name_type: ty, $value: expr, $name: expr) => {
        $crate::Variant::<$name_type,_> {
            key: $name,
            value: $value,
            name_type_holder: PhantomData,
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

enum Void {}

impl Transmogrifier<Void, HNil> for Void {
    fn transmogrify(self) -> Void {
        match self {}
    }
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> LabelledGeneric for MyResult<T, E> {
    type Repr = HEither<Variant<(o, k), Hlist![Field<(_0), T>]>, HEither<Variant<(e, r, r), Hlist![Field<(_0), E>]>, Void>>;

    fn into(self) -> Self::Repr {
        match self {
            MyResult::Ok(t) => HEither::Head(variant!((o, k), hlist!(field!((_0), t)))),
            MyResult::Err(e) => HEither::Tail(HEither::Head(variant!((e, r, r), hlist!(field!((_0), e))))),
        }
    }

    fn from(repr: Self::Repr) -> Self {
        match repr {
            HEither::Head(Variant { value: hlist_pat!(t), .. }) => MyResult::Ok(t.value),
            HEither::Tail(HEither::Head(Variant { value: hlist_pat!(e), .. }))=> MyResult::Err(e.value),
            HEither::Tail(HEither::Tail(void)) => match void {}, // Unreachable
        }
    }
}

enum YourResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> LabelledGeneric for YourResult<T, E> {
    type Repr = HEither<Variant<(o, k), Hlist![Field<(_0), T>]>, HEither<Variant<(e, r, r), Hlist![Field<(_0), E>]>, Void>>;

    fn into(self) -> Self::Repr {
        match self {
            YourResult::Ok(t) => HEither::Head(variant!((o, k), hlist!(field!((_0), t)))),
            YourResult::Err(e) => HEither::Tail(HEither::Head(variant!((e, r, r), hlist!(field!((_0), e))))),
        }
    }

    fn from(repr: Self::Repr) -> Self {
        match repr {
            HEither::Head(Variant { value: hlist_pat!(t), .. }) => YourResult::Ok(t.value),
            HEither::Tail(HEither::Head(Variant { value: hlist_pat!(e), .. }))=> YourResult::Err(e.value),
            HEither::Tail(HEither::Tail(void)) => match void {}, // Unreachable
        }
    }
}

#[test]
fn foo() {
    let s: MyResult<String, u32> = MyResult::Ok("foo".to_string());

    let t: YourResult<String, u32> = s.transmogrify();
}
