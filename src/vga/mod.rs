//! # VGA Driver for the Neotron Pico
//!
//! VGA output on the Neotron Pico uses 14 GPIO pins and two PIO state machines.
//!
//! It can generate 640x480@60Hz and 640x400@70Hz standard VGA video, with a
//! 25.2 MHz pixel clock. The spec is 25.175 MHz, so we are 0.1% off). The
//! assumption is that the CPU is clocked at 126 MHz, i.e. 5x the pixel
//! clock. All of the PIO code relies on this assumption!
//!
//! Currently only an 80x25 two-colour text-mode is supported. Other modes will be
//! added in the future.

// -----------------------------------------------------------------------------
// Licence Statement
// -----------------------------------------------------------------------------
// Copyright (c) Jonathan 'theJPster' Pallant and the Neotron Developers, 2021
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <https://www.gnu.org/licenses/>.
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Sub-modules
// -----------------------------------------------------------------------------

pub(crate) mod font;

// -----------------------------------------------------------------------------
// Imports
// -----------------------------------------------------------------------------

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use defmt::*;
use pico::hal::pio::PIOExt;

// -----------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------

/// Holds some data necessary to run the Video.
///
/// This structure is owned entirely by the main thread (or the drawing
/// thread). Data handled under interrupt is stored in various other places.
pub struct VideoEngine {
	/// How many frames have been drawn
	frame_count: u32,
	// How many ticks the CPU spent drawing, across the entire frame.
	running_ticks: u32,
	/// Look-up table mapping two 1-bpp pixels to two 12-bit RGB values (packed into one 32-bit word).
	///
	/// You can adjust this table to convert text to different colours.
	lookup: [RGBPair; 4],
}

#[repr(C, align(16))]
struct LineBuffer {
	length: u32,
	pixels: [RGBPair; NUM_PIXEL_PAIRS_PER_LINE],
}
/// Holds the four scan-line timing FIFO words we need for one scan-line.
///
/// See `make_timing` for a function which can generate these words. We DMA
/// them into the timing FIFO, so they must sit on a 16-byte boundary.
#[repr(C, align(16))]
struct ScanlineTimingBuffer {
	data: [u32; 4],
}

/// Holds the different kinds of scan-line timing buffers we need for various
/// portions of the screen.
#[repr(C, align(16))]
struct TimingBuffer {
	/// We use this when there are visible pixels on screen
	visible_line: ScanlineTimingBuffer,
	/// We use this during the v-sync front-porch and v-sync back-porch
	vblank_porch_buffer: ScanlineTimingBuffer,
	/// We use this during the v-sync sync pulse
	vblank_sync_buffer: ScanlineTimingBuffer,
}

/// Represents a 12-bit colour value.
///
/// Each channel has four-bits, and they are packed in `GBR` format. This is
/// so the PIO can shift them out right-first, and we have RED0 assigned to
/// the lowest GPIO pin.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RGBColour(u16);

impl RGBColour {
	pub const fn from_24bit(red: u8, green: u8, blue: u8) -> RGBColour {
		let red: u16 = (red as u16) & 0x00F;
		let green: u16 = (green as u16) & 0x00F;
		let blue: u16 = (blue as u16) & 0x00F;
		RGBColour((blue << 12) | (green << 4) | red)
	}
}

/// Represents two `RGBColour` pixels packed together.
///
/// The `first` pixel is packed in the lower 16-bits. This is because the PIO
/// shifts-right.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RGBPair(u32);

impl RGBPair {
	pub const fn from_pixels(first: RGBColour, second: RGBColour) -> RGBPair {
		let first: u32 = first.0 as u32;
		let second: u32 = second.0 as u32;
		RGBPair((second << 16) | first)
	}
}

// -----------------------------------------------------------------------------
// Static and Const Data
// -----------------------------------------------------------------------------

/// How many pixels per scan-line.
///
/// Adjust the pixel PIO program to run at the right speed to the screen is
/// filled. For example, if this is only 320 but you are aiming at 640x480,
/// make the pixel PIO take twice as long per pixel.
const NUM_PIXELS_PER_LINE: usize = 640;

