//BUFFER STRUCTURE
// Bits     Byte     What It Does

// 0-7      1        Code page 437 Representation Number (high ASCII or OEM font)
// 8-11     2        Foreground Colour
// 12-14    2        Backround Colour
// 15       2        Will the character blink

//COLOURS
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)] //allow copy semantics
#[repr(u8)] // all stored as u8
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
} //u4 would be fine but doesnt exist in rust

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] //making sure in memory its stored as a u8
struct ColourCode(u8);
impl ColourCode { //make colour num into a full colour byte
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}






//REPRESENT A FULL CHAR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] //function as a c struct, why the f*** am i using rust at this point
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
#[repr(transparent)]
struct Buffer { // vga buffer def
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}







//Write To The Screen
pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colour_code = self.colour_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    colour_code,
                };
                self.column_position += 1;
            }
        }
    }
    fn new_line(&mut self) {}
}
impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }

        }
    }
}





//testing funcs
pub fn print_something() {
    let mut writer = Writer {
        column_position: 30,
        colour_code: ColourCode::new(Colour::LightRed, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_string("Welcome To AlbaOS!");
}









