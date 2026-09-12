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

use super::define_mode;

define_mode! {
	DefaultFgColor: b"39"
	DefaultBgColor: b"49"
}

pub mod standard {
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
	FgColor8Bit _(pub u8); {
		standalone: |self| b"\x1b[38;5;", self.0, b"m";
		with_csi: |self| b"\x1b[38;5;", self.0;
		with_semicolon: |self| b";38;5;", self.0;
		with_end: |self| b";38;5;", self.0, b"m";
	}

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
	FgColorRgb _{ pub red: u8, pub green: u8, pub blue: u8 } {
		standalone: |self| b"\x1b[38;2;", self.red, b";", self.green, b";", self.blue, b"m";
		with_csi: |self| b"\x1b[38;2;", self.red, b";", self.green, b";", self.blue;
		with_semicolon: |self| b";38;2;", self.red, b";", self.green, b";", self.blue;
		with_end: |self| b";38;2;", self.red, b";", self.blue, b";", self.green, b"m";
	}
	
	BgColorRgb _{ pub red: u8, pub green: u8, pub blue: u8 } {
		standalone: |self| b"\x1b[48;2;", self.red, b";", self.green, b";", self.blue, b"m";
		with_csi: |self| b"\x1b[48;2;", self.red, b";", self.green, b";", self.blue;
		with_semicolon: |self| b";48;2;", self.red, b";", self.green, b";", self.blue;
		with_end: |self| b";48;2;", self.red, b";", self.blue, b";", self.green, b"m";
	}
}