/// How many pixel pairs we send out.
///
/// Each pixel is two 12-bit values packed into one 32-bit word(an `RGBPair`).
/// This is to make more efficient use of DMA and FIFO resources.
const NUM_PIXEL_PAIRS_PER_LINE: usize = NUM_PIXELS_PER_LINE / 2;

/// Number of lines on screen.
const NUM_LINES: usize = 400;

/// The highest number of columns in any text mode.
pub const NUM_TEXT_COLS: usize = NUM_PIXELS_PER_LINE / font::WIDTH_PX;

/// The highest number of rows in any text mode.
pub const NUM_TEXT_ROWS: usize = NUM_LINES as usize / font::HEIGHT_PX;

/// Scan-line on which the visible portion of the screen ends.
///
/// This value is for 400 line, 70 Hz mode. We assume the visible portion starts on line zero.
const V_VISIBLE_END: usize = V_FRONT_PORCH_START - 1;

/// Scan-line on which the front-porch starts.
///
/// This value is for 400 line, 70 Hz mode.
const V_FRONT_PORCH_START: usize = NUM_LINES;

/// Scan-line on which the front-porch ends.
///
/// This value is for 400 line, 70 Hz mode.
const V_FRONT_PORCH_END: usize = V_SYNC_START - 1;

/// Scan-line on which the sync-pulse starts.
///
/// This value is for 400 line, 70 Hz mode.
const V_SYNC_START: usize = 412;

/// Scan-line on which the sync pulse ends.
///
/// This value is for 400 line, 70 Hz mode.
const V_SYNC_END: usize = V_BACK_PORCH_START - 1;

/// Scan-line on which the back-porch starts.
///
/// This value is for 400 line, 70 Hz mode.
const V_BACK_PORCH_START: usize = 416;

/// Scan-line on which the back-porch ends.
///
/// This value is for 400 line, 70 Hz mode.
const V_BACK_PORCH_END: usize = 449;

/// Stores our timing data which we DMA into the timing PIO State Machine
static TIMING_BUFFER: TimingBuffer = TimingBuffer {
	// Note - the order of the arguments to `make_timing` is:
	//
	// * H-Sync Pulse (true = high, false = low)
	// * V-Sync Pulse (true = high, false = low)
	// * Generate pixel start IRQ (true or false)
	visible_line: ScanlineTimingBuffer {
		data: [
			// Front porch (as per the spec)
			make_timing(16 * 5, true, false, false),
			// Sync pulse (as per the spec)
			make_timing(96 * 5, false, false, false),
			// Back porch (shortened by two pixels because the
			// video starts two pixels late)
			make_timing(48 * 5, true, false, false),
			// Visible portion. It also triggers the IRQ to start pixels moving.
			make_timing(640 * 5, true, false, true),
		],
	},
	vblank_porch_buffer: ScanlineTimingBuffer {
		data: [
			// Front porch
			make_timing(16 * 5, true, false, false),
			// Sync pulse
			make_timing(96 * 5, false, false, false),
			// Back porch
			make_timing(48 * 5, true, false, false),
			// 'visible' portion (but it's blank)
			make_timing(640 * 5, true, false, false),
		],
	},
	vblank_sync_buffer: ScanlineTimingBuffer {
		data: [
			// Front porch
			make_timing(16 * 5, true, true, false),
			// Sync pulse
			make_timing(96 * 5, false, true, false),
			// Back porch
			make_timing(48 * 5, true, true, false),
			// Visible portion (but it's blank)
			make_timing(640 * 5, true, true, false),
		],
	},
};

/// Tracks which scan-line we are currently on (for timing purposes => it goes 0..=449)
static CURRENT_TIMING_LINE: AtomicUsize = AtomicUsize::new(0);

/// Tracks which scan-line we are currently on (for pixel purposes => it goes 0..NUM_LINES)
static CURRENT_DISPLAY_LINE: AtomicUsize = AtomicUsize::new(0);

/// Set to `true` when DMA of previous line is complete and next line is scheduled.
static DMA_READY: AtomicBool = AtomicBool::new(false);

