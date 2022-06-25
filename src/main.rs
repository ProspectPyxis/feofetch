mod fetch;

fn main() {
    let os = fetch::get_os();

    os.queue_print();
}

