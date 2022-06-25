use which::which;

struct PacManData {
    check_name: &'static str,
    cmd_name: &'static str,
    args: &'static[&'static str],
}

// TODO: Add more package managers
const PACKAGE_MANAGERS: &[PacManData] = &[
    PacManData {
        check_name: "pacman-key",
        cmd_name: "pacman",
        args: &["-Qq"],
    },
];

pub fn get_packages() -> String {
    let mut pacmans = Vec::new();

    let packages_count = PACKAGE_MANAGERS.iter().filter(|pacman| which(pacman.check_name).is_ok())
        .fold(0, |accum, pacman| {
            let out = std::process::Command::new(pacman.cmd_name)
                .args(pacman.args)
                .output()
                .expect("something went wrong");
            let packages = match std::str::from_utf8(&out.stdout) {
                Ok(p) => p,
                Err(_) => return accum,
            };
            pacmans.push(pacman.cmd_name);
            accum + packages.lines().count()
        });

    if pacmans.is_empty() {
        "unknown".to_string()
    } else {
        format!("{} ({})", packages_count, pacmans.join(", "))
    }
}
