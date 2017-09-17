extern crate blog_cli;

use blog_cli::arg::get_method;
use blog_cli::arg::Method;

use blog_cli::types::create as create_type;
use blog_cli::posts::create as create_post;

use blog_cli::posts::show as show_post;
use blog_cli::posts::edit as edit_post;
use blog_cli::posts::delete as delete_post;
use blog_cli::posts::search as search_post;

fn main() {
    match get_method() {
        Method::CreatePost => {
            create_post();
        },

        Method::CreateType => {
            create_type();
        },

        Method::Show => {
            show_post();
        },

        Method::Edit => {
            edit_post();
        }

        Method::Delete => {
            delete_post();
        }

        Method::Search => {
           search_post();
        }

        _  => {}
    }
}
