mod fetch;
mod config;

fn main() {
    let os = fetch::get_os();
    let version = fetch::get_version();

    os.queue_print(7);
    version.queue_print(7);
}
