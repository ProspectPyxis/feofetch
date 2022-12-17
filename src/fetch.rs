use crate::{config::Config, data::FetchData};
use crossterm::{
	style::{Print, PrintStyledContent, Stylize},
	QueueableCommand,
};
use std::io::{self, stdout};

pub fn print_all_fetches(
	data: &[FetchData],
	conf: &Config,
	ascii: Option<&str>,
) -> Result<(), io::Error> {
	let (ascii_lines_count, ascii_max_length) = match ascii {
		Some(a) => (
			a.lines().count(),
			a.lines().fold(0, |acc, x| acc.max(x.chars().count())) + conf.ascii.align_spaces,
		),
		None => (0, 0),
	};
	let data_start = (ascii_lines_count.saturating_sub(data.len()) / 2).min(ascii_lines_count);
	let total_lines = ascii_lines_count.max(data.len());

	let data_pos = if !conf.use_icons {
		data.iter()
			.fold(0, |acc, x| acc.max(x.label.chars().count()))
	} else {
		1
	} + conf.align_spaces;

	let mut stdout = stdout();

	for _ in 0..conf.offset.1 {
		stdout.queue(Print('\n'))?;
	}

	let mut ascii_lines = ascii.unwrap_or("").lines().peekable();
	let mut data_lines = data.iter().peekable();
	for index in 0..total_lines {
		stdout.queue(Print(" ".repeat(conf.offset.0)))?;

		if ascii_lines.peek().is_some() {
			stdout.queue(PrintStyledContent(
				format!("{:ascii_max_length$}", ascii_lines.next().unwrap()).bold(),
			))?;
		} else {
			stdout.queue(Print(" ".repeat(ascii_max_length)))?;
		}

		if index >= data_start && data_lines.peek().is_some() {
			data_lines
				.next()
				.unwrap()
				.queue_print(&mut stdout, data_pos, conf)?;
		}

		stdout.queue(Print('\n'))?;
	}

	for _ in 0..conf.padding_lines {
		stdout.queue(Print('\n'))?;
	}

	Ok(())
}