/// Somewhere to stash the DMA controller object, so the IRQ can find it
static mut DMA_PERIPH: Option<super::pac::DMA> = None;

/// DMA channel for the timing FIFO
const TIMING_DMA_CHAN: usize = 0;

/// DMA channel for the pixel FIFO
const PIXEL_DMA_CHAN: usize = 1;

/// 12-bit pixels for the even scan-lines (0, 2, 4 ... NUM_LINES - 2). Defaults to black.
static mut PIXEL_DATA_BUFFER_EVEN: LineBuffer = LineBuffer {
	length: (NUM_PIXEL_PAIRS_PER_LINE as u32) - 1,
	pixels: [RGBPair::from_pixels(colours::WHITE, colours::BLACK); NUM_PIXEL_PAIRS_PER_LINE],
};

/// 12-bit pixels for the odd scan-lines (1, 3, 5 ... NUM_LINES-1). Defaults to white.
static mut PIXEL_DATA_BUFFER_ODD: LineBuffer = LineBuffer {
	length: (NUM_PIXEL_PAIRS_PER_LINE as u32) - 1,
	pixels: [RGBPair::from_pixels(colours::BLACK, colours::WHITE); NUM_PIXEL_PAIRS_PER_LINE],
};

/// Some dummy text to display.
///
/// This is arranged as `NUM_TEXT_ROWS` rows of `NUM_TEXT_COLS` columns. Each
/// item is an index into `font::FONT_DATA` (or a Code-Page 850 character).
static mut CHAR_ARRAY: [u8; NUM_TEXT_COLS as usize * NUM_TEXT_ROWS as usize] =
*b"\
\xDA\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xBF\
\xB3Neotron Pico Booting...                                                       \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3         \xDb\xDb\xDb\xBB\xB2\xB2\xDb\xDb\xBB\xDb\xDb\xDb\xDb\xDb\xDb\xDb\xBB\xB2\xDb\xDb\xDb\xDb\xDb\xBB\xB2\xDb\xDb\xDb\xDb\xDb\xDb\xDb\xDb\xBB\xDb\xDb\xDb\xDb\xDb\xDb\xBB\xB2\xB2\xDb\xDb\xDb\xDb\xDb\xBB\xB2\xDb\xDb\xDb\xBB\xB2\xB2\xDb\xDb\xBB          \xB3\
\xB3         \xDb\xDb\xDb\xDb\xBB\xB2\xDb\xDb\xBA\xDb\xDb\xC9\xCD\xCD\xCD\xCD\xBC\xDb\xDb\xC9\xCD\xCD\xDb\xDb\xBB\xC8\xCD\xCD\xDb\xDb\xC9\xCD\xCD\xBC\xDb\xDb\xC9\xCD\xCD\xDb\xDb\xBB\xDb\xDb\xC9\xCD\xCD\xDb\xDb\xBB\xDb\xDb\xDb\xDb\xBB\xB2\xDb\xDb\xBA          \xB3\
\xB3         \xDb\xDb\xC9\xDb\xDb\xBB\xDb\xDb\xBA\xDb\xDb\xDb\xDb\xDb\xBB\xB2\xB2\xDb\xDb\xBA\xB2\xB2\xDb\xDb\xBA\xB2\xB2\xB2\xDb\xDb\xBA\xB2\xB2\xB2\xDb\xDb\xDb\xDb\xDb\xDb\xC9\xBC\xDb\xDb\xBA\xB2\xB2\xDb\xDb\xBA\xDb\xDb\xC9\xDb\xDb\xBB\xDb\xDb\xBA          \xB3\
\xB3         \xDb\xDb\xBA\xC8\xDb\xDb\xDb\xDb\xBA\xDb\xDb\xC9\xCD\xCD\xBC\xB2\xB2\xDb\xDb\xBA\xB2\xB2\xDb\xDb\xBA\xB2\xB2\xB2\xDb\xDb\xBA\xB2\xB2\xB2\xDb\xDb\xC9\xCD\xCD\xDb\xDb\xBB\xDb\xDb\xBA\xB2\xB2\xDb\xDb\xBA\xDb\xDb\xBA\xC8\xDb\xDb\xDb\xDb\xBA          \xB3\
\xB3         \xDb\xDb\xBA\xB2\xC8\xDb\xDb\xDb\xBA\xDb\xDb\xDb\xDb\xDb\xDb\xDb\xBB\xC8\xDb\xDb\xDb\xDb\xDb\xC9\xBC\xB2\xB2\xB2\xDb\xDb\xBA\xB2\xB2\xB2\xDb\xDb\xBA\xB2\xB2\xDb\xDb\xBA\xC8\xDb\xDb\xDb\xDb\xDb\xC9\xBC\xDb\xDb\xBA\xB2\xC8\xDb\xDb\xDb\xBA          \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3                                                                              \xB3\
\xB3This is the end                                                               \xB3\
\xC0\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xC4\xD9";

