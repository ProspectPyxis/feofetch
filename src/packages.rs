use anyhow::Context;
use which::which;

struct PacManData {
	check_name: &'static str,
	display_name: &'static str,
	cmd_name: &'static str,
	args: &'static [&'static str],
}

// TODO: Add more package managers
const PACKAGE_MANAGERS: &[PacManData] = &[
	PacManData {
		check_name: "pacman-key",
		display_name: "pacman",
		cmd_name: "pacman",
		args: &["-Qq"],
	},
	PacManData {
		check_name: "dpkg",
		display_name: "dpkg",
		cmd_name: "dpkg-query",
		args: &["-f", ".\n", "-W"],
	},
	PacManData {
		check_name: "brew",
		display_name: "homebrew",
		cmd_name: "brew",
		args: &["list", "-1"],
	},
];

fn try_get_package_count(pacman: &PacManData) -> anyhow::Result<usize> {
	let out = std::process::Command::new(pacman.cmd_name)
		.args(pacman.args)
		.output()
		.with_context(|| {
			format!(
				"Failed to get stdout while running command {}",
				pacman.cmd_name
			)
		})?;

	let packages_count = std::str::from_utf8(&out.stdout).with_context(|| {
		format!(
			"Failed to parse stdout of command {} to string",
			pacman.cmd_name
		)
	})?;

	Ok(packages_count.lines().count())
}

pub fn get_packages(display_package_manager: bool) -> String {
	let packages_count = PACKAGE_MANAGERS
		.iter()
		.filter(|pacman| which(pacman.check_name).is_ok())
		.fold(0, |accum, pacman| {
			accum
				+ try_get_package_count(pacman).unwrap_or_else(|e| {
					eprintln!("{:#}", e);
					eprintln!("Assuming 0 packages installed by this package manager, moving on");
					0
				})
		});
	let pacmans: Vec<&'static str> = PACKAGE_MANAGERS
		.iter()
		.filter(|pacman| which(pacman.check_name).is_ok())
		.map(|pacman| pacman.display_name)
		.collect();

	if pacmans.is_empty() {
		"unknown".to_string()
	} else if display_package_manager {
		format!("{} ({})", packages_count, pacmans.join(", "))
	} else {
		format!("{}", packages_count)
	}
}
