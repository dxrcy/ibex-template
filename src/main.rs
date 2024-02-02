use ibex::prelude::*;
use ibex::{routes, ssg};

const URL_ROOT: &str = "/ibex-template/";

fn main() {
    let routes = routes! [
        (/)    => at_index(),
        (/404) => at_404(),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}

fn at_index() -> Document {
    document! { [lang="en"]
        @use_base []
        div [style="text-align: center"] {
            h1 { "My Ibex App" }
            p {
                "This app was made using"
                ~ a [href="https://github.com/dxrcy/ibex-template"] {
                    "Ibex Template"
                }
            }
            p { a [href=url!("not/a/real/path")] { "404 Example" } }
            br/
            img [
                alt="Ibex logo",
                src=url!("static/icon.png"),
                width=300,
            ]/
        }
    }
}

fn at_404() -> Document {
    document! { [lang="en"]
        @use_base []
        div ."center" {
            "404 - Not found"
        }
    }
}

fn use_base() -> View {
    view! {
        HEAD {
            @use_meta [ibex::meta! {}]
            title { "My Ibex App" }
            link [rel="shortcut icon", href=url!("static/icon.png")]/
            link [rel="stylesheet", href=url!("css/base.css")]/
            @ssg::use_autoreload []
        }
    }
}
