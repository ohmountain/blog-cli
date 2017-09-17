extern crate blog_cli;

use blog_cli::arg::parse_args;
use blog_cli::arg::Method;

fn main() {
    match parse_args() {
        Method::Create_post => {
            println!("Create type");
        },


        Method::Create_type => {
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


        Method::Error => {
            println!("erro");
        }
    }
}
