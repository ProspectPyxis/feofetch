mod fetch;

fn main() {
    let os = fetch::get_os();
    let release = fetch::get_release();

    os.queue_print(7);
    release.queue_print(7);
}
