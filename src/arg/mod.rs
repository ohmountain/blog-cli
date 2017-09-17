use std::env;

pub enum Method {
    CreateType,
    CreatePost,
    Edit,
    Show,
    Delete,
    Error
}


fn get_args() -> Vec<String> {
    env::args().skip(1).map(|x|x).collect()
}

fn print_help() {
    println!("    ");
    println!("blog-cli usage");
    println!("    ");
    println!("    blog-cli create-type --title type                   create a blog type");
    println!("    blog-clo create-post --title title                  create a blog type");
    println!("    blog-cli edit title                                 edit a blog in emacs");
    println!("    blog-cli show title                                 show a blog in vmd");
    println!("    blog-cli delete title                               delete a blog");
    println!("    ");

}

pub fn get_method() -> Method {
    let args = get_args();

    if args.len() < 3 {
       print_help();
       return Method::Error
    }

    let mut method = Method::Error;

    if args[0] == String::from("create-type") {
        method = Method::CreateType;
    }

    if args[0] == String::from("create-post") {
        method = Method::CreatePost;
    }

    if args[0] == String::from("edit") {
        method = Method::Edit;
    }

    if args[0] == String::from("show") {
        method = Method::Show;
    }

    if args[0] == String::from("delete") {
        method = Method::Delete;
    }


    match method {
        Method::Error => print_help(),
        _ => {},
    }

    method
}
