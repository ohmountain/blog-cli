extern crate base64;

use arg::get_args;
use arg::print_help;
use std::ops::Add;
use net::Perform;
use net::Get;
use self::base64::encode;

pub fn create() {

    let args = get_args();

    if args.len() < 5 {
        print_help();
        return;
    }

    let title_arg = &args[1];
    let title     = &args[2];
    let type_arg  = &args[3];
    let post_type = &args[4];

    if title_arg != &String::from("--title") ||  type_arg != &String::from("--type") {
        print_help();
        return;
    }

    if title.len() > 30 {
        print_help();
        return;
    }

    println!("create post:");
    println!("    title:    {}", title);
    println!("     type:    {}", post_type);
}


pub fn show() {

    let args = get_args();

    if args.len() < 2 {
        print_help();
        return;
    }

    let title     = &args[1];

    if title.len() > 30 {
        print_help();
        return;
    }

    let mut url = String::from("http://127.0.0.1:8000/api/v2/posts/");

    let u = url.add(encode(title).as_str());

    println!("{:?}", u);

    let mut get = Get{ url: u};

    println!("{}", get.perform());
}

pub fn edit() {

    let args = get_args();

    if args.len() < 2 {
        print_help();
        return;
    }

    let title     = &args[1];

    if title.len() > 30 {
        print_help();
        return;
    }

    println!("edit post:");
    println!("    title:    {}", title);
}

pub fn delete() {

    let args = get_args();

    if args.len() < 2 {
        print_help();
        return;
    }

    let title     = &args[1];

    if title.len() > 30 {
        print_help();
        return;
    }

    println!("delete post:");
    println!("    title:    {}", title);
}


pub fn search() {

    let args = get_args();

    if args.len() < 2 {
        print_help();
        return;
    }

    let title     = &args[1];

    if title.len() > 30 {
        print_help();
        return;
    }

    println!("search post:");
    println!("    title:    {}", title);
}
