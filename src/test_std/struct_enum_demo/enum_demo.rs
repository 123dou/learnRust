use std::fmt;
use std::fmt::Formatter;
use std::option::Option::Some;
use std::str::FromStr;

#[test]
fn test_struct() {
    println!("{}", "\x1B[31;43mHello\x1B[0m");
    println!("str = {}", "hello red yellow".red().on_yellow());
    println!("str = {}", "hello red yellow".on_red().blue());
}

trait Colorize {
    fn red(self) -> ColorString;
    fn yellow(self) -> ColorString;
    fn blue(self) -> ColorString;
    fn color<S: Into<Color>>(self, color: S) -> ColorString;
    fn on_red(self) -> ColorString;
    fn on_yellow(self) -> ColorString;
    fn on_blue(self) -> ColorString;
    fn on_color<S: Into<Color>>(self, color: S) -> ColorString;
}

#[derive(Default)]
struct ColorString {
    input: String,
    // 前景色
    fg_color: Option<Color>,
    // 背景色
    bg_color: Option<Color>,
}

enum Color {
    Red,
    Yellow,
    Blue,
}

impl<'a> From<&'a str> for Color {
    fn from(src: &'a str) -> Self {
        src.parse().unwrap_or(Color::Red)
    }
}

impl From<String> for Color {
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Color::Red)
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let src = src.to_lowercase();
        match src.as_ref() {
            "red" => Ok(Color::Red),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

impl Color {
    fn to_fg_str(&self) -> &str {
        match *self {
            Color::Red => "31",
            Color::Blue => "34",
            Color::Yellow => "33",
        }
    }

    fn to_bg_str(&self) -> &str {
        match *self {
            Color::Red => "41",
            Color::Yellow => "43",
            Color::Blue => "44",
        }
    }
}

impl ColorString {
    fn compute_style(&self) -> String {
        let mut res = "\x1B[".to_string();
        let mut has_wrote = false;
        if let Some(ref bg_color) = self.bg_color {
            res.push_str(bg_color.to_bg_str());
            has_wrote = true;
        }
        if let Some(ref fg_color) = self.fg_color {
            if has_wrote {
                res.push(';');
            }
            res.push_str(fg_color.to_fg_str());
        }
        res.push('m');
        res
    }
}

impl fmt::Display for ColorString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let input = &self.input.clone();
        f.write_str(&self.compute_style())?;
        f.write_str(input)?;
        f.write_str("\x1B[0m")?;
        Ok(())
    }
}

impl<'a> Colorize for ColorString {
    fn red(self) -> ColorString {
        self.color(Color::Red)
    }

    fn yellow(self) -> ColorString {
        self.color(Color::Yellow)
    }

    fn blue(self) -> ColorString {
        self.color(Color::Blue)
    }

    fn color<S: Into<Color>>(self, color: S) -> ColorString {
        ColorString {
            fg_color: Some(color.into()),
            ..self
        }
    }

    fn on_red(self) -> ColorString {
        self.on_color(Color::Red)
    }

    fn on_yellow(self) -> ColorString {
        self.on_color(Color::Yellow)
    }

    fn on_blue(self) -> ColorString {
        self.on_color(Color::Blue)
    }

    fn on_color<S: Into<Color>>(self, color: S) -> ColorString {
        ColorString {
            bg_color: Some(color.into()),
            ..self
        }
    }
}

impl<'a> Colorize for &'a str {
    fn red(self) -> ColorString {
        self.color(Color::Red)
    }

    fn yellow(self) -> ColorString {
        self.color(Color::Yellow)
    }

    fn blue(self) -> ColorString {
        self.color(Color::Blue)
    }

    fn color<S: Into<Color>>(self, color: S) -> ColorString {
        ColorString {
            fg_color: Some(color.into()),
            input: self.to_string(),
            ..ColorString::default()
        }
    }

    fn on_red(self) -> ColorString {
        self.on_color(Color::Red)
    }

    fn on_yellow(self) -> ColorString {
        self.on_color(Color::Yellow)
    }

    fn on_blue(self) -> ColorString {
        self.on_color(Color::Blue)
    }

    fn on_color<S: Into<Color>>(self, color: S) -> ColorString {
        ColorString {
            bg_color: Some(color.into()),
            input: self.to_string(),
            ..ColorString::default()
        }
    }
}
