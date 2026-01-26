use axum::{routing::get, Router, response::Redirect};
use axum::response::IntoResponse;
use axum::http::{StatusCode, header};
use tower_service::Service;
use worker::*;
use axum::response::Html;
use include_dir::{include_dir, Dir};
use askama_axum::Template;
use std::collections::HashMap;
use comrak::{markdown_to_html, ComrakOptions};


// +-------------+
// | Git Version |
// +-------------+
const GIT_HASH: &str = env!("GIT_HASH");
const GIT_BRANCH: &str = env!("GIT_BRANCH");
const CURRENT_YEAR: &str = env!("CURRENT_YEAR");


// +----------------+
// | Template logic |
// +----------------+
mod dps_sims; 
mod mythic_plus;
mod player_metadata;
mod about_data;

// +---------------+
// | Static Assets |
// +---------------+

// Include html, css, and and media in local repo.
static ASSETS_DIR: Dir = include_dir!("templates");
// R2 Endpoints for dynamic content
// TODO: make generic handler for /content/
const EVENTS_JSON_URL: &str = "https://r2.seemsgood.org/content/events.json";
const PROGRESS_JSON_URL: &str = "https://r2.seemsgood.org/content/progress.json";
const RAIDER_EXPECTATIONS_URL: &str = "https://r2.seemsgood.org/content/md/raider-expectations.md";

// All routes for webpage that are not dynamic.
fn router() -> Router {
    Router::new() 
        .route("/", get(home_page))
        .route("/about", get(about_page))
        .route("/application", get(apply_page))
        .route("/dps-sims", get(dps_sims::damagesimspage))
        .route("/keys",  get(mythic_plus::mythicplus_page))
        .route("/wowaudit", get(wowaudit_page))
        .route("/talents", get(talents_page))
        .route("/resources", get(resources_page))
        .route("/css/bulma.min.css", get(bulma_css_handler))
        .fallback(Redirect::permanent("/"))
}

// +-------------------+
// | Worker Entrypoint |
// +-------------------+

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    
    // Handle /events and /progress routes manually before passing to router
    let path = req.uri().path();
   
    if path == "/progress" {
        return Ok(fetch_json_endpoint(PROGRESS_JSON_URL, "assets/progress.json").await);
    }
    if path == "/events" {
        return Ok(fetch_json_endpoint(EVENTS_JSON_URL, "assets/events.json").await);
    }
    // Handle /expectations (gh url)
    if path == "/expectations" {
        return Ok(fetch_html_endpoint(RAIDER_EXPECTATIONS_URL, "assets/404.html").await);
    }

    // For all other routes, use the router
    Ok(router().call(req).await?)
}

// +----------------------------+
// | Markdown Extension Options | (striketrough table etc..)
// +----------------------------+

/// use in functions that call ComrakOptions::default()
/// keeps extensions same, less repeated code.
/// example:
/// ```rust
/// let mut options = ComrakOptions::default(); 
/// enable_extensions(&mut options);
/// ```
fn enable_extensions(options: &mut ComrakOptions) {
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.shortcodes = true;
    options.extension.underline = true;
    options.extension.description_lists = true;
    options.extension.greentext = true;
    options.extension.superscript = true;
    options.extension.subscript = true;
    options.extension.spoiler = true;
}

// +------------------------------+
// | Handlers for Dynamic Content |
// +------------------------------+

// Fetch HTML from $url. Fallback on local file. 
// Request  | /expectations -> fetch_html_endpoint -> |MARKDOWN.md| 
// Response | /expectations <- markdown_to_html    <- |MARKDOWN.md|
async fn fetch_html_endpoint(url: &str, fallback_path: &str) -> axum::http::Response<axum::body::Body> {

    let html_data = match fetch_html_from_whitelist(url).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching HTML from {}: {}", url, e);
            match ASSETS_DIR.get_file(fallback_path) {
                Some(file) => file.contents_utf8().unwrap_or("").to_string(),
                None => "<p>Content unavailable.</p>".to_string(),
            }
        }
    };
    let mut options = ComrakOptions::default(); 
    enable_extensions(&mut options);
    let markdown = format!(
        r#"<div class="markdown-body">{}</div>"#,
        markdown_to_html(&html_data, &options)
    );

    (
        [
        (header::CONTENT_TYPE, "text/html; charset=utf-8"),
        (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        ],
        markdown,
    )
        .into_response()
}

// Protect against arbitrary HTML only allow scoped URLs
async fn fetch_html_from_whitelist(url: &str) -> Result<String, String> {
    if url != RAIDER_EXPECTATIONS_URL {
        console_log!(
            "SECURITY: Blocked attempt to fetch non-whitelisted HTML URL: {}",
            url
        );
        return Err("URL not whitelisted".into());
    }

    let mut request_init = RequestInit::new();
    request_init.with_method(Method::Get);

    let request = Request::new_with_init(url, &request_init)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;

    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch HTML: {:?}", e))?;

    let status = response.status_code();
    if status < 200 || status >= 300 {
        return Err(format!("HTML request failed with status: {}", status));
    }

    response
        .text()
        .await
        .map_err(|e| format!("Failed to read HTML response: {:?}", e))
}


