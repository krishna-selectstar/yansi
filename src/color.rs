use std::fmt;

use {Paint, Style};

/// An enum representing an ANSI color code.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub enum Color {
    /// No color has been set. Nothing is changed when applied.
    Unset,

    /// Terminal default #9. (foreground code `39`, background code `49`).
    Default,

    /// Black #0 (foreground code `30`, background code `40`).
    Black,

    /// Red: #1 (foreground code `31`, background code `41`).
    Red,

    /// Green: #2 (foreground code `32`, background code `42`).
    Green,

    /// Yellow: #3 (foreground code `33`, background code `43`).
    Yellow,

    /// Blue: #4 (foreground code `34`, background code `44`).
    Blue,

    /// Magenta: #5 (foreground code `35`, background code `45`).
    Magenta,

    /// Cyan: #6 (foreground code `36`, background code `46`).
    Cyan,

    /// White: #7 (foreground code `37`, background code `47`).
    White,

    /// Bright black #0 (foreground code `90`, background code `100`).
    BrightBlack,

    /// Bright red: #1 (foreground code `91`, background code `101`).
    BrightRed,

    /// Bright green: #2 (foreground code `92`, background code `102`).
    BrightGreen,

    /// Bright yellow: #3 (foreground code `93`, background code `103`).
    BrightYellow,

    /// Bright blue: #4 (foreground code `94`, background code `104`).
    BrightBlue,

    /// Bright magenta: #5 (foreground code `95`, background code `105`).
    BrightMagenta,

    /// Bright cyan: #6 (foreground code `96`, background code `106`).
    BrightCyan,

    /// Bright white: #7 (foreground code `97`, background code `107`).
    BrightWhite,

    /// A color number from 0 to 255, for use in 256-color terminals.
    Fixed(u8),

    /// A 24-bit RGB color, as specified by ISO-8613-3.
    RGB(u8, u8, u8),
}

impl Color {
    /// Constructs a new `Paint` structure that encapsulates `item` with the
    /// foreground color set to the color `self`.
    ///
    /// ```rust
    /// use yansi::Color::Blue;
    ///
    /// println!("This is going to be blue: {}", Blue.paint("yay!"));
    /// ```
    #[inline]
    pub fn paint<T>(self, item: T) -> Paint<T> {
        Paint::new(item).fg(self)
    }

    /// Constructs a new `Style` structure with the foreground color set to the
    /// color `self`.
    ///
    /// ```rust
    /// use yansi::Color::Green;
    ///
    /// let success = Green.style().bold();
    /// println!("Hey! {}", success.paint("Success!"));
    /// ```
    #[inline]
    pub fn style(self) -> Style {
        Style::new(self)
    }

    pub fn is_bright(&self) -> bool {
        match *self {
            Color::Unset
            | Color::Default
            | Color::Black
            | Color::Red
            | Color::Green
            | Color::Yellow
            | Color::Blue
            | Color::Magenta
            | Color::Cyan
            | Color::White
            | Color::Fixed(_)
            | Color::RGB(_, _, _) => false,
            Color::BrightBlack
            | Color::BrightRed
            | Color::BrightGreen
            | Color::BrightYellow
            | Color::BrightBlue
            | Color::BrightMagenta
            | Color::BrightCyan
            | Color::BrightWhite => true,
        }
    }

    pub(crate) fn ansi_fmt(&self, f: &mut dyn fmt::Write, is_background: bool) -> fmt::Result {
        match (is_background, self.is_bright()) {
            (true, true) => write!(f, "10"),
            (false, true) => write!(f, "9"),
            (true, false) => write!(f, "4"),
            (false, false) => write!(f, "3"),
        }?;

        match *self {
            Color::Unset => Ok(()),
            Color::Default => write!(f, "9"),
            Color::Black | Color::BrightBlack => write!(f, "0"),
            Color::Red | Color::BrightRed => write!(f, "1"),
            Color::Green | Color::BrightGreen => write!(f, "2"),
            Color::Yellow | Color::BrightYellow => write!(f, "3"),
            Color::Blue | Color::BrightBlue => write!(f, "4"),
            Color::Magenta | Color::BrightMagenta => write!(f, "5"),
            Color::Cyan | Color::BrightCyan => write!(f, "6"),
            Color::White | Color::BrightWhite => write!(f, "7"),
            Color::Fixed(num) => write!(f, "8;5;{}", num),
            Color::RGB(r, g, b) => write!(f, "8;2;{};{};{}", r, g, b),
        }
    }
}

impl Default for Color {
    #[inline(always)]
    fn default() -> Self {
        Color::Unset
    }
}
