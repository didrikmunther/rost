use std::iter;

pub type Letter = (usize, char, bool);

pub static EOF: Letter = (0, ' ', true);

pub fn get_letters<'a>(text: &'a str) -> Vec<Letter> {
    text
        .chars()
        .enumerate()
        .map(|(i, v)| (i, v, false))
        .chain(iter::once(EOF))
        .collect::<Vec<Letter>>()
}
