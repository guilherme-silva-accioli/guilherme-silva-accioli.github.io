mod posts;

use posts::{BlogPost, POSTS, render_markdown};
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

// 2. Reusable Polaroid / Field Specimen Card Component
#[derive(Properties, PartialEq)]
pub struct PostCardProps {
    pub post: BlogPost,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let post = &props.post;
    html! {
        <Link<Route> to={Route::Post { id: post.id.to_string() }} classes="thumb-card-link">
            <article class="thumb-card">
                <div class="thumb-wrapper">
                    <img src={post.thumbnail} alt={post.title} loading="lazy" />
                    <span class="specimen-tag">{ post.log_no }</span>
                </div>
                <div class="thumb-info">
                    <time class="thumb-date">
                        <span class="calendar-symbol">{ "§" }</span>
                        { post.date }
                    </time>
                    <h3 class="thumb-title">{ post.title }</h3>
                </div>
            </article>
        </Link<Route>>
    }
}

// 3. Home Page Component (Masthead + 3-Column Field Notes Grid)
#[function_component(HomePage)]
fn home_page() -> Html {
    html! {
        <div class="page-container">
            <header class="masthead">
                <div class="masthead-stamp">
                    <span>{ "№ 01" }</span>
                    <span class="stamp-divider">{ "•" }</span>
                    <span>{ "DISPATCHES & FIELD LOGS" }</span>
                    <span class="stamp-divider">{ "•" }</span>
                    <span>{ "AUTUMN 2026" }</span>
                </div>
                <h1 class="journal-title">{ "Guilherme Silva Accioli" }</h1>
                <p class="journal-tagline">
                    { "An open laboratory journal documenting explorations in Rust, WebAssembly, systems programming, and client-side engineering." }
                </p>
                <div class="masthead-actions">
                    <Link<Route> to={Route::Blog} classes="btn btn-primary">{ "Browse Field Notes →" }</Link<Route>>
                    <Link<Route> to={Route::About} classes="btn btn-secondary">{ "About the Researcher" }</Link<Route>>
                </div>
            </header>

            <section class="latest-posts-section">
                <div class="section-title-row">
                    <div>
                        <span class="section-kicker">{ "INDEX ARCHIVE" }</span>
                        <h2>{ "Recent Dispatches" }</h2>
                    </div>
                    <span class="section-subtitle">{ "Click any plate to examine study" }</span>
                </div>
                <div class="thumbnail-grid">
                    {
                        POSTS.iter().map(|post| {
                            html! {
                                <PostCard post={post.clone()} key={post.id} />
                            }
                        }).collect::<Html>()
                    }
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
                <span class="section-kicker">{ "FIELD LOGBOOK" }</span>
                <h2>{ "Complete Archive of Dispatches" }</h2>
                <p>{ "Chronological index of experiments, architecture notes, and software studies." }</p>
            </header>
            <div class="thumbnail-grid">
                {
                    POSTS.iter().map(|post| {
                        html! {
                            <PostCard post={post.clone()} key={post.id} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}

// 5. Single Blog Post Page Component (Renders Markdown from Individual File)
#[derive(Properties, PartialEq)]
struct PostPageProps {
    id: String,
}

#[function_component(PostPage)]
fn post_page(props: &PostPageProps) -> Html {
    let post = POSTS.iter().find(|p| p.id == props.id);

    html! {
        <div class="page-container">
            <Link<Route> to={Route::Home} classes="back-link">{ "← Back to Field Index" }</Link<Route>>
            {
                if let Some(post) = post {
                    html! {
                        <article class="post-detail">
                            <div class="detail-header">
                                <div class="detail-meta">
                                    <span class="specimen-badge">{ post.log_no }</span>
                                    <time class="post-date">{ post.date }</time>
                                </div>
                                <h1 class="detail-title">{ post.title }</h1>
                            </div>
                            <div class="post-hero-image">
                                <img src={post.thumbnail} alt={post.title} />
                            </div>
                            <div class="post-content">
                                <p class="post-lead">{ post.summary }</p>
                                <hr class="entry-divider" />
                                <div class="markdown-body">
                                    { render_markdown(post.content_md) }
                                </div>
                            </div>
                        </article>
                    }
                } else {
                    html! {
                        <div class="not-found-card">
                            <span class="section-kicker">{ "TERRA INCOGNITA" }</span>
                            <h2>{ "Log Entry Not Found" }</h2>
                            <p>{ format!("No dispatch with catalog code \"{}\" exists in the registry.", props.id) }</p>
                            <Link<Route> to={Route::Home} classes="btn btn-primary">{ "Return to Index" }</Link<Route>>
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
                <span class="section-kicker">{ "FIELD STATION" }</span>
                <h2>{ "About the Researcher" }</h2>
            </header>
            <div class="about-card">
                <div class="about-header">
                    <span class="avatar-seal">{ "🧭" }</span>
                    <div>
                        <h3 class="about-name">{ "Guilherme Silva Accioli" }</h3>
                        <span class="about-role">{ "Systems Developer & WebAssembly Explorer" }</span>
                    </div>
                </div>
                <p class="about-bio">
                    { "This field journal documents investigations into compiled frontend software, high-performance web tooling, and resilient application architectures built with Rust and WebAssembly." }
                </p>
                <div class="specimen-tags-section">
                    <span class="tags-label">{ "RESEARCH DOMAINS & TOOLKIT:" }</span>
                    <div class="skills-list">
                        <span class="skill-tag">{ "Rust Language" }</span>
                        <span class="skill-tag">{ "WebAssembly" }</span>
                        <span class="skill-tag">{ "Yew Framework" }</span>
                        <span class="skill-tag">{ "Trunk Bundler" }</span>
                        <span class="skill-tag">{ "GitHub Pages" }</span>
                        <span class="skill-tag">{ "Systems Architecture" }</span>
                    </div>
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
            <span class="section-kicker">{ "CHART ERROR 404" }</span>
            <h1>{ "Uncharted Territory" }</h1>
            <p>{ "The requested dispatch or coordinate could not be found in the current logbook." }</p>
            <Link<Route> to={Route::Home} classes="btn btn-primary">{ "Return to Base Index" }</Link<Route>>
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
                            <span class="logo-compass">{ "🧭" }</span>
                            <div class="logo-text">
                                <span class="logo-main">{ "G. SILVA ACCIOLI" }</span>
                                <span class="logo-sub">{ "FIELD NOTES · EST. 2026" }</span>
                            </div>
                        </Link<Route>>
                        <div class="nav-links">
                            <Link<Route> to={Route::Home} classes="nav-item">{ "Index" }</Link<Route>>
                            <Link<Route> to={Route::Blog} classes="nav-item">{ "Field Notes" }</Link<Route>>
                            <Link<Route> to={Route::About} classes="nav-item">{ "About" }</Link<Route>>
                        </div>
                    </div>
                </nav>

                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>

                <footer class="footer">
                    <div class="footer-content">
                        <p class="footer-quote">{ "“All models are wrong, but some are useful.”" }</p>
                        <p class="footer-meta">{ "Written in Rust · Rendered in WebAssembly via Yew · Dispatched from GitHub Pages" }</p>
                    </div>
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