// Handler for ../templates/css/bulma.min.css 
async fn bulma_css_handler() -> axum::http::Response<axum::body::Body> {
    match ASSETS_DIR.get_file("css/bulma.min.css") {
        Some(file) => {
            let body = file.contents_utf8().unwrap_or("").to_string();
            (
                [(header::CONTENT_TYPE, "text/css")],
                body
            ).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            "Bulma CSS file not found".to_string()
        ).into_response()
    }
}

// Generic to fetch JSON from endpoint with fallback
async fn fetch_json_endpoint(url: &str, fallback_path: &str) -> axum::http::Response<axum::body::Body> {
    // Try to fetch from API
    let json_data = match fetch_from_r2(url).await {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching from {}: {}", url, e);
            // Fallback to static file
            match ASSETS_DIR.get_file(fallback_path) {
                Some(file) => file.contents_utf8().unwrap_or("{}").to_string(),
                None => "{}".to_string(),
            }
        }
    };
    
    (
        [
            (header::CONTENT_TYPE, "application/json"),
            (header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
        ],
        json_data
    ).into_response()
}

// Helper to fetch JSON from R2
async fn fetch_from_r2(url: &str) -> std::result::Result<String, String> {
    // Verify input of url here. 
    if url != EVENTS_JSON_URL && url != PROGRESS_JSON_URL {
        console_log!("SECURITY: Blocked attempt to fetch from non-whitelisted URL: {}", url);
        return Err(format!("URL not whitelisted: {}", url));
    }
    
    let mut request_init = RequestInit::new();
    request_init.with_method(Method::Get);
    
    let request = Request::new_with_init(url, &request_init)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;
    
    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch: {:?}", e))?;
    
    let status = response.status_code();
    if status < 200 || status >= 300 {
        return Err(format!("Request failed with status: {}", status));
    }
    
    response
        .text()
        .await
        .map_err(|e| format!("Failed to read response text: {:?}", e))
}


// +---------------+
// | Base Template |
// +---------------+
// Define a base struct once for noti and git branch
// Base includes Navbar, Footer, Theme, CSS imports, and other reused assets.
#[derive(Clone)]
pub struct GitInfo {
    pub hash: &'static str,
    pub branch: &'static str,
}

impl GitInfo {
    pub fn current() -> Self {
        Self {
            hash: GIT_HASH,
            branch: GIT_BRANCH,
        }
    }
}

// fix static typed date in layout.html 
// get current year at build time.
#[derive(Clone)]
pub struct DateInfo {
    pub date: &'static str,
}

impl DateInfo {
    pub fn current() -> Self {
        Self {
            date: CURRENT_YEAR,
        }
    }
}

#[derive(Clone)]
pub struct BaseTemplate {
    pub show_noti: bool,
    pub git: GitInfo,
    pub date: DateInfo,
}

impl BaseTemplate {
    pub fn new(show_noti: bool) -> Self {
        Self {
            show_noti,
            git: GitInfo::current(),
            date: DateInfo::current(),
        }
    }
}



// +---------------------------+
// | Build Pages from Templates|
// +---------------------------+

// Home Page
use player_metadata::{build_roster, Player, build_raid, RaidMetaData};
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    base: BaseTemplate,
    raid_metadata: Vec<RaidMetaData>,
    rosters: HashMap<String, Vec<Player>>, 
}
async fn home_page() -> Html<String> {
    let mut rosters = HashMap::new();
    rosters.insert("Dimensius".to_string(), build_roster("Dimensius"));
    rosters.insert("Gallywix".to_string(), build_roster("Gallywix"));
    rosters.insert("Kyvesa".to_string(), build_roster("Kyvesa"));
    rosters.insert("Fyrakk".to_string(), build_roster("Fyrakk"));

    let raid_metadata = build_raid();
    let template = IndexTemplate { 
        base: BaseTemplate::new(true),
        raid_metadata,
        rosters,
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}

// Apply Page 
#[derive(Template)]
#[template(path = "apply.html")]
struct ApplyTemplate {
    base: BaseTemplate,

}
async fn apply_page() -> Html<String> {
    let template = ApplyTemplate { 
        base: BaseTemplate::new(false)
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}

// About Page
use about_data::{ContactInfo, build_contacts};
#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    base: BaseTemplate,
    contacts: Vec<ContactInfo>,
}
async fn about_page() -> Html<String> {
    let contacts = build_contacts();
    let template = AboutTemplate { 
        base: BaseTemplate::new(true),
        contacts,
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}

// Spreadsheet Page (wowaudit)
// TODO link to /resources
#[derive(Template)]
#[template(path = "wowaudit.html")]
struct WowauditTemplate {
    base: BaseTemplate,
}
async fn wowaudit_page() -> Html<String> {
    let template = WowauditTemplate { 
        base: BaseTemplate::new(true)
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}

// Talents Page (talents.seemsgood.org)
#[derive(Template)]
#[template(path = "talents.html")]
struct TalentsTemplate {
    base: BaseTemplate,
}
async fn talents_page() -> Html<String> {
    let template = TalentsTemplate { 
        base: BaseTemplate::new(true)
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}

// Resources Page (Raider expectations, loot management, trial process, and raid schedule.)
#[derive(Template)]
#[template(path = "resources.html")]
struct ResourcesTemplate{
    base: BaseTemplate,
}
async fn resources_page() -> Html<String> {
    let template = ResourcesTemplate { 
        base: BaseTemplate::new(true)
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}

