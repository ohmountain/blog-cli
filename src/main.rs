extern crate blog_cli;

use blog_cli::arg::get_method;
use blog_cli::arg::Method;

fn main() {
    match get_method() {
        Method::CreatePost => {
            println!("Create type");
        },


        Method::CreateType => {
            println!("Create post");
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
