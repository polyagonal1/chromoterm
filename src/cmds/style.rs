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

//! Utilities for styling text
//! 
//! The main entrypoint for styling text in this module is the [`SetStyle`]
//! trait. It is automatically implemented for any [writer][Write]. 
//! 
//! The three modules in this module all contain different styles that text 
//! can have, like underlines, bold mode, the color of the text, etc. All the 
//! types in those modules implement the [`Style`] trait which is what the 
//! methods in [`SetStyle`] work over.
//! 
//! # Examples
//! ```
//! use std::io::{self, Write};
//! use chromoterm::style::{
//! 	SetStyle,
//! 	general::Italic,
//! 	underlines::{StraightUnderline, NoUnderline},
//! 	colors::{
//! 		standard::*,
//! 		standard_bright::*,
//! 	},
//! };
//!
//! let mut stdout = io::stdout().lock();
//!
//! writeln!(stdout, "Normally styled text")?;
//!
//! stdout.set_standalone(StraightUnderline)?;
//! writeln!(stdout, "Underlined text")?;
//!
//! stdout.setter()
//! 	.set(NoUnderline)?
//! 	.set(Italic)?
//! 	.set(GreenFg)?
//! 	.set(BrightBlueBg)?;
//!
//! writeln!(stdout, "Green italicised text with a bright blue background")?;
//!
//! stdout.reset_style()?;
//!
//! writeln!(stdout, "Normally styled text")?;
//!
//! Ok::<(), io::Error>(())
//! ```

use std::io::{self, Write};
use style_inner::StyleInner;

pub mod general;
pub mod underlines;
pub mod colors;

/// Trait allowing text to be styled. It is automatically implemented on any
/// writer.
///
/// # Examples
/// ```rust
/// use std::io::{self, Write};
/// # use chromoterm::style::{
/// # 	SetStyle,
/// # 	underlines::StraightUnderline,
/// # 	colors::standard::*,
/// # };
///
/// let mut stdout = io::stdout().lock();
///
/// stdout.setter()
/// 	.set(StraightUnderline)?
/// 	.set(BlueFg)?;
///
/// writeln!(stdout, "This text will be blue and underlined")?;
///
/// stdout.reset_style()?;
///
/// writeln!(stdout, "This text will not be blue or underlined")?;
///
/// # Ok::<(), io::Error>(())
/// ```
pub trait SetStyle: Write {
	/// Returns a [`StyleSetter<&'a mut Self>`][StyleSetter] which allows you
	/// to set multiple modes in one SGR escape sequence, leveraging the borrow
	/// checker to ensure correctness.
	///
	/// The returned `StyleSetter` contains an exclusive mutable reference to
	/// `Self` which is used for setting the style, which means you cannot
	/// write to `Self` when the `StyleSetter` hasn't yet been dropped, as it
	/// may still be using `Self` for setting the style. This means you can't
	/// do this:
	/// ```compile_fail
	/// use std::io::{self, Write};
	/// # use chromoterm::style::{SetStyle, general::Bold, colors::standard::RedFg};
	///
	/// let mut stdout = io::stdout().lock();
	///
	/// let mut setter = stdout.setter();
	/// setter = setter.set(Bold)?;
	///
	/// writeln!(stdout, "Hello World")?;
	///
	/// setter.set(RedFg)?;
	///
	/// # Ok::<(), io::Error>(())
	/// ```
	///
	/// # Examples
	/// ```
	/// use std::io::{self, Write};
	/// use chromoterm::style::{
	/// 	SetStyle,
	/// 	general::Bold,
	/// 	colors::{
	/// 		standard::*,
	/// 		standard_bright::*,
	/// 	}
	/// };
	///
	/// let mut stdout = io::stdout().lock();
	///
	/// stdout.setter()
	/// 	.set(BrightYellowFg)?
	/// 	.set(BlueBg)?
	/// 	.set(Bold)?;
	///
	/// writeln!(stdout, "Bold, bright yellow text with a blue background")?;
	///
	/// stdout.reset_style()?;
	///
	/// writeln!(stdout, "Normally styled text")?;
	///
	/// # Ok::<(), io::Error>(())
	/// ```
	fn setter<'a>(&'a mut self) -> StyleSetter<&'a mut Self> {
		StyleSetter {
			writer: &mut *self,
			has_any_style_been_set: false,
		}
	}

	/// Sets a single given style.
	///
	/// # Examples
	/// ```
	/// use std::io::{self, Write};
	/// # use chromoterm::style::{SetStyle, general::Italic};
	///
	/// let mut stdout = io::stdout().lock();
	///
	/// stdout.set_standalone(Italic)?;
	///
	/// writeln!(stdout, "This text is in italics.")?;
	///
	/// # Ok::<(), io::Error>(())
	/// ```
	fn set_standalone<S: Style>(&mut self, style: S) -> io::Result<()> {
		style.set_standalone(self)
	}

	/// Resets the style to the default. 
	/// 
	/// # Examples
	/// ```
	/// use std::io::{self, Write};
	/// # use chromoterm::style::{SetStyle, general::Bold};
	///
	/// let mut stdout = io::stdout().lock();
	///
	/// stdout.set_standalone(Bold)?;
	///
	/// stdout.reset_style()?;
	///
	/// writeln!(stdout, "Normally styled (not bold) text")?;
	///
	/// # Ok::<(), io::Error>(())
	/// ```
	fn reset_style(&mut self) -> io::Result<()> {
		self.write_all(b"\x1b[0m")
	}
}

