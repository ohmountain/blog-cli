extern crate blog_cli;

use blog_cli::arg::get_method;
use blog_cli::arg::Method;

use blog_cli::types::create as create_type;

fn main() {
    match get_method() {
        Method::CreatePost => {
            println!("Create type");
        },


        Method::CreateType => {
            create_type();
        },


        Method::Show => {
            println!("show post");
        },


        Method::Delete => {
            println!("delete post");
        }

        Method::Edit => {
            println!("edit post");
        }


        _  => {}
    }


}
