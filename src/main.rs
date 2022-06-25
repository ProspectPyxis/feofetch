mod config;
mod fetch;
mod packages;

fn main() {
    let conf = config::get_config();

    let data = fetch::fetch_all(&conf);
    fetch::print_all_fetches(&data, &conf);
}
