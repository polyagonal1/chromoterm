use std::io::{self, Write};

use chromoterm::style::{
	SetStyle,
	general::{InverseColors, ResetInverseColors, Invisible},
	colors::{
		DefaultBgColor, DefaultFgColor,
		FgColor8Bit,
		standard::*,
	},
};

fn write_color(stdout: &mut io::StdoutLock, i: u8) -> io::Result<()> {
	if i % 21 == 0 {
		stdout.write_all(b"\n")?;
	}

	stdout.setter()
		.set(InverseColors)?
		.set(FgColor8Bit(i))?;

	write!(stdout, "{i:<3}")?;

	stdout.set_standalone(ResetInverseColors)?;
	stdout.write_all(b" ")?;

	Ok(())
}

fn main() -> io::Result<()> {
	let mut stdout = io::stdout().lock();

	stdout.set_standalone(DefaultBgColor)?;
	stdout.set_standalone(Invisible)?;

	for i in 0..8u8 {
		write_color(&mut stdout, i)?;
	}

	// stdout.write_all(b"\n")?;

	for i in 8..16u8 {
		write_color(&mut stdout, i)?;
	}

	// stdout.write_all(b"\n")?;

	for i in 16..232u8 {
		write_color(&mut stdout, i)?;
	}

	// stdout.write_all(b"\n\n")?;

	for i in 232..=255u8 {
		write_color(&mut stdout, i)?;
	}

	stdout.write_all(b"\n\n")?;

	Ok(())
}
