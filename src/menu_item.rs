pub enum MenuItem {
    Main,
    Ships,
    Planets,
}

impl MenuItem {
    pub fn next(&self) -> Self {
        match self {
            Self::Main => { Self::Ships }
            Self::Ships => { Self::Planets }
            Self::Planets => { Self::Main }
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Self::Main => { Self::Planets }
            Self::Ships => { Self::Main }
            Self::Planets => { Self::Ships }
        }
    }
}

impl From<&MenuItem> for usize {
    fn from(value: &MenuItem) -> Self {
        match value {
            MenuItem::Main => { 0 }
            MenuItem::Ships => { 1 }
            MenuItem::Planets => { 2 }
        }
    }
}

