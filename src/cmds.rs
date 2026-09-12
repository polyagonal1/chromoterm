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

#[cfg(feature = "cursor_controls")]
mod cursor;

#[cfg(feature = "alternate_screen")]
#[cfg_attr(docsrs, doc(cfg(feature = "alternate_screen")))]
pub mod screen;

#[cfg(feature = "erase_functions")]
#[cfg_attr(docsrs, doc(cfg(feature = "erase_functions")))]
pub mod erase;

#[cfg(feature = "style")]
#[cfg_attr(docsrs, doc(cfg(feature = "style")))]
pub mod style;

#[cfg(feature = "cursor_controls")]
#[cfg_attr(docsrs, doc(cfg(feature = "cursor_controls")))]
pub use cursor::CursorControls;

#[cfg(any(feature = "cursor_controls", feature = "style"))]
use writable::*;

#[cfg(any(feature = "cursor_controls", feature = "style"))]
mod writable {
	use std::io::{self, Write};
	use lexical_write_integer::{FormattedSize, ToLexical};

	#[cfg(any(feature = "cursor_controls", feature = "erase_functions"))]
	pub(super) const CSI: &'static [u8] = b"\x1b[";
	
	pub(super) trait Writeable {
		fn write_to<W: Write>(self, writer: &mut W) -> io::Result<()>;
	}

	impl<T: Writeable + Copy> Writeable for &T {
		fn write_to<W: Write>(self, writer: &mut W) -> io::Result<()> {
			(*self).write_to(writer)
		}
	}

	impl Writeable for &[u8] {
		#[inline]
		fn write_to<W: Write>(self, writer: &mut W) -> io::Result<()> {
			writer.write_all(self)
		}
	}

	impl<const N: usize> Writeable for [u8; N] {
		#[inline]
		fn write_to<W: Write>(self, writer: &mut W) -> io::Result<()> {
			self.as_slice().write_to(writer)
		}
	}

	impl Writeable for u16 {
		#[inline]
		fn write_to<W: Write>(self, writer: &mut W) -> io::Result<()> {
			let mut buf = [0u8; u16::FORMATTED_SIZE];

			let bytes = self.to_lexical(&mut buf);

			writer.write_all(&*bytes)
		}
	}

	impl Writeable for u8 {
		#[inline]
		fn write_to<W: Write>(self, writer: &mut W) -> io::Result<()> {
			let mut buf = [0u8; u8::FORMATTED_SIZE];

			let bytes = self.to_lexical(&mut buf);

			writer.write_all(&*bytes)
		}
	}

	macro_rules! write_all {
		(
			$writer:expr
			$(,
				$item:expr
			)* $(,)?
		) => {
			$(
				// $item.write_to($writer)?;
				crate::cmds::Writeable::write_to($item, $writer)?;
			)*
		}
	}

	pub(super) use write_all;
}