use core::fmt::{self, Write};

const VGA_BUFFER_ADDR: usize = 0xb8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

#[repr(C)]
#[derive(Clone, Copy)]
struct VgaChar {
    character: u8,
    color: u8,
}
impl VgaChar {
    const fn new(character: u8, color: u8) -> Self {
        Self { character, color }
    }
}

pub struct VgaWriter {
    buffer: &'static mut [[VgaChar; VGA_WIDTH]; VGA_HEIGHT],
    current_row: usize,
    current_col: usize,
    color: u8,
}

impl VgaWriter {
    unsafe fn new() -> Self {
        Self {
            buffer: unsafe { &mut *(VGA_BUFFER_ADDR as *mut [[VgaChar; VGA_WIDTH]; VGA_HEIGHT]) },
            current_row: 0,
            current_col: 0,
            color: 0x0f,
        }
    }

    pub fn draw_table(
        &mut self,
        table: &Table,
        errors: Vec<String>,
        winner: Option<(Player, Win)>,
    ) {
        self.clear();

        self.write_string("Tic Tac Toe\n\n");

        let grid_start_row = self.current_row;

        for grid_row in 0..3 {
            self.write_string("     |     |     \n");

            for grid_col in 0..3 {
                let index = grid_row * 3 + grid_col;
                let ch = match table.state[index] {
                    Some(Player::X) => {
                        self.set_color(0x0B); // light blue
                        b'X'
                    }
                    Some(Player::O) => {
                        self.set_color(0x0E); // yellow
                        b'O'
                    }
                    None => {
                        self.set_color(0x08); // dark gray
                        b'1' + index as u8
                    }
                };

                self.write_string("  ");
                self.write_byte(ch);
                self.write_string("  ");
                self.set_color(0x0f); // reset color

                if grid_col < 2 {
                    self.write_byte(b'|');
                }
            }
            self.write_byte(b'\n');

            self.write_string("     |     |     \n");

            if grid_row < 2 {
                self.write_string(" ----+-----+---- \n");
            }
        }

        if let Some((player, win)) = winner {
            self.draw_strikethrough(grid_start_row, &win, player);
        }

        for error in errors {
            self.set_color(0x0C); // light red
            self.write_string(&format!("\nError: {}\n", error));
            self.set_color(0x0f); // reset color
        }

        if let Some((player, _)) = winner {
            self.set_color(0x0A); // light green
            match player {
                Player::X => self.write_string("\nPlayer X wins!\n"),
                Player::O => self.write_string("\nPlayer O wins!\n"),
            }
            self.set_color(0x0f); // reset color
        }
    }

    fn draw_strikethrough(&mut self, grid_start_row: usize, win: &Win, player: Player) {
        let Win(pos1, pos2, pos3) = *win;

        let get_cell_center = |pos: usize| -> (usize, usize) {
            let grid_row = pos / 3;
            let grid_col = pos % 3;
            let row = grid_start_row + grid_row * 4 + 1;
            let col = grid_col * 6 + 2;
            (row, col)
        };

        let (row1, col1) = get_cell_center(pos1);
        let (row2, col2) = get_cell_center(pos2);
        let (row3, col3) = get_cell_center(pos3);

        let strikethrough_color = match player {
            Player::X => 0x0C, // light red
            Player::O => 0x0D, // light purple
        };

        // horizontal wins
        if row1 == row2 && row2 == row3 {
            let row = row1;
            for col in col1..=col3 {
                if col < VGA_WIDTH
                    && row < VGA_HEIGHT
                    && (col != col1 && col != col2 && col != col3)
                {
                    self.buffer[row][col] = VgaChar::new(b'-', strikethrough_color);
                }
            }
        }
        // vertical wins
        else if col1 == col2 && col2 == col3 {
            let col = col1;
            for row in row1..=row3 {
                if (col < VGA_WIDTH && row < VGA_HEIGHT)
                    && (row != row1 && row != row2 && row != row3)
                {
                    self.buffer[row][col] = VgaChar::new(b'|', strikethrough_color);
                }
            }
        // diagonal wins (ugly, i know)
        } else {
            let positions = [pos1, pos2, pos3];
            let mut sorted_positions = positions;
            sorted_positions.sort();

            if sorted_positions == [0, 4, 8] {
                self.buffer[grid_start_row + 2][4] = VgaChar::new(b'\\', strikethrough_color);
                self.buffer[grid_start_row + 3][5] = VgaChar::new(b'\\', strikethrough_color);
                self.buffer[grid_start_row + 4][6] = VgaChar::new(b'\\', strikethrough_color);

                self.buffer[grid_start_row + 6][10] = VgaChar::new(b'\\', strikethrough_color);
                self.buffer[grid_start_row + 7][11] = VgaChar::new(b'\\', strikethrough_color);
                self.buffer[grid_start_row + 8][12] = VgaChar::new(b'\\', strikethrough_color);
            } else if sorted_positions == [2, 4, 6] {
                self.buffer[grid_start_row + 2][12] = VgaChar::new(b'/', strikethrough_color);
                self.buffer[grid_start_row + 3][11] = VgaChar::new(b'/', strikethrough_color);
                self.buffer[grid_start_row + 4][10] = VgaChar::new(b'/', strikethrough_color);

                self.buffer[grid_start_row + 6][6] = VgaChar::new(b'/', strikethrough_color);
                self.buffer[grid_start_row + 7][5] = VgaChar::new(b'/', strikethrough_color);
                self.buffer[grid_start_row + 8][4] = VgaChar::new(b'/', strikethrough_color);
            }
        }
    }

    pub fn clear(&mut self) {
        let blank = VgaChar::new(b' ', self.color);
        for row in self.buffer.iter_mut() {
            row.fill(blank);
        }
        self.current_row = 0;
        self.current_col = 0;
    }

    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.current_col >= VGA_WIDTH {
                    self.new_line();
                }

                if self.current_row < VGA_HEIGHT {
                    self.buffer[self.current_row][self.current_col] = VgaChar {
                        character: byte,
                        color: self.color,
                    };
                    self.current_col += 1;
                }
            }
        }
    }

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.current_col = 0;
        if self.current_row < VGA_HEIGHT - 1 {
            self.current_row += 1;
        } else {
            // Scroll up
            for row in 1..VGA_HEIGHT {
                self.buffer[row - 1] = self.buffer[row];
            }
            let blank = VgaChar::new(b' ', self.color);
            self.buffer[VGA_HEIGHT - 1].fill(blank);
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<VgaWriter> = Mutex::new(unsafe { VgaWriter::new() });
}

pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

macro_rules! print {
    ($($arg:tt)*) => {
        crate::vga::_print(format_args!($($arg)*))
    };
}

macro_rules! println {
    () => (crate::vga::print!("\n"));
    ($($arg:tt)*) => (crate::vga::print!("{}\n", format_args!($($arg)*)));
}

use alloc::{format, string::String, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::game::{
    event::Player,
    table::{Table, Win},
};

pub(crate) use {print, println};
