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
	/// Makes the text not underlined (the default).
	NoUnderline: b"24"
}

define_mode! {
	/// Makes the text underlined
	StraightUnderline: b"4"
	
	DoubleUnderline: b"4:2"
	
	CurlyUnderline: b"4:3"
	
	DottedUnderline: b"4:4"
	
	DashedUnderline: b"4:5"
}