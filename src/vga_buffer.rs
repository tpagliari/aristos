use volatile::Volatile;
use core::{fmt, usize};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // each enum is stored as u8
pub enum Color {
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
}

/// The colors (bg+fg) with which a char appears on the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // ensure Struct has the same memory layout as its single field.
struct ColorCode(u8);

impl ColorCode {
    /** Create a new ColorCode by combining foreground and background colors.
      - bg color is shifted left by 4 bits to make space for the fg color,
        so that in one unique u8 we have the leftmost 4 bits that are bg and the rightmost 4 bits are fg.
    */
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// To have a char on the screen we need an ascii character and a ColorCode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    char_ascii : u8,
    color_code : ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// The buffer is a matrix nxm where each element is a screen char;
/// its memory representation is compatible with the VGA area,
/// so that the VGA buffer is of type Buffer.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/** A writer struct to handle writing to the screen buffer:
 - `column_position`: keeps track of the current column position where the next character will be written
 - `color_code`: stores the current color code to be used
 - `buffer`: mutable reference to screen buffer, allowing the writer to modify the buffer
 */ 
pub struct Writer {
    column_position : usize,
    color_code : ColorCode,
    buffer : &'static mut Buffer // 'static means that the reference is valid for the entire program lifetime
}

impl Writer {

    fn new_line(&mut self) {
        for r in 1..BUFFER_HEIGHT {
            for c in 0..BUFFER_WIDTH {
                let char: ScreenChar = self.buffer.chars[r][c].read();
                self.buffer.chars[r-1][c].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1); // only the bottom row needs to be cleared, the others are overwritten
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let cancel: ScreenChar = ScreenChar {
            char_ascii : b' ',
            color_code : self.color_code
        };
        for c in 0..BUFFER_WIDTH {
            self.buffer.chars[row][c].write(cancel);
        } 
    }

    pub fn write_a_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                let row: usize = BUFFER_HEIGHT - 1;
                let col: usize = self.column_position;

                if col >= BUFFER_WIDTH { self.new_line(); }

                self.buffer.chars[row][col].write(
                    ScreenChar { char_ascii: byte, color_code: self.color_code }
                );
                
                self.column_position += 1;
            }
        }
    }

    pub fn write_a_str(&mut self, xs: &str) {
        for x in xs.as_bytes() {
            match x {
                0x20..=0x7e | b'\n' => self.write_a_byte(*x), // all printable ASCII byte or newline
                _ => self.write_a_byte(0xfe), // whatever non ascii becomes a square 
            }
        }
    }
}

/// Write trait for the Writer struct
impl fmt::Write for Writer {
    // Required method
    fn write_str(&mut self, xs: &str) -> fmt::Result {
        self.write_a_str(xs);
        Ok(())
    }
}

/// temporary to try it out
pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    write!(writer, "Welcome to AristOS,\nan os from the ancient Greece.\n\n").unwrap();
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}