/// A set of useful constants representing common RGB colours.
pub mod colours {
	/// The colour white
	pub const WHITE: super::RGBColour = super::RGBColour(0xFFF);

	/// The colour black
	pub const BLACK: super::RGBColour = super::RGBColour(0x000);

	/// The colour blue
	pub const BLUE: super::RGBColour = super::RGBColour(0xF00);

	/// The colour green
	pub const GREEN: super::RGBColour = super::RGBColour(0x0F0);

	/// The colour red
	pub const RED: super::RGBColour = super::RGBColour(0x00F);
}

// -----------------------------------------------------------------------------
// Functions
// -----------------------------------------------------------------------------

impl VideoEngine {
	// Initialise the main-thread resources
	pub fn new() -> VideoEngine {
		VideoEngine {
			frame_count: 0,
			running_ticks: 0,
			lookup: [
				RGBPair::from_pixels(colours::BLUE, colours::BLUE),
				RGBPair::from_pixels(colours::BLUE, colours::WHITE),
				RGBPair::from_pixels(colours::WHITE, colours::BLUE),
				RGBPair::from_pixels(colours::WHITE, colours::WHITE),
			],
		}
	}

	pub fn poll(&mut self) {
		if DMA_READY.load(Ordering::Relaxed) {
			let awake_at = cortex_m::peripheral::SYST::get_current();
			DMA_READY.store(false, Ordering::Relaxed);
			let current_line_num = CURRENT_DISPLAY_LINE.load(Ordering::Relaxed);
			if current_line_num == 0 {
				let cpu_percent = ((self.running_ticks * 1000 * 100) / 400) / 31_777;
				info!(
					"Frame {}, awake {} ticks ({}% load)",
					self.frame_count, self.running_ticks, cpu_percent
				);
				self.running_ticks = 0;
				self.frame_count += 1;
			}

			// new line - pick a buffer to draw into (not the one that is currently rendering!)
			let scan_line_buffer = unsafe {
				if (current_line_num & 1) == 0 {
					&mut PIXEL_DATA_BUFFER_ODD
				} else {
					&mut PIXEL_DATA_BUFFER_EVEN
				}
			};

			// Convert our position in scan-lines to a text row, and a line within each glyph on that row
			let text_row = (current_line_num / font::HEIGHT_PX) as usize;
			let font_row = (current_line_num % font::HEIGHT_PX) as usize;

			if text_row < NUM_TEXT_ROWS {
				// Note (unsafe): We could stash the char array inside `self`
				// but at some point we are going to need one CPU rendering
				// the text, and the other CPU running code and writing to
				// the buffer. This might be Undefined Behaviour, but
				// unfortunately real-time video is all about shared mutable
				// state. At least our platform is fixed, so we can simply
				// test if it works, for some given version of the Rust compiler.
				let row_slice = unsafe {
					&CHAR_ARRAY[(text_row * NUM_TEXT_COLS)..((text_row + 1) * NUM_TEXT_COLS)]
				};
				// Every font look-up we are about to do for this row will
				// involve offsetting by the row within each glyph. As this
				// is the same for every glyph on this row, we calculate a
				// new pointer once, in advance, and save ourselves an
				// addition each time around the loop.
				let font_ptr = unsafe { font::DATA.as_ptr().add(font_row) };

				// Get a pointer into our scan-line buffer
				let scan_line_buffer_ptr = scan_line_buffer.pixels.as_mut_ptr();
				let mut px_idx = 0;

				// Convert from characters to coloured pixels, using the font as a look-up table.
				for ch in row_slice.iter() {
					let index = (*ch as isize) * 16;
					// Note (unsafe): We use pointer arithmetic here because we
					// can't afford a bounds-check on an array. This is safe
					// because the font is `256 * width` bytes long and we can't
					// index more than `255 * width` bytes into it.
					let mono_pixels = unsafe { *font_ptr.offset(index) } as usize;
					// Convert from eight mono pixels in one byte to four RGB pairs
					unsafe {
						core::ptr::write_volatile(
							scan_line_buffer_ptr.offset(px_idx + 0),
							self.lookup[(mono_pixels >> 6) & 3],
						);
						core::ptr::write_volatile(
							scan_line_buffer_ptr.offset(px_idx + 1),
							self.lookup[(mono_pixels >> 4) & 3],
						);
						core::ptr::write_volatile(
							scan_line_buffer_ptr.offset(px_idx + 2),
							self.lookup[(mono_pixels >> 2) & 3],
						);
						core::ptr::write_volatile(
							scan_line_buffer_ptr.offset(px_idx + 3),
							self.lookup[(mono_pixels >> 0) & 3],
						);
					}
					px_idx += 4;
				}
			}
			let running_end = cortex_m::peripheral::SYST::get_current();
			self.running_ticks += (awake_at - running_end) & 0x00FF_FFFF;
		}
	}
}

