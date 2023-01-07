//! # Neotron Pico Console Font
//!
//! This is the cp850-8x8 font from FreeBSD. See
//! <http://web.mit.edu/freebsd/head/share/syscons/fonts/cp850-8x8.fnt>
//!
//! Use `uudecode` to decode the font file, then `xxd` to convert it into binary.
//!
//! The compilation of software known as FreeBSD is distributed under the
//! following terms:
//!
//! Copyright (c) 1992-2014 The FreeBSD Project. All rights reserved.
//!
//! Redistribution and use in source and binary forms, with or without
//! modification, are permitted provided that the following conditions
//! are met:
//! 1. Redistributions of source code must retain the above copyright
//!    notice, this list of conditions and the following disclaimer.
//! 2. Redistributions in binary form must reproduce the above copyright
//!    notice, this list of conditions and the following disclaimer in the
//!    documentation and/or other materials provided with the distribution.
//!
//! THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
//! ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//! IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
//! ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE
//! FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//! DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
//! OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
//! HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
//! LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
//! OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
//! SUCH DAMAGE.

/// An 8x16 font
pub static FONT: super::Font = super::Font {
	height: 8,
	data: &DATA,
};

/// Our font data - arranged as 256 glyphs of 1 byte/row x 8 row/glyph.
#[link_section = ".data"]
static DATA: [u8; 256 * 8] = [
	// Char::Null
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b1000_0001,
	0b1010_0101,
	0b1000_0001,
	0b1011_1101,
	0b1001_1001,
	0b1000_0001,
	0b0111_1110,
	0b0111_1110,
	0b1111_1111,
	0b1101_1011,
	0b1111_1111,
	0b1100_0011,
	0b1110_0111,
	0b1111_1111,
	0b0111_1110,
	0b0110_1100,
	0b1111_1110,
	0b1111_1110,
	0b1111_1110,
	0b0111_1100,
	0b0011_1000,
	0b0001_0000,
	0b0000_0000,
	0b0001_0000,
	0b0011_1000,
	0b0111_1100,
	0b1111_1110,
	0b0111_1100,
	0b0011_1000,
	0b0001_0000,
	0b0000_0000,
	0b0011_1000,
	0b0111_1100,
	0b0011_1000,
	0b1111_1110,
	0b1111_1110,
	0b1101_0110,
	0b0001_0000,
	0b0011_1000,
	0b0001_0000,
	0b0011_1000,
	0b0111_1100,
	0b1111_1110,
	0b1111_1110,
	0b0111_1100,
	0b0001_0000,
	0b0011_1000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0011_1100,
	0b0011_1100,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1111,
	0b1111_1111,
	0b1110_0111,
	0b1100_0011,
	0b1100_0011,
	0b1110_0111,
	0b1111_1111,
	0b1111_1111,
	0b0000_0000,
	0b0011_1100,
	0b0110_0110,
	0b0100_0010,
	0b0100_0010,
	0b0110_0110,
	0b0011_1100,
	0b0000_0000,
	0b1111_1111,
	0b1100_0011,
	0b1001_1001,
	0b1011_1101,
	0b1011_1101,
	0b1001_1001,
	0b1100_0011,
	0b1111_1111,
	0b0000_1111,
	0b0000_0111,
	0b0000_1111,
	0b0111_1101,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b0111_1000,
	0b0011_1100,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0011_1100,
	0b0001_1000,
	0b0111_1110,
	0b0001_1000,
	0b0011_1111,
	0b0011_0011,
	0b0011_1111,
	0b0011_0000,
	0b0011_0000,
	0b0111_0000,
	0b1111_0000,
	0b1110_0000,
	0b0111_1111,
	0b0110_0011,
	0b0111_1111,
	0b0110_0011,
	0b0110_0011,
	0b0110_0111,
	0b1110_0110,
	0b1100_0000,
	0b0001_1000,
	0b1101_1011,
	0b0011_1100,
	0b1110_0111,
	0b1110_0111,
	0b0011_1100,
	0b1101_1011,
	0b0001_1000,
	0b1000_0000,
	0b1110_0000,
	0b1111_1000,
	0b1111_1110,
	0b1111_1000,
	0b1110_0000,
	0b1000_0000,
	0b0000_0000,
	0b0000_0010,
	0b0000_1110,
	0b0011_1110,
	0b1111_1110,
	0b0011_1110,
	0b0000_1110,
	0b0000_0010,
	0b0000_0000,
	0b0001_1000,
	0b0011_1100,
	0b0111_1110,
	0b0001_1000,
	0b0001_1000,
	0b0111_1110,
	0b0011_1100,
	0b0001_1000,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0000_0000,
	0b0110_0110,
	0b0000_0000,
	0b0111_1111,
	0b1101_1011,
	0b1101_1011,
	0b0111_1011,
	0b0001_1011,
	0b0001_1011,
	0b0001_1011,
	0b0000_0000,
	0b0011_1110,
	0b0110_0001,
	0b0011_1100,
	0b0110_0110,
	0b0110_0110,
	0b0011_1100,
	0b1000_0110,
	0b0111_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b0111_1110,
	0b0111_1110,
	0b0000_0000,
	0b0001_1000,
	0b0011_1100,
	0b0111_1110,
	0b0001_1000,
	0b0111_1110,
	0b0011_1100,
	0b0001_1000,
	0b1111_1111,
	0b0001_1000,
	0b0011_1100,
	0b0111_1110,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0111_1110,
	0b0011_1100,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0000_1100,
	0b1111_1110,
	0b0000_1100,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0011_0000,
	0b0110_0000,
	0b1111_1110,
	0b0110_0000,
	0b0011_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1100_0000,
	0b1100_0000,
	0b1100_0000,
	0b1111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0010_0100,
	0b0110_0110,
	0b1111_1111,
	0b0110_0110,
	0b0010_0100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0011_1100,
	0b0111_1110,
	0b1111_1111,
	0b1111_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1111,
	0b1111_1111,
	0b0111_1110,
	0b0011_1100,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0011_1100,
	0b0011_1100,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0001_1000,
	0b0000_0000,
	0b0110_0110,
	0b0110_0110,
	0b0010_0100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0110_1100,
	0b0110_1100,
	0b1111_1110,
	0b0110_1100,
	0b1111_1110,
	0b0110_1100,
	0b0110_1100,
	0b0000_0000,
	0b0001_1000,
	0b0011_1110,
	0b0110_0000,
	0b0011_1100,
	0b0000_0110,
	0b0111_1100,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b1100_0110,
	0b1100_1100,
	0b0001_1000,
	0b0011_0000,
	0b0110_0110,
	0b1100_0110,
	0b0000_0000,
	0b0011_1000,
	0b0110_1100,
	0b0011_1000,
	0b0111_0110,
	0b1101_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0011_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_1100,
	0b0001_1000,
	0b0011_0000,
	0b0011_0000,
	0b0011_0000,
	0b0001_1000,
	0b0000_1100,
	0b0000_0000,
	0b0011_0000,
	0b0001_1000,
	0b0000_1100,
	0b0000_1100,
	0b0000_1100,
	0b0001_1000,
	0b0011_0000,
	0b0000_0000,
	0b0000_0000,
	0b0110_0110,
	0b0011_1100,
	0b1111_1111,
	0b0011_1100,
	0b0110_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0111_1110,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0011_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0110,
	0b0000_1100,
	0b0001_1000,
	0b0011_0000,
	0b0110_0000,
	0b1100_0000,
	0b1000_0000,
	0b0000_0000,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1101_0110,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b0001_1000,
	0b0011_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0111_1110,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b0000_0110,
	0b0001_1100,
	0b0011_0000,
	0b0110_0110,
	0b1111_1110,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b0000_0110,
	0b0011_1100,
	0b0000_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0001_1100,
	0b0011_1100,
	0b0110_1100,
	0b1100_1100,
	0b1111_1110,
	0b0000_1100,
	0b0001_1110,
	0b0000_0000,
	0b1111_1110,
	0b1100_0000,
	0b1100_0000,
	0b1111_1100,
	0b0000_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0011_1000,
	0b0110_0000,
	0b1100_0000,
	0b1111_1100,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b1111_1110,
	0b1100_0110,
	0b0000_1100,
	0b0001_1000,
	0b0011_0000,
	0b0011_0000,
	0b0011_0000,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b0111_1110,
	0b0000_0110,
	0b0000_1100,
	0b0111_1000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0011_0000,
	0b0000_0110,
	0b0000_1100,
	0b0001_1000,
	0b0011_0000,
	0b0001_1000,
	0b0000_1100,
	0b0000_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0110_0000,
	0b0011_0000,
	0b0001_1000,
	0b0000_1100,
	0b0001_1000,
	0b0011_0000,
	0b0110_0000,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b0000_1100,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0001_1000,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1101_1110,
	0b1101_1110,
	0b1101_1110,
	0b1100_0000,
	0b0111_1000,
	0b0000_0000,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0000_0000,
	0b1111_1100,
	0b0110_0110,
	0b0110_0110,
	0b0111_1100,
	0b0110_0110,
	0b0110_0110,
	0b1111_1100,
	0b0000_0000,
	0b0011_1100,
	0b0110_0110,
	0b1100_0000,
	0b1100_0000,
	0b1100_0000,
	0b0110_0110,
	0b0011_1100,
	0b0000_0000,
	0b1111_1000,
	0b0110_1100,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0110_1100,
	0b1111_1000,
	0b0000_0000,
	0b1111_1110,
	0b0110_0010,
	0b0110_1000,
	0b0111_1000,
	0b0110_1000,
	0b0110_0010,
	0b1111_1110,
	0b0000_0000,
	0b1111_1110,
	0b0110_0010,
	0b0110_1000,
	0b0111_1000,
	0b0110_1000,
	0b0110_0000,
	0b1111_0000,
	0b0000_0000,
	0b0011_1100,
	0b0110_0110,
	0b1100_0000,
	0b1100_0000,
	0b1100_1110,
	0b0110_0110,
	0b0011_1010,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1111_1110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0000_0000,
	0b0011_1100,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0001_1110,
	0b0000_1100,
	0b0000_1100,
	0b0000_1100,
	0b1100_1100,
	0b1100_1100,
	0b0111_1000,
	0b0000_0000,
	0b1110_0110,
	0b0110_0110,
	0b0110_1100,
	0b0111_1000,
	0b0110_1100,
	0b0110_0110,
	0b1110_0110,
	0b0000_0000,
	0b1111_0000,
	0b0110_0000,
	0b0110_0000,
	0b0110_0000,
	0b0110_0010,
	0b0110_0110,
	0b1111_1110,
	0b0000_0000,
	0b1100_0110,
	0b1110_1110,
	0b1111_1110,
	0b1111_1110,
	0b1101_0110,
	0b1100_0110,
	0b1100_0110,
	0b0000_0000,
	0b1100_0110,
	0b1110_0110,
	0b1111_0110,
	0b1101_1110,
	0b1100_1110,
	0b1100_0110,
	0b1100_0110,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b1111_1100,
	0b0110_0110,
	0b0110_0110,
	0b0111_1100,
	0b0110_0000,
	0b0110_0000,
	0b1111_0000,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_1110,
	0b0111_1100,
	0b0000_1110,
	0b1111_1100,
	0b0110_0110,
	0b0110_0110,
	0b0111_1100,
	0b0110_1100,
	0b0110_0110,
	0b1110_0110,
	0b0000_0000,
	0b0011_1100,
	0b0110_0110,
	0b0011_0000,
	0b0001_1000,
	0b0000_1100,
	0b0110_0110,
	0b0011_1100,
	0b0000_0000,
	0b0111_1110,
	0b0111_1110,
	0b0101_1010,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1101_0110,
	0b1101_0110,
	0b1111_1110,
	0b0110_1100,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1100_0110,
	0b0000_0000,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0011_1100,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b1111_1110,
	0b1100_0110,
	0b1000_1100,
	0b0001_1000,
	0b0011_0010,
	0b0110_0110,
	0b1111_1110,
	0b0000_0000,
	0b0011_1100,
	0b0011_0000,
	0b0011_0000,
	0b0011_0000,
	0b0011_0000,
	0b0011_0000,
	0b0011_1100,
	0b0000_0000,
	0b1100_0000,
	0b0110_0000,
	0b0011_0000,
	0b0001_1000,
	0b0000_1100,
	0b0000_0110,
	0b0000_0010,
	0b0000_0000,
	0b0011_1100,
	0b0000_1100,
	0b0000_1100,
	0b0000_1100,
	0b0000_1100,
	0b0000_1100,
	0b0011_1100,
	0b0000_0000,
	0b0001_0000,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1111,
	0b0011_0000,
	0b0001_1000,
	0b0000_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1000,
	0b0000_1100,
	0b0111_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b1110_0000,
	0b0110_0000,
	0b0111_1100,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b1101_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0000,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0001_1100,
	0b0000_1100,
	0b0111_1100,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0000,
	0b0111_1100,
	0b0000_0000,
	0b0011_1100,
	0b0110_0110,
	0b0110_0000,
	0b1111_1000,
	0b0110_0000,
	0b0110_0000,
	0b1111_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_0110,
	0b1100_1100,
	0b1100_1100,
	0b0111_1100,
	0b0000_1100,
	0b1111_1000,
	0b1110_0000,
	0b0110_0000,
	0b0110_1100,
	0b0111_0110,
	0b0110_0110,
	0b0110_0110,
	0b1110_0110,
	0b0000_0000,
	0b0001_1000,
	0b0000_0000,
	0b0011_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0000_0110,
	0b0000_0000,
	0b0000_0110,
	0b0000_0110,
	0b0000_0110,
	0b0110_0110,
	0b0110_0110,
	0b0011_1100,
	0b1110_0000,
	0b0110_0000,
	0b0110_0110,
	0b0110_1100,
	0b0111_1000,
	0b0110_1100,
	0b1110_0110,
	0b0000_0000,
	0b0011_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1110_1100,
	0b1111_1110,
	0b1101_0110,
	0b1101_0110,
	0b1101_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1101_1100,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1101_1100,
	0b0110_0110,
	0b0110_0110,
	0b0111_1100,
	0b0110_0000,
	0b1111_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_0110,
	0b1100_1100,
	0b1100_1100,
	0b0111_1100,
	0b0000_1100,
	0b0001_1110,
	0b0000_0000,
	0b0000_0000,
	0b1101_1100,
	0b0111_0110,
	0b0110_0000,
	0b0110_0000,
	0b1111_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b1100_0000,
	0b0111_1100,
	0b0000_0110,
	0b1111_1100,
	0b0000_0000,
	0b0011_0000,
	0b0011_0000,
	0b1111_1100,
	0b0011_0000,
	0b0011_0000,
	0b0011_0110,
	0b0001_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1100_0110,
	0b1101_0110,
	0b1101_0110,
	0b1111_1110,
	0b0110_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1110,
	0b0000_0110,
	0b1111_1100,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b0100_1100,
	0b0001_1000,
	0b0011_0010,
	0b0111_1110,
	0b0000_0000,
	0b0000_1110,
	0b0001_1000,
	0b0001_1000,
	0b0111_0000,
	0b0001_1000,
	0b0001_1000,
	0b0000_1110,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0111_0000,
	0b0001_1000,
	0b0001_1000,
	0b0000_1110,
	0b0001_1000,
	0b0001_1000,
	0b0111_0000,
	0b0000_0000,
	0b0111_0110,
	0b1101_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_0000,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1100_0110,
	0b1111_1110,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0000,
	0b1100_0000,
	0b1100_0110,
	0b0111_1100,
	0b0000_1100,
	0b0111_1000,
	0b1100_1100,
	0b0000_0000,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0000_1100,
	0b0001_1000,
	0b0111_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0000,
	0b0111_1100,
	0b0000_0000,
	0b0111_1100,
	0b1000_0010,
	0b0111_1000,
	0b0000_1100,
	0b0111_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b1100_0110,
	0b0000_0000,
	0b0111_1000,
	0b0000_1100,
	0b0111_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0011_0000,
	0b0001_1000,
	0b0111_1000,
	0b0000_1100,
	0b0111_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0011_0000,
	0b0011_0000,
	0b0111_1000,
	0b0000_1100,
	0b0111_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b1100_0000,
	0b1100_0000,
	0b0111_1110,
	0b0000_1100,
	0b0011_1000,
	0b0111_1100,
	0b1000_0010,
	0b0111_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0000,
	0b0111_1100,
	0b0000_0000,
	0b1100_0110,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0000,
	0b0111_1100,
	0b0000_0000,
	0b0011_0000,
	0b0001_1000,
	0b0111_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0000,
	0b0111_1100,
	0b0000_0000,
	0b0110_0110,
	0b0000_0000,
	0b0011_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0111_1100,
	0b1000_0010,
	0b0011_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0011_0000,
	0b0001_1000,
	0b0000_0000,
	0b0011_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b1100_0110,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0110,
	0b1100_0110,
	0b0000_0000,
	0b0011_1000,
	0b0110_1100,
	0b0111_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0110,
	0b1100_0110,
	0b0000_0000,
	0b0001_1000,
	0b0011_0000,
	0b1111_1110,
	0b1100_0000,
	0b1111_1000,
	0b1100_0000,
	0b1111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b0001_0010,
	0b1111_1110,
	0b1001_0000,
	0b1111_1110,
	0b0000_0000,
	0b0011_1110,
	0b0110_1100,
	0b1100_1100,
	0b1111_1110,
	0b1100_1100,
	0b1100_1100,
	0b1100_1110,
	0b0000_0000,
	0b0111_1100,
	0b1000_0010,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b1100_0110,
	0b0000_0000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0011_0000,
	0b0001_1000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0111_1000,
	0b1000_0100,
	0b0000_0000,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0110_0000,
	0b0011_0000,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b1100_0110,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1110,
	0b0000_0110,
	0b1111_1100,
	0b1100_0110,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b1100_0110,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0010,
	0b0111_1100,
	0b1100_1110,
	0b1101_0110,
	0b1110_0110,
	0b0111_1100,
	0b1000_0000,
	0b0011_1000,
	0b0110_1100,
	0b0110_0100,
	0b1111_0000,
	0b0110_0000,
	0b0110_0110,
	0b1111_1100,
	0b0000_0000,
	0b0011_1010,
	0b0110_1100,
	0b1100_1110,
	0b1101_0110,
	0b1110_0110,
	0b0110_1100,
	0b1011_1000,
	0b0000_0000,
	0b0000_0000,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_1110,
	0b0001_1011,
	0b0001_1000,
	0b0011_1100,
	0b0001_1000,
	0b1101_1000,
	0b0111_0000,
	0b0000_0000,
	0b0001_1000,
	0b0011_0000,
	0b0111_1000,
	0b0000_1100,
	0b0111_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0000_1100,
	0b0001_1000,
	0b0000_0000,
	0b0011_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0000_1100,
	0b0001_1000,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0001_1000,
	0b0011_0000,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b1100_1100,
	0b0111_0110,
	0b0000_0000,
	0b0111_0110,
	0b1101_1100,
	0b0000_0000,
	0b1101_1100,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0000_0000,
	0b0111_0110,
	0b1101_1100,
	0b0000_0000,
	0b1110_0110,
	0b1111_0110,
	0b1101_1110,
	0b1100_1110,
	0b0000_0000,
	0b0011_1100,
	0b0110_1100,
	0b0110_1100,
	0b0011_1110,
	0b0000_0000,
	0b0111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0011_1000,
	0b0110_1100,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b0111_1100,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0011_0000,
	0b0110_0011,
	0b0011_1110,
	0b0000_0000,
	0b0111_1110,
	0b1000_0001,
	0b1011_1001,
	0b1010_0101,
	0b1011_1001,
	0b1010_0101,
	0b1000_0001,
	0b0111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1110,
	0b0000_0110,
	0b0000_0110,
	0b0000_0000,
	0b0000_0000,
	0b0110_0011,
	0b1110_0110,
	0b0110_1100,
	0b0111_1110,
	0b0011_0011,
	0b0110_0110,
	0b1100_1100,
	0b0000_1111,
	0b0110_0011,
	0b1110_0110,
	0b0110_1100,
	0b0111_1010,
	0b0011_0110,
	0b0110_1010,
	0b1101_1111,
	0b0000_0110,
	0b0001_1000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0011_1100,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0011_0011,
	0b0110_0110,
	0b1100_1100,
	0b0110_0110,
	0b0011_0011,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1100_1100,
	0b0110_0110,
	0b0011_0011,
	0b0110_0110,
	0b1100_1100,
	0b0000_0000,
	0b0000_0000,
	0b0010_0010,
	0b1000_1000,
	0b0010_0010,
	0b1000_1000,
	0b0010_0010,
	0b1000_1000,
	0b0010_0010,
	0b1000_1000,
	0b0101_0101,
	0b1010_1010,
	0b0101_0101,
	0b1010_1010,
	0b0101_0101,
	0b1010_1010,
	0b0101_0101,
	0b1010_1010,
	0b0111_0111,
	0b1101_1101,
	0b0111_0111,
	0b1101_1101,
	0b0111_0111,
	0b1101_1101,
	0b0111_0111,
	0b1101_1101,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b1111_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_0000,
	0b0110_0000,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0110,
	0b0000_0000,
	0b0111_1100,
	0b1000_0010,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0110,
	0b0000_0000,
	0b0001_1000,
	0b0000_1100,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0110,
	0b0000_0000,
	0b0111_1110,
	0b1000_0001,
	0b1001_1101,
	0b1010_0001,
	0b1010_0001,
	0b1001_1101,
	0b1000_0001,
	0b0111_1110,
	0b0011_0110,
	0b0011_0110,
	0b1111_0110,
	0b0000_0110,
	0b1111_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0000_0000,
	0b0000_0000,
	0b1111_1110,
	0b0000_0110,
	0b1111_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b1111_0110,
	0b0000_0110,
	0b1111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0111_1110,
	0b1100_0000,
	0b1100_0000,
	0b0111_1110,
	0b0001_1000,
	0b0001_1000,
	0b0110_0110,
	0b0110_0110,
	0b0011_1100,
	0b0111_1110,
	0b0001_1000,
	0b0111_1110,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b1111_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1111,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1111,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b1111_1111,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0111_0110,
	0b1101_1100,
	0b0111_1100,
	0b0000_0110,
	0b0111_1110,
	0b1100_0110,
	0b0111_1110,
	0b0000_0000,
	0b0111_0110,
	0b1101_1100,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b1111_1110,
	0b1100_0110,
	0b0000_0000,
	0b0011_0110,
	0b0011_0110,
	0b0011_0111,
	0b0011_0000,
	0b0011_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0011_1111,
	0b0011_0000,
	0b0011_0111,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b1111_0111,
	0b0000_0000,
	0b1111_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1111,
	0b0000_0000,
	0b1111_0111,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0011_0111,
	0b0011_0000,
	0b0011_0111,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0000_0000,
	0b0000_0000,
	0b1111_1111,
	0b0000_0000,
	0b1111_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0011_0110,
	0b0011_0110,
	0b1111_0111,
	0b0000_0000,
	0b1111_0111,
	0b0011_0110,
	0b0011_0110,
	0b0011_0110,
	0b0000_0000,
	0b1100_0110,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b1100_0110,
	0b0000_0000,
	0b0011_0000,
	0b0111_1110,
	0b0000_1100,
	0b0111_1100,
	0b1100_1100,
	0b1100_1100,
	0b0111_1000,
	0b0000_0000,
	0b1111_1000,
	0b0110_1100,
	0b0110_0110,
	0b1111_0110,
	0b0110_0110,
	0b0110_1100,
	0b1111_1000,
	0b0000_0000,
	0b0111_1100,
	0b1000_0010,
	0b1111_1110,
	0b1100_0000,
	0b1111_1100,
	0b1100_0000,
	0b1111_1110,
	0b0000_0000,
	0b1100_0110,
	0b0000_0000,
	0b1111_1110,
	0b1100_0000,
	0b1111_1100,
	0b1100_0000,
	0b1111_1110,
	0b0000_0000,
	0b0011_0000,
	0b0001_1000,
	0b1111_1110,
	0b1100_0000,
	0b1111_1100,
	0b1100_0000,
	0b1111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0011_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0000_1100,
	0b0001_1000,
	0b0011_1100,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0011_1100,
	0b0100_0010,
	0b0011_1100,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0110_0110,
	0b0000_0000,
	0b0011_1100,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b1111_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1111,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_0000,
	0b0001_1000,
	0b0011_1100,
	0b0001_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b1111_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0011_0000,
	0b0110_0000,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b0111_1000,
	0b1100_1100,
	0b1100_1100,
	0b1101_1000,
	0b1100_1100,
	0b1100_0110,
	0b1100_1100,
	0b0000_0000,
	0b0111_1100,
	0b1000_0010,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b0000_1100,
	0b0000_0110,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b0111_0110,
	0b1101_1100,
	0b0111_1100,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0111_0110,
	0b1101_1100,
	0b0011_1000,
	0b0110_1100,
	0b1100_0110,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0110_0110,
	0b0111_1100,
	0b1100_0000,
	0b1110_0000,
	0b0110_0000,
	0b0111_1100,
	0b0110_0110,
	0b0110_0110,
	0b0111_1100,
	0b0110_0000,
	0b1111_0000,
	0b1111_0000,
	0b0110_0000,
	0b0111_1100,
	0b0110_0110,
	0b0111_1100,
	0b0110_0000,
	0b1111_0000,
	0b0000_0000,
	0b0001_1000,
	0b0011_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0111_1100,
	0b1000_0010,
	0b0000_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0110_0000,
	0b0011_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1100,
	0b0000_0000,
	0b0001_1000,
	0b0011_0000,
	0b1100_0110,
	0b1100_0110,
	0b1100_0110,
	0b0111_1110,
	0b0000_0110,
	0b1111_1100,
	0b0000_1100,
	0b0001_1000,
	0b0110_0110,
	0b0110_0110,
	0b0011_1100,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b1111_1111,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_1100,
	0b0001_1000,
	0b0011_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0001_1000,
	0b0111_1110,
	0b0001_1000,
	0b0001_1000,
	0b0000_0000,
	0b0111_1110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1111_1111,
	0b0000_0000,
	0b1111_1111,
	0b1110_0001,
	0b0011_0010,
	0b1110_0100,
	0b0011_1010,
	0b1111_0110,
	0b0010_1010,
	0b0101_1111,
	0b1000_0110,
	0b0111_1111,
	0b1101_1011,
	0b1101_1011,
	0b0111_1011,
	0b0001_1011,
	0b0001_1011,
	0b0001_1011,
	0b0000_0000,
	0b0011_1110,
	0b0110_0001,
	0b0011_1100,
	0b0110_0110,
	0b0110_0110,
	0b0011_1100,
	0b1000_0110,
	0b0111_1100,
	0b0000_0000,
	0b0001_1000,
	0b0000_0000,
	0b0111_1110,
	0b0000_0000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0000_1100,
	0b0011_1000,
	0b0011_1000,
	0b0110_1100,
	0b0110_1100,
	0b0011_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b1100_0110,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0001_1000,
	0b0011_1000,
	0b0001_1000,
	0b0001_1000,
	0b0011_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1000,
	0b0000_1100,
	0b0011_1000,
	0b0000_1100,
	0b0111_1000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0111_1000,
	0b0000_1100,
	0b0001_1000,
	0b0011_0000,
	0b0111_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0011_1100,
	0b0011_1100,
	0b0011_1100,
	0b0011_1100,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	0b0000_0000,
	// Char::SOH
	// Char::STX
	// Char::ETX
	// Char::EOT
	// Char::ENQ
	// Char::ACK
	// Char::BEL
	// Char::BS
	// Char::HT
	// Char::LF
	// Char::VT
	// Char::FF
	// Char::CR
	// Char::SO
	// Char::SI
	// Char::DLE
	// Char::DC1
	// Char::DC2
	// Char::DC3
	// Char::DC4
	// Char::NAK
	// Char::SYN
	// Char::ETB
	// Char::CAN
	// Char::EM
	// Char::SUB
	// Char::Escape
	// Char::FS
	// Char::GS
	// Char::RS
	// Char::US
	// Char::Space
	// Char::ExclamationMark
	// Char::QuotationMark
	// Char::NumberSign
	// Char::DollarSign
	// Char::PercentSign
	// Char::Ampersand
	// Char::Apostrophe
	// Char::LeftParenthesis
	// Char::RightParenthesis
	// Char::Asterisk
	// Char::PlusSign
	// Char::Comma
	// Char::HyphenMinus
	// Char::FullStop
	// Char::Solidus
	// Char::DigitZero
	// Char::DigitOne
	// Char::DigitTwo
	// Char::DigitThree
	// Char::DigitFour
	// Char::DigitFive
	// Char::DigitSix
	// Char::DigitSeven
	// Char::DigitEight
	// Char::DigitNine
	// Char::Colon
	// Char::Semicolon
	// Char::LessThanSign
	// Char::EqualsSign
	// Char::GreaterThanSign
	// Char::QuestionMark
	// Char::CommercialAt
	// Char::LatinCapitalLetterA
	// Char::LatinCapitalLetterB
	// Char::LatinCapitalLetterC
	// Char::LatinCapitalLetterD
	// Char::LatinCapitalLetterE
	// Char::LatinCapitalLetterF
	// Char::LatinCapitalLetterG
	// Char::LatinCapitalLetterH
	// Char::LatinCapitalLetterI
	// Char::LatinCapitalLetterJ
	// Char::LatinCapitalLetterK
	// Char::LatinCapitalLetterL
	// Char::LatinCapitalLetterM
	// Char::LatinCapitalLetterN
	// Char::LatinCapitalLetterO
	// Char::LatinCapitalLetterP
	// Char::LatinCapitalLetterQ
	// Char::LatinCapitalLetterR
	// Char::LatinCapitalLetterS
	// Char::LatinCapitalLetterT
	// Char::LatinCapitalLetterU
	// Char::LatinCapitalLetterV
	// Char::LatinCapitalLetterW
	// Char::LatinCapitalLetterX
	// Char::LatinCapitalLetterY
	// Char::LatinCapitalLetterZ
	// Char::LeftSquareBracket
	// Char::ReverseSolidus
	// Char::RightSquareBracket
	// Char::CircumflexAccent
	// Char::LowLine
	// Char::GraveAccent
	// Char::LatinSmallLetterA
	// Char::LatinSmallLetterB
	// Char::LatinSmallLetterC
	// Char::LatinSmallLetterD
	// Char::LatinSmallLetterE
	// Char::LatinSmallLetterF
	// Char::LatinSmallLetterG
	// Char::LatinSmallLetterH
	// Char::LatinSmallLetterI
	// Char::LatinSmallLetterJ
	// Char::LatinSmallLetterK
	// Char::LatinSmallLetterL
	// Char::LatinSmallLetterM
	// Char::LatinSmallLetterN
	// Char::LatinSmallLetterO
	// Char::LatinSmallLetterP
	// Char::LatinSmallLetterQ
	// Char::LatinSmallLetterR
	// Char::LatinSmallLetterS
	// Char::LatinSmallLetterT
	// Char::LatinSmallLetterU
	// Char::LatinSmallLetterV
	// Char::LatinSmallLetterW
	// Char::LatinSmallLetterX
	// Char::LatinSmallLetterY
	// Char::LatinSmallLetterZ
	// Char::LeftCurlyBracket
	// Char::VerticalLine
	// Char::RightCurlyBracket
	// Char::Tilde
	// Char::Delete
	// Char::LatinCapitalLetterCWithCedilla
	// Char::LatinSmallLetterUWithDiaeresis
	// Char::LatinSmallLetterEWithAcute
	// Char::LatinSmallLetterAWithCircumflex
	// Char::LatinSmallLetterAWithDiaeresis
	// Char::LatinSmallLetterAWithGrave
	// Char::LatinSmallLetterAWithRingAbove
	// Char::LatinSmallLetterCWithCedilla
	// Char::LatinSmallLetterEWithCircumflex
	// Char::LatinSmallLetterEWithDiaeresis
	// Char::LatinSmallLetterEWithGrave
	// Char::LatinSmallLetterIWithDiaeresis
	// Char::LatinSmallLetterIWithCircumflex
	// Char::LatinSmallLetterIWithGrave
	// Char::LatinCapitalLetterAWithDiaeresis
	// Char::LatinCapitalLetterAWithRingAbove
	// Char::LatinCapitalLetterEWithAcute
	// Char::LatinSmallLetterAe
	// Char::LatinCapitalLetterAe
	// Char::LatinSmallLetterOWithCircumflex
	// Char::LatinSmallLetterOWithDiaeresis
	// Char::LatinSmallLetterOWithGrave
	// Char::LatinSmallLetterUWithCircumflex
	// Char::LatinSmallLetterUWithGrave
	// Char::LatinSmallLetterYWithDiaeresis
	// Char::LatinCapitalLetterOWithDiaeresis
	// Char::LatinCapitalLetterUWithDiaeresis
	// Char::LatinSmallLetterOWithStroke
	// Char::PoundSign
	// Char::LatinCapitalLetterOWithStroke
	// Char::MultiplicationSign
	// Char::LatinSmallLetterFWithHook
	// Char::LatinSmallLetterAWithAcute
	// Char::LatinSmallLetterIWithAcute
	// Char::LatinSmallLetterOWithAcute
	// Char::LatinSmallLetterUWithAcute
	// Char::LatinSmallLetterNWithTilde
	// Char::LatinCapitalLetterNWithTilde
	// Char::FeminineOrdinalIndicator
	// Char::MasculineOrdinalIndicator
	// Char::InvertedQuestionMark
	// Char::RegisteredSign
	// Char::NotSign
	// Char::VulgarFractionOneHalf
	// Char::VulgarFractionOneQuarter
	// Char::InvertedExclamationMark
	// Char::LeftPointingDoubleAngleQuotationMark
	// Char::RightPointingDoubleAngleQuotationMark
	// Char::LightShade
	// Char::MediumShade
	// Char::DarkShade
	// Char::BoxDrawingsLightVertical
	// Char::BoxDrawingsLightVerticalAndLeft
	// Char::LatinCapitalLetterAWithAcute
	// Char::LatinCapitalLetterAWithCircumflex
	// Char::LatinCapitalLetterAWithGrave
	// Char::CopyrightSign
	// Char::BoxDrawingsDoubleVerticalAndLeft
	// Char::BoxDrawingsDoubleVertical
	// Char::BoxDrawingsDoubleDownAndLeft
	// Char::BoxDrawingsDoubleUpAndLeft
	// Char::CentSign
	// Char::YenSign
	// Char::BoxDrawingsLightDownAndLeft
	// Char::BoxDrawingsLightUpAndRight
	// Char::BoxDrawingsLightUpAndHorizontal
	// Char::BoxDrawingsLightDownAndHorizontal
	// Char::BoxDrawingsLightVerticalAndRight
	// Char::BoxDrawingsLightHorizontal
	// Char::BoxDrawingsLightVerticalAndHorizontal
	// Char::LatinSmallLetterAWithTilde
	// Char::LatinCapitalLetterAWithTilde
	// Char::BoxDrawingsDoubleUpAndRight
	// Char::BoxDrawingsDoubleDownAndRight
	// Char::BoxDrawingsDoubleUpAndHorizontal
	// Char::BoxDrawingsDoubleDownAndHorizontal
	// Char::BoxDrawingsDoubleVerticalAndRight
	// Char::BoxDrawingsDoubleHorizontal
	// Char::BoxDrawingsDoubleVerticalAndHorizontal
	// Char::CurrencySign
	// Char::LatinSmallLetterEth
	// Char::LatinCapitalLetterEth
	// Char::LatinCapitalLetterEWithCircumflex
	// Char::LatinCapitalLetterEWithDiaeresis
	// Char::LatinCapitalLetterEWithGrave
	// Char::LatinSmallLetterDotlessI
	// Char::LatinCapitalLetterIWithAcute
	// Char::LatinCapitalLetterIWithCircumflex
	// Char::LatinCapitalLetterIWithDiaeresis
	// Char::BoxDrawingsLightUpAndLeft
	// Char::BoxDrawingsLightDownAndRight
	// Char::FullBlock
	// Char::LowerHalfBlock
	// Char::BrokenBar
	// Char::LatinCapitalLetterIWithGrave
	// Char::UpperHalfBlock
	// Char::LatinCapitalLetterOWithAcute
	// Char::LatinSmallLetterSharpS
	// Char::LatinCapitalLetterOWithCircumflex
	// Char::LatinCapitalLetterOWithGrave
	// Char::LatinSmallLetterOWithTilde
	// Char::LatinCapitalLetterOWithTilde
	// Char::MicroSign
	// Char::LatinSmallLetterThorn
	// Char::LatinCapitalLetterThorn
	// Char::LatinCapitalLetterUWithAcute
	// Char::LatinCapitalLetterUWithCircumflex
	// Char::LatinCapitalLetterUWithGrave
	// Char::LatinSmallLetterYWithAcute
	// Char::LatinCapitalLetterYWithAcute
	// Char::Macron
	// Char::AcuteAccent
	// Char::SoftHyphen
	// Char::PlusMinusSign
	// Char::DoubleLowLine
	// Char::VulgarFractionThreeQuarters
	// Char::PilcrowSign
	// Char::SectionSign
	// Char::DivisionSign
	// Char::Cedilla
	// Char::DegreeSign
	// Char::Diaeresis
	// Char::MiddleDot
	// Char::SuperscriptOne
	// Char::SuperscriptThree
	// Char::SuperscriptTwo
	// Char::BlackSquare
	// Char::NoBreakSpace
];

// End of file
