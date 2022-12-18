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

fn get_packages_count(sum: usize, pacman: &PacManData) -> anyhow::Result<usize> {
	let out = std::process::Command::new(pacman.cmd_name)
		.args(pacman.args)
		.output()
		.with_context(|| {
			format!(
				"Failed to read stdout while reading packages of {} (command: {})",
				pacman.display_name, pacman.cmd_name,
			)
		})?;
	let packages = std::str::from_utf8(&out.stdout).with_context(|| {
		format!(
			"Failed to parse stdout while reading packages of {} (command: {})",
			pacman.display_name, pacman.cmd_name
		)
	})?;
	Ok(sum + packages.lines().count())
}

pub fn get_packages(display_package_manager: bool) -> anyhow::Result<String> {
	let packages_count = PACKAGE_MANAGERS
		.iter()
		.filter(|pacman| which(pacman.check_name).is_ok())
		.try_fold(0, get_packages_count)?;
	let pacmans: Vec<&'static str> = PACKAGE_MANAGERS
		.iter()
		.filter(|pacman| which(pacman.check_name).is_ok())
		.map(|pacman| pacman.display_name)
		.collect();

	Ok(if pacmans.is_empty() {
		"unknown".to_string()
	} else if display_package_manager {
		format!("{} ({})", packages_count, pacmans.join(", "))
	} else {
		format!("{}", packages_count)
	})
}
