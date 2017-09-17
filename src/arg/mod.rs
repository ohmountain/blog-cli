use std::env;

pub enum Method {
    CreateType,
    CreatePost,
    Edit,
    Show,
    Delete,
    Search,
    Error
}

pub fn get_args() -> Vec<String> {
    env::args().skip(1).map(|x|x).collect()
}

pub fn print_help() {
    println!("    ");
    println!("blog-cli usage");
    println!("    ");
    println!("    blog-cli create-type --title type --sort number                  create a blog type with sort from 1 to 255");
    println!("    blog-cli create-post --title title                               create a blog type");
    println!("    blog-cli edit title                                              edit a blog in emacs");
    println!("    blog-cli show title                                              show a blog in vmd");
    println!("    blog-cli delete title                                            delete a blog");
    println!("    blog-cli search title                                            search a blog");
    println!("    ");
    println!("    标题长度不超过10个中文字或30个英文字");
    println!("    ");

}

pub fn get_method() -> Method {
    let args = get_args();

    if args.len() < 2 {
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

    if args[0] == String::from("search") {
        method = Method::Search;
    }


    match method {
        Method::Error => print_help(),
        _ => {},
    }

    method
}
