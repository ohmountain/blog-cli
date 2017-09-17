use std::env;
use arg::get_args;
use arg::print_help;
use std::u8;

pub fn create() -> bool {

    let args = get_args();

    if (args.len() < 5) {
        print_help();
        return false;
    }

    let title_arg = &args[1];
    let title     = &args[2];
    let sort_arg  = &args[3];
    let sort_str      = &args[4];

    if title_arg != &String::from("--title") || sort_arg != &String::from("--sort") {
        print_help();
        return false;
    }

    if title.len() > 255 {
        print_help();
        return false;
    }

    let sort_number = u8::from_str_radix(sort_str, 10);

    let sort;

    match sort_number {
        Ok(s) => {
            if (s < 1) {
                print_help();
                return false;
            }
            sort = s;
        },

        Err(_) => {
            print_help();
            return false;
        }
    }

    println!("Create a type: ");
    println!("    title:    {}", title);
    println!("    sort:     {}", sort);

    true
}
