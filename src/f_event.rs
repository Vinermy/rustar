pub enum Event {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Backspace,
    Tab,
    Enter,
    Escape,
    Char(char),
    F(u8),
    Tick
}