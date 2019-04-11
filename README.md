# Frunk Enum Support

These crates augment the [frunk](https://docs.rs/frunk/latest/frunk/) crate to allow transmogrification of `enum`s as well as `struct`s.  For more information on transmogrification and the `LabelledGeneric` trait it's based around, see https://docs.rs/frunk/latest/frunk/#transmogrifying and https://docs.rs/frunk/latest/frunk/labelled/trait.LabelledGeneric.html.

To take advantage of this feature for your own enum you need to:

* Add the `frunk-enum-derive` crate to your `Cargo.toml`
* Mark the enum with the custom derive:

    ```
    #[derive(LabelledGenericEnum)]
    enum Foo {
        Bar(String),
        Baz { name: String, id: Identity },
    }
    ```
* Add the `frunk-enum-core` and `frunk-core` crates to your `Cargo.toml`
* Then (assuming there's a `NewFoo` enum with the same structure as `Foo`) you can write:

    ```
    let foo = Foo::Baz { name: "Andy".into(), id: Identity };
    let new_foo: NewFoo = foo.transmogrify();
    ```

This works by deriving an implementation of `LabelledGeneric` for `Foo` which allows conversion to and from an instance of a generic sum-type.  The core crate provides tools for converting between these generic sum-types where the bodies of the variants are recursively transmogrifiable.  This allows for arbitrarily deep conversion between types, especially useful where the two types in question are autogenerated from some common input file, but are deemed to be different types by the rust compiler (e.g. because they're in separate crates).
