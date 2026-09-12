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
	/// Makes text bold
	Bold: b"1"
	
	/// Makes the text appear fainter
	Faint: b"2"
	
	/// Makes the text italic.
	Italic: b"3"
	
	Blinking: b"5"
	
	InverseColors: b"7"
	
	Invisible: b"8"
	
	Strikethrough: b"9"
}

define_mode! {
	ResetWeight: b"22"
	
	NoItalics: b"23"
	
	ResetBlinking: b"25"
	
	ResetInverseColors: b"27"
	
	Visible: b"28"
	
	ResetStrikethrough: b"29"
}