use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BlogPost {
    pub id: &'static str,
    pub log_no: &'static str,
    pub title: &'static str,
    pub date: &'static str,
    pub summary: &'static str,
    pub content_md: &'static str,
    pub thumbnail: &'static str,
}

pub const POSTS: &[BlogPost] = &[
    BlogPost {
        id: "getting-started-with-yew",
        log_no: "№ 001",
        title: "Getting Started with Rust & Yew in WebAssembly",
        date: "October 30, 2026",
        summary: "Learn how to build lightning-fast web applications using Rust and WebAssembly.",
        content_md: include_str!("../posts/01-getting-started-with-yew.md"),
        thumbnail: "https://images.unsplash.com/photo-1555066931-4365d14bab8c?w=800&auto=format&fit=crop&q=80",
    },
    BlogPost {
        id: "why-webassembly-matters",
        log_no: "№ 002",
        title: "Why WebAssembly Matters for the Modern Web",
        date: "October 28, 2026",
        summary: "WebAssembly brings near-native performance to the browser, unlocking new possibilities.",
        content_md: include_str!("../posts/02-why-webassembly-matters.md"),
        thumbnail: "https://images.unsplash.com/photo-1518770660439-4636190af475?w=800&auto=format&fit=crop&q=80",
    },
    BlogPost {
        id: "deploying-to-github-pages",
        log_no: "№ 003",
        title: "Automated Deployments with GitHub Actions",
        date: "October 25, 2026",
        summary: "How to bundle your WASM app with Trunk and publish it automatically.",
        content_md: include_str!("../posts/03-deploying-to-github-pages.md"),
        thumbnail: "https://images.unsplash.com/photo-1618401471353-b98afee0b2eb?w=800&auto=format&fit=crop&q=80",
    },
    BlogPost {
        id: "rust-concurrency-patterns",
        log_no: "№ 004",
        title: "Fearless Concurrency & Performance in Rust",
        date: "October 20, 2026",
        summary: "Exploring async programming, threads, and memory safety without garbage collection.",
        content_md: include_str!("../posts/04-rust-concurrency-patterns.md"),
        thumbnail: "https://images.unsplash.com/photo-1526374965328-7f61d4dc18c5?w=800&auto=format&fit=crop&q=80",
    },
    BlogPost {
        id: "building-beautiful-uis-with-css-grid",
        log_no: "№ 005",
        title: "Architecting Vintage Layouts with Modern CSS Grid",
        date: "October 16, 2026",
        summary: "A practical guide to multi-column cards, aspect ratios, and fluid typography.",
        content_md: include_str!("../posts/05-building-beautiful-uis-with-css-grid.md"),
        thumbnail: "https://images.unsplash.com/photo-1507238691740-187a5b1d37b8?w=800&auto=format&fit=crop&q=80",
    },
    BlogPost {
        id: "future-of-web-tooling",
        log_no: "№ 006",
        title: "The Next Era of Compiled Web Tooling",
        date: "October 10, 2026",
        summary: "How Rust-based compilers, bundlers, and runtimes are revolutionizing frontend speed.",
        content_md: include_str!("../posts/06-future-of-web-tooling.md"),
        thumbnail: "https://images.unsplash.com/photo-1451187580459-43490279c0fa?w=800&auto=format&fit=crop&q=80",
    },
];

/// Helper that converts markdown strings into rendered Yew Html
pub fn render_markdown(md: &str) -> Html {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Html::from_html_unchecked(AttrValue::from(html_output))
}