/// Initialise all the static data and peripherals we need for our video display.
///
/// We need to keep `pio` and `dma` to run the video. We need `resets` to set
/// things up, so we only borrow that.
pub fn init(pio: super::pac::PIO0, dma: super::pac::DMA, resets: &mut super::pac::RESETS) {
	// Grab PIO0 and the state machines it contains
	let (mut pio, sm0, sm1, _sm2, _sm3) = pio.split(resets);

	// This program runs the timing loop. We post timing data (i.e. the length
	// of each period, along with what the H-Sync and V-Sync pins should do)
	// and it sets the GPIO pins and busy-waits the appropriate amount of
	// time. It also takes an extra 'instruction' which we can use to trigger
	// the appropriate interrupts.
	//
	// Post <value:32> where value: <clock_cycles:14> <hsync:1> <vsync:1>
	// <instruction:16>
	//
	// The SM will execute the instruction (typically either a NOP or an IRQ),
	// set the H-Sync and V-Sync pins as desired, then wait the given number
	// of clock cycles.
	//
	// Note: autopull should be set to 32-bits, OSR is set to shift right.
	let timing_program = pio_proc::pio!(
		32,
		"
		.wrap_target
		; Step 1. Push next 2 bits of OSR into `pins`, to set H-Sync and V-Sync
		out pins, 2
		; Step 2. Push last 14 bits of OSR into X for the timing loop.
		out x, 14
		; Step 3. Execute bottom 16-bits of OSR as an instruction. This take two cycles.
		out exec, 16
		loop0:
			; Spin until X is zero
			jmp x-- loop0
		.wrap
		"
	);

	// This is the video pixels program. It waits for an IRQ
	// (posted by the timing loop) then pulls pixel data from the FIFO. We post
	// the number of pixels for that line, then the pixel data.
	//
	// Post <num_pixels> <pixel1> <pixel2> ... <pixelN>; each <pixelX> maps to
	// the RGB output pins. On a Neotron Pico, there are 12 (4 Red, 4 Green and
	// 4 Blue) - so we set autopull to 12, and each value should be 12-bits long.
	//
	// Currently the FIFO supplies only the pixels, not the length value. When
	// we read the length from the FIFO as well, all hell breaks loose.
	//
	// Note autopull should be set to 32-bits, OSR is set to shift right.
	let pixel_program = pio_proc::pio!(
		32,
		"
		.wrap_target
		; Wait for timing state machine to start visible line
		wait 1 irq 0
		; Read the line length (in pixel-pairs)
		out x, 32
		loop1:
			; Write out first pixel - takes 5 clocks per pixel (double-width)
			out pins, 16 [4]
			; Write out second pixel - takes 5 clocks per pixel (double-width, allowing one for the jump)
			out pins, 16 [3]
			; Repeat until all pixel pairs sent
			jmp x-- loop1
		; Clear all pins after visible section
		mov pins null
		.wrap
		"
	);

	// These two state machines run thus:
	//
	// | Clock | Timing PIOSM | Pixel PIOSM      |
	// |:------|:-------------|:-----------------|
	// | 1     | out pins, 2  | wait 1 irq 0     |
	// | 2     | out x, 14    | wait 1 irq 0     |
	// | 3     | out exec, 16 | wait 1 irq 0     |
	// | 4     | <irq>        | wait 1 irq 0     |
	// | 5     | jmp x--      | wait 1 irq 0     |
	// | 6     |              | set x, 31        |
	// | 7     |              | out pins, 16 [4] |
	// | 8     |              | ..               |
	// | 9     |              | ..               |
	// | 10    |              | ..               |
	// | 11    |              | ..               |
	// | 12    |              | out pins, 16 [3] |
	// | 13    |              | ..               |
	// | 14    |              | ..               |
	// | 15    |              | ..               |
	// | 16    |              | jump x-- loop1   |
	//
	// Note: Credit to
	// https://gregchadwick.co.uk/blog/playing-with-the-pico-pt5/ who had a
	// very similar idea to me, but wrote it up far better than I ever could.

	let timing_installed = pio.install(&timing_program.program).unwrap();
	let (mut timing_sm, _, timing_fifo) =
		pico::hal::pio::PIOBuilder::from_program(timing_installed)
			.buffers(pico::hal::pio::Buffers::OnlyTx)
			.out_pins(0, 2)
			.autopull(true)
			.out_shift_direction(pico::hal::pio::ShiftDirection::Right)
			.pull_threshold(32)
			.build(sm0);
	timing_sm.set_pindirs([
		(0, pico::hal::pio::PinDir::Output),
		(1, pico::hal::pio::PinDir::Output),
	]);

	// Important notes!
	//
	// You must not set a clock_divider (other than 1.0) on the pixel state
	// machine. You might want the pixels to be twice as wide (or mode), but
	// enabling a clock divider adds a lot of jitter (i.e. the start each
	// each line differs by some number of 126 MHz clock cycles).

	let pixels_installed = pio.install(&pixel_program.program).unwrap();
	let (mut pixel_sm, _, pixel_fifo) = pico::hal::pio::PIOBuilder::from_program(pixels_installed)
		.buffers(pico::hal::pio::Buffers::OnlyTx)
		.out_pins(2, 12)
		.autopull(true)
		.out_shift_direction(pico::hal::pio::ShiftDirection::Right)
		.pull_threshold(32)
		.build(sm1);
	pixel_sm.set_pindirs((2..=13).map(|x| (x, pico::hal::pio::PinDir::Output)));

	// Read from the timing buffer and write to the timing FIFO. We get an
	// IRQ when the transfer is complete (i.e. when line has been fully
	// loaded).
	dma.ch[TIMING_DMA_CHAN].ch_ctrl_trig.write(|w| {
		w.data_size().size_word();
		w.incr_read().set_bit();
		w.incr_write().clear_bit();
		unsafe { w.treq_sel().bits(timing_fifo.dreq_value()) };
		unsafe { w.chain_to().bits(TIMING_DMA_CHAN as u8) };
		unsafe { w.ring_size().bits(0) };
		w.ring_sel().clear_bit();
		w.bswap().clear_bit();
		w.irq_quiet().clear_bit();
		w.en().set_bit();
		w.sniff_en().clear_bit();
		w
	});
	dma.ch[TIMING_DMA_CHAN]
		.ch_read_addr
		.write(|w| unsafe { w.bits(TIMING_BUFFER.visible_line.data.as_ptr() as usize as u32) });
	dma.ch[TIMING_DMA_CHAN]
		.ch_write_addr
		.write(|w| unsafe { w.bits(timing_fifo.fifo_address() as usize as u32) });
	dma.ch[TIMING_DMA_CHAN]
		.ch_trans_count
		.write(|w| unsafe { w.bits(TIMING_BUFFER.visible_line.data.len() as u32) });

	// Read from the pixel buffer (even first) and write to the pixel FIFO
	dma.ch[PIXEL_DMA_CHAN].ch_ctrl_trig.write(|w| {
		w.data_size().size_word();
		w.incr_read().set_bit();
		w.incr_write().clear_bit();
		unsafe { w.treq_sel().bits(pixel_fifo.dreq_value()) };
		unsafe { w.chain_to().bits(PIXEL_DMA_CHAN as u8) };
		unsafe { w.ring_size().bits(0) };
		w.ring_sel().clear_bit();
		w.bswap().clear_bit();
		w.irq_quiet().clear_bit();
		w.en().set_bit();
		w.sniff_en().clear_bit();
		w
	});
	dma.ch[PIXEL_DMA_CHAN]
		.ch_read_addr
		.write(|w| unsafe { w.bits(PIXEL_DATA_BUFFER_EVEN.as_ptr()) });
	dma.ch[PIXEL_DMA_CHAN]
		.ch_write_addr
		.write(|w| unsafe { w.bits(pixel_fifo.fifo_address() as usize as u32) });
	dma.ch[PIXEL_DMA_CHAN]
		.ch_trans_count
		.write(|w| unsafe { w.bits(PIXEL_DATA_BUFFER_EVEN.pixels.len() as u32 + 1) });
	dma.inte0.write(|w| unsafe {
		w.inte0()
			.bits((1 << PIXEL_DMA_CHAN) | (1 << TIMING_DMA_CHAN))
	});

	// Enable the DMA
	dma.multi_chan_trigger
		.write(|w| unsafe { w.bits((1 << PIXEL_DMA_CHAN) | (1 << TIMING_DMA_CHAN)) });

	info!("DMA enabled");

	unsafe {
		// Hand off the DMA peripheral to the interrupt
		DMA_PERIPH = Some(dma);

		// Enable the interrupts (DMA_PERIPH has to be set first)
		cortex_m::interrupt::enable();
		pico::hal::pac::NVIC::unpend(pico::hal::pac::Interrupt::DMA_IRQ_0);
		pico::hal::pac::NVIC::unmask(pico::hal::pac::Interrupt::DMA_IRQ_0);
	}

	info!("IRQs enabled");

	info!("DMA set-up complete");

	timing_sm.start();
	pixel_sm.start();

	info!("State Machines running");

	// We drop our state-machine and PIO objects here - this means the video
	// cannot be reconfigured at a later time, but they do keep on running
	// as-is.
}