impl<W: Write> SetStyle for W {}

/// Represents a style that text can be in, like bold mode or the color of the 
/// text.
pub trait Style: StyleInner {}

mod style_inner {
	use std::io::{self, Write};

	pub trait StyleInner {

		fn set_standalone<W: Write>(&self, writer: W) -> io::Result<()>;

		fn set_with_csi<W: Write>(&self, writer: W) -> io::Result<()>;

		fn set_with_semicolon<W: Write>(&self, writer: W) -> io::Result<()>;

		fn set_with_end<W: Write>(&self, writer: W) -> io::Result<()>;
	}
}

/// Wrapper around a writer (but usually a mutable reference to a writer) which
/// allows setting multiple styles with the same SGR sequence.
///
/// This struct is created with [`SetStyle::setter`]. See that trait method's
/// documentation for more info.
pub struct StyleSetter<W: Write> {
	writer: W,
	has_any_style_been_set: bool,
}

impl<'a, W: Write> Drop for StyleSetter<W> {
	fn drop(&mut self) {
		if self.has_any_style_been_set {
			let _ = self.writer.write_all(b"m");
		}
	}
}

impl<'a, W: Write> StyleSetter<W> {
	/// Set the given style.
	pub fn set<S: Style>(mut self, mode: S) -> io::Result<Self> {
		if self.has_any_style_been_set {
			mode.set_with_semicolon(&mut self.writer)?;
		} else {
			self.has_any_style_been_set = true;
			mode.set_with_csi(&mut self.writer)?;
		}

		Ok(self)
	}
}

macro_rules! define_mode {
	($(
		$(#[$($attrs:tt)*])*
		$name:ident $(_$test:tt)? $(; $($semicolon1:ty,)?)? {
			
			$(#[$($standalone_attrs:tt)*])*
			standalone: |$standalone_self_var_name:ident| $($standalone_to_write:expr),+;
			
			$(#[$($with_csi_attrs:tt)*])*
			with_csi: |$with_csi_self_var_name:ident| $($with_csi_to_write:expr),+;
			
			$(#[$($with_semicolonattrs:tt)*])*
			with_semicolon: |$with_semicolon_self_var_name:ident| $($with_semicolon_to_write:expr),+;
			
			$(#[$($with_end_attrs:tt)*])*
			with_end: |$with_end_self_var_name:ident| $($with_end_to_write:expr),+;
		}
	)*) => {$(
		$(#[$($attrs)*])*
		pub struct $name $($test)? $(; $($semicolon1)?)?
		
		impl crate::style::Style for $name {}
		
		impl crate::style::StyleInner for $name {
			
			$(#[$($standalone_attrs)*])*
			fn set_standalone<W: ::std::io::Write>(
				&$standalone_self_var_name,
				mut writer: W
			) -> ::std::io::Result<()> {
        		crate::cmds::write_all!(&mut writer, $($standalone_to_write),+);
				
				Ok(())
    		}
			
			$(#[$($with_csi_attrs)*])*
			fn set_with_csi<W: ::std::io::Write>(
				&$with_csi_self_var_name,
				mut writer: W
			) -> ::std::io::Result<()> {
        		crate::cmds::write_all!(&mut writer, $($with_csi_to_write),+);

				Ok(())
    		}
			
			$(#[$($with_semicolonattrs)*])*
			fn set_with_semicolon<W: ::std::io::Write>(
				&$with_semicolon_self_var_name,
				mut writer: W
			) -> ::std::io::Result<()> {
        		crate::cmds::write_all!(&mut writer, $($with_semicolon_to_write),+);

				Ok(())
    		}
			
			$(#[$($with_end_attrs)*])*
			fn set_with_end<W: ::std::io::Write>(
				&$with_end_self_var_name,
				mut writer: W
			) -> ::std::io::Result<()> {
        		crate::cmds::write_all!(&mut writer, $($with_end_to_write),+);

				Ok(())
    		}
		}
	)*};
	($(
		$(#[$($attrs:tt)*])*
		$name:ident: $code:literal
	)*) => {$(
		$crate::style::define_mode! {
			$(#[$($attrs)*])*
			$name; {
				standalone: |self| constcat::concat_slices!([u8]: b"\x1b[", $code, b"m");
				with_csi: |self| constcat::concat_slices!([u8]: b"\x1b[", $code);
				with_semicolon: |self| constcat::concat_slices!([u8]: b";", $code);
				with_end: |self| constcat::concat_slices!([u8]: b";", $code, b"m");
			}
		}
	)*};
}

use define_mode;
