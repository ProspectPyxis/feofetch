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
    let mut packages_count = 0;

    for pacman in PACKAGE_MANAGERS {
        if which(pacman.check_name).is_ok() {
            let out = std::process::Command::new(pacman.cmd_name)
                .args(pacman.args)
                .output()
                .expect("something went wrong");

            println!("{}", std::str::from_utf8(&out.stdout).unwrap());
        }
    }
    "hello".to_string()
}
