use std::fmt;
use std::fmt::Formatter;

#[test]
fn test_struct() {
    println!("{}", "\x1B[31;43mHello\x1B[0m");
    println!("str = {}", "hello red yellow".red().on_yellow());
    println!("str = {}", "hello red yellow".on_yellow().red());
}

trait Colorize {
    const FG_RED: &'static str = "31";
    const BG_YELLOW: &'static str = "43";
    fn red(self) -> ColorString;
    fn on_yellow(self) -> ColorString;
}

#[derive(Default)]
struct ColorString {
    input: String,
    // 前景色
    fg_color: String,
    // 背景色
    bg_color: String,
}

impl ColorString {
    fn compute_style(&self) -> String {
        let mut res = "\x1B[".to_string();
        let mut has_wrote = false;
        if !self.bg_color.is_empty() {
            res.push_str(self.bg_color.as_str());
            has_wrote = true;
        }
        if !self.fg_color.is_empty() {
            if has_wrote {
                res.push(';');
            }
            res.push_str(self.fg_color.as_str());
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
        ColorString {
            fg_color: String::from(ColorString::FG_RED),
            ..self
        }
    }

    fn on_yellow(self) -> ColorString {
        ColorString {
            bg_color: String::from(ColorString::BG_YELLOW),
            ..self
        }
    }
}

impl<'a> Colorize for &'a str {
    fn red(self) -> ColorString {
        ColorString {
            fg_color: String::from(ColorString::FG_RED),
            input: String::from(self),
            ..ColorString::default()
        }
    }

    fn on_yellow(self) -> ColorString {
        ColorString {
            bg_color: String::from(ColorString::BG_YELLOW),
            input: String::from(self),
            ..ColorString::default()
        }
    }
}