/// Call this function whenever the DMA reports that it has completed a transfer.
///
/// We use this as a prompt to either start a transfer or more Timing words,
/// or a transfer or more pixel words.
pub unsafe fn irq() {
	let dma: &mut super::pac::DMA = match DMA_PERIPH.as_mut() {
		Some(dma) => dma,
		None => {
			return;
		}
	};
	let status = dma.ints0.read().bits();

	// Check if this is a DMA interrupt for the sync DMA channel
	let timing_dma_chan_irq = (status & (1 << TIMING_DMA_CHAN)) != 0;

	// Check if this is a DMA interrupt for the line DMA channel
	let pixel_dma_chan_irq = (status & (1 << PIXEL_DMA_CHAN)) != 0;

	if timing_dma_chan_irq {
		// clear timing_dma_chan bit in DMA interrupt bitfield
		dma.ints0.write(|w| w.bits(1 << TIMING_DMA_CHAN));

		let old_timing_line = CURRENT_TIMING_LINE.load(Ordering::Relaxed);
		let next_timing_line = if old_timing_line == V_BACK_PORCH_END {
			// Wrap around
			0
		} else {
			// Keep going
			old_timing_line + 1
		};
		CURRENT_TIMING_LINE.store(next_timing_line, Ordering::Relaxed);

		let buffer = if next_timing_line <= V_VISIBLE_END {
			// Visible lines
			&TIMING_BUFFER.visible_line
		} else if next_timing_line <= V_FRONT_PORCH_END {
			// VGA front porch before VGA sync pulse
			&TIMING_BUFFER.vblank_porch_buffer
		} else if next_timing_line <= V_SYNC_END {
			// Sync pulse
			&TIMING_BUFFER.vblank_sync_buffer
		} else {
			// VGA back porch following VGA sync pulse
			&TIMING_BUFFER.vblank_porch_buffer
		};
		dma.ch[TIMING_DMA_CHAN]
			.ch_al3_read_addr_trig
			.write(|w| w.bits(buffer as *const _ as usize as u32))
	}

	if pixel_dma_chan_irq {
		dma.ints0.write(|w| w.bits(1 << PIXEL_DMA_CHAN));

		// A pixel DMA transfer is now complete. This only fires on visible lines.

		let mut next_display_line = CURRENT_DISPLAY_LINE.load(Ordering::Relaxed) + 1;
		if next_display_line > V_VISIBLE_END {
			next_display_line = 0;
		};

		// Set the DMA load address according to which line we are on. We use
		// the 'trigger' alias to restart the DMA at the same time as we
		// write the new read address. The DMA had stopped because the
		// previous line was transferred completely.
		if (next_display_line & 1) == 1 {
			// Odd visible line is next
			dma.ch[PIXEL_DMA_CHAN]
				.ch_al3_read_addr_trig
				.write(|w| w.bits(PIXEL_DATA_BUFFER_ODD.as_ptr()))
		} else {
			// Even visible line is next
			dma.ch[PIXEL_DMA_CHAN]
				.ch_al3_read_addr_trig
				.write(|w| w.bits(PIXEL_DATA_BUFFER_EVEN.as_ptr()))
		}

		CURRENT_DISPLAY_LINE.store(next_display_line, Ordering::Relaxed);
		DMA_READY.store(true, Ordering::Relaxed);
	}
}

