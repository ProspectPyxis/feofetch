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

const STDOUT_PARSE_FAIL_MSG: &str =
	"Assuming 0 packages installed by this package manager and moving on";

pub fn get_packages(display_package_manager: bool) -> String {
	let packages_count = PACKAGE_MANAGERS
		.iter()
		.filter(|pacman| which(pacman.check_name).is_ok())
		.fold(0, |accum, pacman| {
			let out = std::process::Command::new(pacman.cmd_name)
				.args(pacman.args)
				.output()
				.with_context(|| {
					format!(
						"Failed to get stdout while running command {}",
						pacman.cmd_name
					)
				});
			match out {
				Ok(o) => match std::str::from_utf8(&o.stdout).with_context(|| {
					format!(
						"Failed to parse stdout of command {} to string",
						pacman.cmd_name
					)
				}) {
					Ok(p) => accum + p.lines().count(),
					Err(e) => {
						eprintln!("{:#}", e);
						eprintln!("{}", STDOUT_PARSE_FAIL_MSG);
						accum
					}
				},
				Err(e) => {
					eprintln!("{:#}", e);
					eprintln!("{}", STDOUT_PARSE_FAIL_MSG);
					accum
				}
			}
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
