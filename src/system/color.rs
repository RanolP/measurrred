use usvg::Color;

pub trait ToWindowsColor {
    fn to_windows_color(&self) -> u32;
}

impl ToWindowsColor for Color {
    fn to_windows_color(&self) -> u32 {
        (self.red as u32) | ((self.green as u32) << 8) | ((self.blue as u32) << 16)
    }
}
