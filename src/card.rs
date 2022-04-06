
/**
 * Every Card is represented in one byte.
 * 
 * The First four bytes are representing the color.
 * This is done in a very simple way each bit representing a color, if all bits are set its a wild card.
 * 
 * The last four bytes are representing the number or action.
 * Everything from 0-9 in binary is just a normal number.
 * Above 9 are the action cards.
 */

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

impl Into<u8> for Color {
    fn into(self) -> u8 {
        match self {
            Color::Red => 0b1000_0000,
            Color::Blue => 0b100_0000,
            Color::Green => 0b10_0000,
            Color::Yellow => 0b1_0000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Reverse,
    Skip,
    PlusTwo,
    PlusFour,
    ColorChange,
}

impl Into<u8> for Action {
    fn into(self) -> u8 {
        match self {
            Action::Reverse => 0b1010,
            Action::Skip => 0b1011,
            Action::PlusTwo => 0b1100,
            Action::PlusFour => 0b1101,
            Action::ColorChange => 0b1110,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Card {
    NumberCard(Color, u8),
    ColoredAction(Color, Action),
    WildcardAction(Action),
}

impl Into<u8> for Card {
    fn into(self) -> u8 {
        match self {
            Card::NumberCard(c, n) => c as u8 | n,
            Card::ColoredAction(c, a) => c as u8 | a as u8,
            Card::WildcardAction(a) => 0b1111_0000 | a as u8,
        }
    }
}

