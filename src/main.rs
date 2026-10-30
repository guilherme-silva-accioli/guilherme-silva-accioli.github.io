use yew::prelude::*;
use yew_router::prelude::*;

// 1. Define all routes (pages) in the application
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/blog/:id")]
    Post { id: String },
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// 2. Data model for blog articles
#[derive(Clone, PartialEq)]
struct BlogPost {
    id: &'static str,
    title: &'static str,
    date: &'static str,
    summary: &'static str,
    content: &'static str,
}

const POSTS: &[BlogPost] = &[
    BlogPost {
        id: "getting-started-with-yew",
        title: "Getting Started with Rust & Yew",
        date: "October 30, 2026",
        summary: "Learn how to build lightning-fast web applications using Rust and WebAssembly.",
        content: "Yew is a modern Rust framework for creating multi-threaded frontend web apps with WebAssembly. It offers high performance, robust compile-time guarantees, and an ergonomic syntax similar to JSX.",
    },
    BlogPost {
        id: "why-webassembly-matters",
        title: "Why WebAssembly Matters for Modern Web",
        date: "October 28, 2026",
        summary: "WebAssembly brings near-native performance to the browser, unlocking new possibilities.",
        content: "WebAssembly (WASM) enables languages other than JavaScript to run safely and efficiently in web browsers. It is widely used for high-performance apps, games, visualization tools, and reliable web applications.",
    },
    BlogPost {
        id: "deploying-to-github-pages",
        title: "Automated Deployments with GitHub Actions",
        date: "October 25, 2026",
        summary: "How to bundle your WASM app with Trunk and publish it automatically.",
        content: "By setting up a GitHub Actions workflow, every commit to your main branch compiles your Rust code into WASM and automatically deploys the static files to GitHub Pages.",
    },
];

// 3. Home Page Component
#[function_component(HomePage)]
fn home_page() -> Html {
    html! {
        <div class="page-container">
            <section class="hero">
                <span class="badge">{ "🦀 Powered by Rust & WebAssembly" }</span>
                <h1>{ "Welcome to My Website" }</h1>
                <p class="subtitle">
                    { "A blazing-fast personal blog and portfolio built with Yew, WebAssembly, and hosted on GitHub Pages." }
                </p>
                <div class="hero-actions">
                    <Link<Route> to={Route::Blog} classes="btn btn-primary">{ "Read the Blog →" }</Link<Route>>
                    <Link<Route> to={Route::About} classes="btn btn-secondary">{ "About Me" }</Link<Route>>
                </div>
            </section>
        </div>
    }
}

// 4. Blog List Page Component
#[function_component(BlogPage)]
fn blog_page() -> Html {
    html! {
        <div class="page-container">
            <header class="section-header">
                <h2>{ "Blog Articles" }</h2>
                <p>{ "Thoughts, tutorials, and notes on software and technology." }</p>
            </header>
            <div class="posts-grid">
                {
                    POSTS.iter().map(|post| {
                        html! {
                            <article class="post-card" key={post.id}>
                                <span class="post-date">{ post.date }</span>
                                <h3>{ post.title }</h3>
                                <p>{ post.summary }</p>
                                <Link<Route> to={Route::Post { id: post.id.to_string() }} classes="read-more">
                                    { "Read Article →" }
                                </Link<Route>>
                            </article>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

// 5. Single Blog Post Page Component
#[derive(Properties, PartialEq)]
struct PostPageProps {
    id: String,
}

#[function_component(PostPage)]
fn post_page(props: &PostPageProps) -> Html {
    let post = POSTS.iter().find(|p| p.id == props.id);

    html! {
        <div class="page-container">
            <Link<Route> to={Route::Blog} classes="back-link">{ "← Back to Articles" }</Link<Route>>
            {
                if let Some(post) = post {
                    html! {
                        <article class="post-detail">
                            <span class="post-date">{ post.date }</span>
                            <h1>{ post.title }</h1>
                            <div class="post-content">
                                <p>{ post.content }</p>
                            </div>
                        </article>
                    }
                } else {
                    html! {
                        <div class="not-found-card">
                            <h2>{ "Post Not Found" }</h2>
                            <p>{ format!("No article with ID \"{}\" exists.", props.id) }</p>
                            <Link<Route> to={Route::Blog} classes="btn btn-primary">{ "Go to Blog" }</Link<Route>>
                        </div>
                    }
                }
            }
        </div>
    }
}

// 6. About Page Component
#[function_component(AboutPage)]
fn about_page() -> Html {
    html! {
        <div class="page-container">
            <header class="section-header">
                <h2>{ "About Me" }</h2>
            </header>
            <div class="about-card">
                <h3>{ "Guilherme Silva Accioli" }</h3>
                <p>
                    { "This website serves as a live demonstration of a modern frontend application built completely in Rust with the Yew framework." }
                </p>
                <div class="skills-list">
                    <span class="skill-tag">{ "Rust" }</span>
                    <span class="skill-tag">{ "WebAssembly" }</span>
                    <span class="skill-tag">{ "Yew" }</span>
                    <span class="skill-tag">{ "Trunk" }</span>
                    <span class="skill-tag">{ "GitHub Pages" }</span>
                </div>
            </div>
        </div>
    }
}

// 7. 404 Not Found Page Component
#[function_component(NotFoundPage)]
fn not_found_page() -> Html {
    html! {
        <div class="page-container not-found-card">
            <h1>{ "404" }</h1>
            <h2>{ "Page Not Found" }</h2>
            <p>{ "The page you are looking for does not exist." }</p>
            <Link<Route> to={Route::Home} classes="btn btn-primary">{ "Back to Home" }</Link<Route>>
        </div>
    }
}

// 8. Router Switcher
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Blog => html! { <BlogPage /> },
        Route::Post { id } => html! { <PostPage {id} /> },
        Route::About => html! { <AboutPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

// 9. Root App Component with Navbar and Footer
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-layout">
                <nav class="navbar">
                    <div class="nav-content">
                        <Link<Route> to={Route::Home} classes="nav-logo">
                            <span class="logo-icon">{ "🦀" }</span>
                            <span>{ "Guilherme" }</span>
                        </Link<Route>>
                        <div class="nav-links">
                            <Link<Route> to={Route::Home} classes="nav-item">{ "Home" }</Link<Route>>
                            <Link<Route> to={Route::Blog} classes="nav-item">{ "Blog" }</Link<Route>>
                            <Link<Route> to={Route::About} classes="nav-item">{ "About" }</Link<Route>>
                        </div>
                    </div>
                </nav>

                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>

                <footer class="footer">
                    <p>{ "Built with Rust + Yew + WebAssembly • Hosted on GitHub Pages" }</p>
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
