use crate::article::Articles;
use crate::renderer::Renderer;
use crate::site::Site;
use axum::extract::State;
use axum::{Router, extract, response::Html, routing::get};
use tower_http::services::ServeDir;
use url::Url;

#[derive(Clone)]
struct AppState {
    renderer: Renderer,
    articles: Articles,
}

#[tokio::main]
pub async fn start_server(base_url: &Url, renderer: Renderer, site: &Site) {
    let state = AppState {
        renderer,
        articles: site.articles.clone(),
    };

    let app = Router::new()
        .route("/", get(get_homepage))
        .merge(article_routes())
        .fallback_service(ServeDir::new("assets"))
        .with_state(state);

    // let app = Router::new().merge(site_router);

    let bind_address = base_url
        .socket_addrs(|| Some(3000))
        .ok()
        .and_then(|addresses| addresses.into_iter().next())
        .unwrap_or_else(|| "0.0.0.0:3000".parse().unwrap());

    let listener = tokio::net::TcpListener::bind(bind_address)
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn article_routes() -> Router<AppState> {
    Router::new().route("/posts/{article_slug}", get(get_article))
}

async fn get_homepage(State(state): State<AppState>) -> Html<String> {
    let is_production = false;
    let homepage_html = state
        .renderer
        .render_homepage(state.articles, is_production);

    Html(homepage_html)
}

async fn get_article(
    State(state): State<AppState>,
    extract::Path(article_slug): extract::Path<String>,
) -> Html<String> {
    if let Some(article) = state.articles.iter().find(|a| a.slug == article_slug) {
        Html(state.renderer.render_article(article))
    } else {
        Html("Not Found".to_string())
    }
}
