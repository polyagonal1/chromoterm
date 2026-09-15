/*
    chromoterm – terminal manipulation library
    Copyright (C) 2026  @polyagonal1

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>
*/

//! Defines ways to make colored text.

use super::define_mode;

define_mode! {
	DefaultFgColor: b"39"
	DefaultBgColor: b"49"
}

pub mod standard {
	//! Standard colors supported by basically every terminal in existence.
	//!
	//! These are very basic colors that you would generally use in loggers
	//! (red for errors, yellow for warnings, etc).
	//!
	//! However, they won't be very useful for terminal games or other terminal
	//! applications that need more specific colors. If you need more specific
	//! colors, have a look at [`super::FgColor8Bit`] or [`super::FgColorRgb`].
	//!
	//! The exact RGB value each one outputs is very terminal-dependent and is
	//! often user-configurable.

	use super::define_mode;
	
	define_mode! {
		BlackFg: b"30"
		BlackBg: b"40"

		RedFg: b"31"
		RedBg: b"41"

		GreenFg: b"32"
		GreenBg: b"42"

		YellowFg: b"33"
		YellowBg: b"43"

		BlueFg: b"34"
		BlueBg: b"44"

		MagentaFg: b"35"
		MagentaBg: b"45"

		CyanFg: b"36"
		CyanBg: b"46"

		WhiteFg: b"37"
		WhiteBg: b"47"
	}
}

pub mod standard_bright {
	//! Bright variants of the standard colors defined in [`super::standard`].
	//!
	//! Like the standard colors, these are also very well-supported.

	use super::define_mode;
	
	define_mode! {
		BrightBlackFg: b"90"
		BrightBlackBg: b"100"
		
		BrightRedFg: b"91"
		BrightedBg: b"101"
		
		BrightGreenFg: b"92"
		BrightGreenBg: b"102"
		
		BrightYellowFg: b"93"
		BrightYellowBg: b"103"
		
		BrightBlueFg: b"94"
		BrightBlueBg: b"104"
		
		BrightMagentaFg: b"95"
		BrightMagentaBg: b"105"
		
		BrightCyanFg: b"96"
		BrightCyanBg: b"106"
		
		BrightWhiteFg: b"97"
		BrightWhiteBg: b"107"
	}
}

// =====================================================
// ====================== 256-Color ====================
// =====================================================

define_mode! {
	/// Foreground color based on a color id.
	///
	/// This struct sets the foreground color based on a color id which maps to 
	/// a specific color based on this color table: 
	/// ![Color table describing which id maps to which color](https://github.com/polyagonal1/chromoterm/blob/main/docs/256-color/256color-table-fg.png)
	/// 
	/// The ids `0..=7` are the same as the standard colors. 
	/// The ids `8..=15` are the same as the standard bright colors.
	/// The ids `16..=231` make a 6x6x6 color cube with 216 colors.
	/// The ids `232..=255` is a greyscale from black to white in 24 steps. 
	/// 
	/// Not that all possible values of a `u8` are valid ids, since the ids are 
	/// in the range `0..=255`, which is the range of a `u8`.
	FgColor8Bit _(pub u8); {
		standalone: |self| b"\x1b[38;5;", self.0, b"m";
		with_csi: |self| b"\x1b[38;5;", self.0;
		with_semicolon: |self| b";38;5;", self.0;
		with_end: |self| b";38;5;", self.0, b"m";
	}

	/// Background color based on a color id. 
	/// 
	/// This struct sets the background color based on a color id which maps to 
	/// a specific color based on this color table: 
	/// ![Color table describing which id maps to which color](https://github.com/polyagonal1/chromoterm/blob/main/docs/256-color/256color-table-bg.png)
	/// 
	/// The table has the same colors as the one for the foreground color but 
	/// set the background color instead; therefore it has the same rules: 
	/// 
	/// The ids `0..=7` are the same as the standard colors. 
	/// The ids `8..=15` are the same as the standard bright colors.
	/// The ids `16..=231` make a 6x6x6 color cube with 216 colors.
	/// The ids `232..=255` is a greyscale from black to white in 24 steps. 
	BgColor8Bit _(pub u8); {
		standalone: |self| b"\x1b[48;5;", self.0, b"m";
		with_csi: |self| b"\x1b[48;5;", self.0;
		with_semicolon: |self| b";48;5;", self.0;
		with_end: |self| b";48;5;", self.0, b"m";
	}
}

// =====================================================
// ====================== Truecolor ====================
// =====================================================

define_mode! {
	/// Sets the foreground color using an RGB value. 
	///
	/// This is not supported everywhere. You can try checking the `COLORTERM` 
	/// environment variable for `truecolor` or `24bit` for support, but this 
	/// may not necessarily be set even for terminals that do support 
	/// truecolor and should not be considered a reliable method.
	FgColorRgb _{ pub red: u8, pub green: u8, pub blue: u8 } {
		standalone: |self| b"\x1b[38;2;", self.red, b";", self.green, b";", self.blue, b"m";
		with_csi: |self| b"\x1b[38;2;", self.red, b";", self.green, b";", self.blue;
		with_semicolon: |self| b";38;2;", self.red, b";", self.green, b";", self.blue;
		with_end: |self| b";38;2;", self.red, b";", self.blue, b";", self.green, b"m";
	}
	
	/// Sets the background color using an RGB value. 
	///
	/// This is not supported everywhere. You can try checking the `COLORTERM` 
	/// environment variable for `truecolor` or `24bit` for support, but this 
	/// may not necessarily be set even for terminals that do support 
	/// truecolor and should not be considered a reliable method.
	BgColorRgb _{ pub red: u8, pub green: u8, pub blue: u8 } {
		standalone: |self| b"\x1b[48;2;", self.red, b";", self.green, b";", self.blue, b"m";
		with_csi: |self| b"\x1b[48;2;", self.red, b";", self.green, b";", self.blue;
		with_semicolon: |self| b";48;2;", self.red, b";", self.green, b";", self.blue;
		with_end: |self| b";48;2;", self.red, b";", self.blue, b";", self.green, b"m";
	}
}
