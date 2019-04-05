#[derive(Debug, PartialEq, Eq, derive::LabelledGenericEnum)]
enum Colour {
    Red,
    Green,
    Blue,
    Other(i32),
}

#[derive(Debug, PartialEq, Eq, derive::LabelledGenericEnum)]
enum Color {
    Red,
    Green,
    Blue,
    Other(i32),
}

#[cfg(test)]
mod tests {
    use super::*;
    use frunk::labelled::Transmogrifier;

    #[test]
    fn unit_variants() {
        let color = Color::Red;
        let colour: Colour = color.transmogrify();
        assert_eq!(colour, Colour::Red);
    }
}
