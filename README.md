# Seems Good Webpage Repository
## Local Setup Prereqs/Dependencies:
- [install rust](https://rustup.rs/)
- install wasm target system
    `rustup install wasm32-unknown-unknown`
- [install npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- install npx package:
    `npm install -g npx`
- clone the repo:
    `git clone https://github.com/Jeremy-Gstein/seemsgood_guild.git`
- setup local server:
    `npx wrangler dev`
- deploy local to cloudflare cdn:
    `npx wrangler deploy`

### List of Resources and other software used:
- [Render HTML file with Axum](https://github.com/programatik29/axum-tutorial)
- [Cloudflare Worker Docs](https://developers.cloudflare.com/workers/)
- [Askama Templating Docs](https://djc.github.io/askama/)
- Bulma (css)
- Axum (routing, handling http)
- Cloudflare workers

## Helper Scripts:
helpers/ contains 4 bash scripts that help with formatting the roster, getting latest mythic plus, and raid progress.
The current Implementation uses a local raspberry pi to run `./helpers/helpers_main.sh` daily via a systemd service and a small docker container with curl+jq Dependencies. 

For testing/running locally you will need a helpers/.env file with the following variables:
```bash
cat << EOF > helpers/.env
File: helpers/.env
# --- Required for ./wcl-progress.sh.sh ---
File: helpers/.env
# --- Required for ./wcl-progress.sh.sh ---
WCL_CLIENT_ID=
WCL_CLIENT_SECRET=
WCL_GUILD_ID=
WCL_ZONE_ID=

# --- Required for ./json_helpers.sh ---
WOWAUDIT_TOKEN=

# --- Required for ./handle_r2_upload.sh ---
R2_ACCOUNT_ID=
R2_ACCESS_KEY_ID=  
R2_SECRET_ACCESS_KEY=
R2_BUCKET_NAME=
# --- Optional ---
R2_DESTINATION_DIR=content

EOF
```

## Contributing:
- Step 1. Create a route in ./src/lib.rs under router following the others scheme.
```rust
    Router::new() // created under this method
        // ...
        .route("/your_page_name", get(your_page))
        // ...
```
- Step 2. Create a Template struct and page function in ./src/lib.rs
```rust
#[derive(Template)]
#[template(path = "your_page.html")]
struct YourPageTemplate {
    base: BaseTemplate, // base UI (nav bar, footer, theme, and main css file)
    // any other parameters needed (not including noti will fail during build)
}
async fn your_page() -> Html<String> {
    let t = YourPageTemplate {
        base: BaseTemplate::new(true), // set the notification banner to true to display on page. (guild application promo)
    };
    let r = t.render().unwrap();
    Html(r)
}
```
- Step 3. For backend logic that is not html or js related. use a seperate file in ./src and load as a module
- Step 4. Create a html file under ./templates/ and extend the ui with askama (keeps navbar and footer and loads css)
```html
{% extends layhout.html %} 
    <!-- Navbar shown here. css and js from layout.html loaded here -->
{% block content %}
    <!-- your layout/ui content here -->
{% include "assets/$your-content.html" %}
    <!-- your html, js, css here (in a seperate file located in ./assets) -->
{% endblock %}
    <!-- Footer from layout.html shown here -->
```
Feel free to get in contact and ask for any help!



made with :heart: in Vim
