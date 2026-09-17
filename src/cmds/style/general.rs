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

//! Font effects and other text styling utilities.

use super::define_mode;

define_mode! {
	/// Makes text bold
	Bold: b"1"
	
	/// Makes the text appear fainter – the opposite of [`Bold`].
	Faint: b"2"
	
	/// Makes the text italic.
	Italic: b"3"
	
	/// Makes text blink.
	Blinking: b"5"
	
	/// Swaps the foreground color and the background color.
	InverseColors: b"7"
	
	/// Makes text invisible.
	/// 
	/// From my testing, this does not seem to affect the background color of 
	/// the text: it has the same affect as setting the foreground color of the 
	/// text to the background color of the text.
	Invisible: b"8"
	
	/// Makes text have a line cutting through them horizontally.
	Strikethrough: b"9"
}

define_mode! {
	/// Resets the effect of [`Bold`] and [`Faint`].
	ResetWeight: b"22"

	/// Resets the effect of [`Italic`].
	NoItalics: b"23"

	/// Resets the effect of [`Blinking`].
	ResetBlinking: b"25"

	/// Resets the effect of [`InverseColors`].
	ResetInverseColors: b"27"

	/// Resets the effect of [`Invisible`].
	Visible: b"28"

	/// Resets the effect of [`ResetStrikethrough`].
	ResetStrikethrough: b"29"
}