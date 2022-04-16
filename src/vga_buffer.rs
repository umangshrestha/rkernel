use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0x00,
    Blue = 0x01,
    Green = 0x02,
    Cyan = 0x03,
    Red = 0x04,
    Magenta = 0x05,
    Brown = 0x06,
    LightGray = 0x07,
    DarkGray = 0x08,
    LightBlue = 0x09,
    LightGreen = 0x0A,
    LightCyan = 0x0B,
    LightRed = 0x0C,
    Pink = 0x0D,
    Yellow = 0x0E,
    White = 0x0F,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);


impl ColorCode {
    pub fn new(foreground_color: Color, background_color: Color) -> Self {
        Self((background_color as u8) << 4 |
            (foreground_color as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii: u8,
    color: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


struct Writer {
    col_pos: usize,
    color_code: ColorCode,
    buf: &'static mut Buffer,
}


impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.col_pos;
                let color = self.color_code;
                
                self.buf.chars[row][col].write(ScreenChar {
                    ascii: byte,
                    color,
                });

                self.col_pos += 1;

            }
        }
    }

    fn new_line(&mut self) {}

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                //print ASCII
                0x20..=0x7e| b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}


pub fn hello_world() {
    let mut writer = Writer {
        col_pos : 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buf: unsafe {&mut *(0xb8000 as *mut Buffer)},
    };

    writer.write_string("Hello World");
}