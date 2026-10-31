mod gallery;
mod mural;
mod posts;

use gallery::{GalleryItem, GALLERY_ITEMS};
use mural::MuralPage;
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
    #[at("/gallery")]
    Gallery,
    #[at("/mural")]
    Mural,
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

// 3. Gallery Card Component (Plate, Image, Quote, External Source)
#[derive(Properties, PartialEq)]
pub struct GalleryCardProps {
    pub item: GalleryItem,
    pub on_select: Callback<GalleryItem>,
}

#[function_component(GalleryCard)]
pub fn gallery_card(props: &GalleryCardProps) -> Html {
    let item = &props.item;
    let on_image_click = {
        let on_select = props.on_select.clone();
        let item = item.clone();
        Callback::from(move |_| {
            on_select.emit(item.clone());
        })
    };

    html! {
        <article class="gallery-plate-card">
            <div class="plate-top-bar">
                <span class="plate-number">{ item.plate_no }</span>
                <span class="plate-year">{ item.year }</span>
            </div>

            <div class="plate-image-frame" onclick={on_image_click}>
                <img src={item.image_url} alt={item.title} loading="lazy" />
                <div class="plate-zoom-badge">
                    <span>{ "🔍 Expand Plate" }</span>
                </div>
            </div>

            <div class="plate-details">
                <h3 class="plate-title">{ item.title }</h3>

                {
                    if let Some(quote) = item.quote {
                        html! {
                            <blockquote class="plate-quote">
                                <p class="quote-text">{ format!("“{}”", quote) }</p>
                                {
                                    if let Some(author) = item.quote_author {
                                        html! { <cite class="quote-author">{ format!("— {}", author) }</cite> }
                                    } else {
                                        html! {}
                                    }
                                }
                            </blockquote>
                        }
                    } else {
                        html! {}
                    }
                }

                {
                    if let (Some(url), Some(name)) = (item.source_url, item.source_name) {
                        html! {
                            <div class="plate-footer">
                                <a href={url} target="_blank" rel="noopener noreferrer" class="plate-source-link">
                                    <span class="source-arrow">{ "↗" }</span>
                                    <span>{ "Source: " }</span>
                                    <span class="source-name">{ name }</span>
                                </a>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </article>
    }
}

// 4. Home Page Component (Masthead + Posts + Gallery Preview)
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
                    <Link<Route> to={Route::Gallery} classes="btn btn-secondary">{ "View Image Gallery" }</Link<Route>>
                </div>
            </header>

            <section class="latest-posts-section">
                <div class="section-title-row">
                    <div>
                        <span class="section-kicker">{ "INDEX ARCHIVE" }</span>
                        <h2>{ "Recent Dispatches" }</h2>
                    </div>
                    <Link<Route> to={Route::Blog} classes="section-subtitle-link">{ "All Notes →" }</Link<Route>>
                </div>
                <div class="thumbnail-grid">
                    {
                        POSTS.iter().take(3).map(|post| {
                            html! {
                                <PostCard post={post.clone()} key={post.id} />
                            }
                        }).collect::<Html>()
                    }
                </div>
            </section>

            <section class="gallery-home-section">
                <div class="section-title-row">
                    <div>
                        <span class="section-kicker">{ "VISUAL ATLAS" }</span>
                        <h2>{ "Featured Plates & Quotes" }</h2>
                    </div>
                    <Link<Route> to={Route::Gallery} classes="section-subtitle-link">{ "Open Gallery →" }</Link<Route>>
                </div>
                <div class="gallery-home-grid">
                    {
                        GALLERY_ITEMS.iter().take(3).map(|item| {
                            html! {
                                <div class="gallery-home-card" key={item.id}>
                                    <div class="gallery-home-thumb">
                                        <img src={item.image_url} alt={item.title} loading="lazy" />
                                        <span class="specimen-tag">{ item.plate_no }</span>
                                    </div>
                                    <div class="gallery-home-content">
                                        <h4>{ item.title }</h4>
                                        {
                                            if let Some(quote) = item.quote {
                                                html! { <p class="gallery-home-quote">{ format!("“{}”", quote) }</p> }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </section>
        </div>
    }
}

// 5. Blog List Page Component
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

// 6. Gallery Page Component (With Lightbox Modal)
#[function_component(GalleryPage)]
fn gallery_page() -> Html {
    let selected_item = use_state(|| None::<GalleryItem>);

    let on_select = {
        let selected_item = selected_item.clone();
        Callback::from(move |item: GalleryItem| {
            selected_item.set(Some(item));
        })
    };

    let on_close = {
        let selected_item = selected_item.clone();
        Callback::from(move |_| {
            selected_item.set(None);
        })
    };

    html! {
        <div class="page-container">
            <header class="section-header">
                <span class="section-kicker">{ "VISUAL ATLAS & ARCHIVES" }</span>
                <h2>{ "Field Plates & Quotations" }</h2>
                <p>{ "A curated exhibition of visual specimens, historical computing plates, and reflections." }</p>
            </header>

            <div class="gallery-grid">
                {
                    GALLERY_ITEMS.iter().map(|item| {
                        html! {
                            <GalleryCard item={item.clone()} on_select={on_select.clone()} key={item.id} />
                        }
                    }).collect::<Html>()
                }
            </div>

            {
                if let Some(item) = (*selected_item).clone() {
                    html! {
                        <div class="lightbox-overlay" onclick={on_close.clone()}>
                            <div class="lightbox-modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                                <button class="lightbox-close-btn" onclick={on_close}>{ "✕" }</button>
                                <div class="lightbox-image-container">
                                    <img src={item.image_url} alt={item.title} />
                                </div>
                                <div class="lightbox-caption">
                                    <span class="lightbox-plate">{ format!("{} • {}", item.plate_no, item.year) }</span>
                                    <h3>{ item.title }</h3>
                                    {
                                        if let Some(quote) = item.quote {
                                            html! {
                                                <blockquote class="lightbox-quote">
                                                    <p>{ format!("“{}”", quote) }</p>
                                                    {
                                                        if let Some(author) = item.quote_author {
                                                            html! { <cite>{ format!("— {}", author) }</cite> }
                                                        } else {
                                                            html! {}
                                                        }
                                                    }
                                                </blockquote>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    {
                                        if let (Some(url), Some(name)) = (item.source_url, item.source_name) {
                                            html! {
                                                <a href={url} target="_blank" rel="noopener noreferrer" class="lightbox-source-link">
                                                    <span>{ "↗ View Original Source: " }</span>
                                                    <strong>{ name }</strong>
                                                </a>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

// 7. Comments Component (Giscus / GitHub Discussions)
#[derive(Properties, PartialEq)]
pub struct CommentsProps {
    pub post_id: String,
}

#[function_component(Comments)]
pub fn comments(props: &CommentsProps) -> Html {
    let container_ref = use_node_ref();

    {
        let container_ref = container_ref.clone();
        let post_id = props.post_id.clone();
        use_effect_with(post_id, move |id| {
            if let Some(container) = container_ref.cast::<web_sys::HtmlElement>() {
                container.set_inner_html("");
                let document = gloo_utils::document();
                if let Ok(script) = document.create_element("script") {
                    let _ = script.set_attribute("src", "https://giscus.app/client.js");
                    let _ = script.set_attribute("data-repo", "guilherme-silva-accioli/guilherme-silva-accioli.github.io");
                    let _ = script.set_attribute("data-repo-id", "R_kgDOVcg5fQ");
                    let _ = script.set_attribute("data-category", "General");
                    let _ = script.set_attribute("data-category-id", "DIC_kwDOVcg5fc4DKDGk");
                    let _ = script.set_attribute("data-mapping", "specific");
                    let _ = script.set_attribute("data-term", id);
                    let _ = script.set_attribute("data-strict", "0");
                    let _ = script.set_attribute("data-reactions-enabled", "1");
                    let _ = script.set_attribute("data-emit-metadata", "0");
                    let _ = script.set_attribute("data-input-position", "top");
                    let _ = script.set_attribute("data-theme", "light");
                    let _ = script.set_attribute("data-lang", "en");
                    let _ = script.set_attribute("crossorigin", "anonymous");
                    let _ = script.set_attribute("async", "true");
                    let _ = container.append_child(&script);
                }
            }
            || ()
        });
    }

    html! {
        <section class="comments-section">
            <div class="comments-header">
                <span class="section-kicker">{ "COMMUNITY OBSERVATIONS" }</span>
                <h3 class="comments-title">{ "Field Dispatches & Discussion" }</h3>
                <p class="comments-subtitle">
                    { "Contributions and notes from fellow researchers. Authenticate with GitHub to record an observation." }
                </p>
            </div>
            <div ref={container_ref} class="comments-widget-container"></div>
        </section>
    }
}

// 8. Single Blog Post Page Component
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
                            <hr class="comments-entry-divider" />
                            <Comments post_id={post.id.to_string()} />
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

// 9. About Page Component
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

// 10. 404 Not Found Page Component
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

// 11. Router Switcher
fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Blog => html! { <BlogPage /> },
        Route::Post { id } => html! { <PostPage {id} /> },
        Route::Gallery => html! { <GalleryPage /> },
        Route::Mural => html! { <MuralPage /> },
        Route::About => html! { <AboutPage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

// 12. Root App Component with Navbar and Footer
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
                            <Link<Route> to={Route::Gallery} classes="nav-item">{ "Gallery" }</Link<Route>>
                            <Link<Route> to={Route::Mural} classes="nav-item">{ "Mural" }</Link<Route>>
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