/// Generate a 32-bit value we can send to the Timing FIFO.
///
/// * `period` - The length of this portion of the scan-line, in system clock ticks
/// * `hsync` - true if the H-Sync pin should be high during this period, else false
/// * `vsync` - true if the H-Sync pin should be high during this period, else false
/// * `raise_irq` - true the timing statemachine should raise an IRQ at the start of this period
///
/// Returns a 32-bit value you can post to the Timing FIFO.
const fn make_timing(period: u32, hsync: bool, vsync: bool, raise_irq: bool) -> u32 {
	let command = if raise_irq {
		// This command sets IRQ 0. It is the same as:
		//
		// ```
		// pio::InstructionOperands::IRQ {
		// 	clear: false,
		// 	wait: false,
		// 	index: 0,
		// 	relative: false,
		// }.encode()
		// ```
		//
		// Unfortunately encoding this isn't a const-fn, so we cheat:
		0xc000
	} else {
		// This command is a no-op (it moves Y into Y)
		//
		// ```
		// pio::InstructionOperands::MOV {
		// 	destination: pio::MovDestination::Y,
		// 	op: pio::MovOperation::None,
		// 	source: pio::MovSource::Y,
		// }.encode()
		// ```
		//
		// Unfortunately encoding this isn't a const-fn, so we cheat:
		0xa042
	};
	let mut value: u32 = 0;
	if hsync {
		value |= 1 << 0;
	}
	if vsync {
		value |= 1 << 1;
	}
	value |= (period - 6) << 2;
	value | command << 16
}

impl LineBuffer {
	/// Convert the line buffer to a 32-bit address that the DMA engine understands.
	fn as_ptr(&self) -> u32 {
		self as *const _ as usize as u32
	}
}

// -----------------------------------------------------------------------------
// End of file
// -----------------------------------------------------------------------------
