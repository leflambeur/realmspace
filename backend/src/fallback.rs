use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode, Uri},
    response::IntoResponse,
};
use leptos::prelude::*;
use leptos_axum::render_app_to_stream;
use std::path::Path;
use tower::ServiceExt;
use tower_http::services::ServeDir;

/// Handles requests by serving static files or rendering the Leptos app.
pub async fn fallback(
    uri: Uri,
    State(_options): State<LeptosOptions>, // Options are used only for static file handling
    req: Request<Body>,
) -> impl IntoResponse {
    // Convert the `Arc<str>` site_root to `&Path`
    let site_root = Path::new(&*_options.site_root);

    // Attempt to serve a static file
    match ServeDir::new(site_root).oneshot(req).await {
        Ok(res) if res.status() == StatusCode::OK => res.into_response(),
        _ => {
            // If the file is not found, recreate the request for the fallback
            let fallback_req = Request::builder()
                .uri(uri)
                .body(Body::empty())
                .unwrap();

            // Render the Leptos app
            let app_handler = render_app_to_stream(|| {
                view! {
                    <h1>"404: Page Not Found"</h1>
                }
            });

            app_handler(fallback_req).await
        }
    }
}
