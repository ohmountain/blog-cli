extern crate serde_json;
extern crate console;

use arg::get_args;
use arg::print_help;
use std::u8;
use std::collections::HashMap;
use net::Post;
use net::Perform;
use self::serde_json::Value;
use self::console::Style;
use self::console::Emoji;

pub fn create() -> bool {

    let args = get_args();

    if args.len() < 5 {
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

    if title.len() > 30 {
        print_help();
        return false;
    }

    let sort_number = u8::from_str_radix(sort_str, 10);

    let sort;

    match sort_number {
        Ok(s) => {
            if s < 1 {
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

    let mut map = HashMap::new();

    map.insert(String::from("title"), title.to_string());


    map.insert(String::from("sort"), sort_str.to_string());


    let mut post = Post {
        url: String::from("http://localhost:3000/api/v2/type"),
        parameters: map
    };


    let data = post.perform();


    let d:Value = serde_json::from_str((&data.as_str())).unwrap();


    let code = d.get("code").unwrap().as_u64().unwrap_or(0);

    let cyan = Style::new().green().bold();
    if code != 200 {
        println!("This is {} neat", cyan.apply_to("quite"));
    } else {
        println!("\n{}   {}\n", Emoji("✨", ":-)"), cyan.apply_to("创建成功!"));
    }

    true
}
