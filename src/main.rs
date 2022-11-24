fn main() {}

use axum::{
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        HeaderValue,
    },
    response::IntoResponse,
    routing::get,
    Router,
};

pub fn routes() -> Router {
    let mut router = Router::new();

    macro_rules! static_asset {
        ($file:expr) => {
            router = router.route(
                concat!("/", $file),
                get(|| {
                    const CONTENT_TYPE: HeaderValue = content_type($file);
                    handler(CONTENT_TYPE, include_bytes!(concat!("../static/", $file)))
                }),
            );
        };
    }

    // Roboto
    static_asset!("fonts/roboto/LICENSE.txt");
    static_asset!("fonts/roboto/bold/normal/roboto-bold-normal.cyrillic-ext.woff2");
    static_asset!("fonts/roboto/bold/normal/roboto-bold-normal.cyrillic.woff2");
    static_asset!("fonts/roboto/bold/normal/roboto-bold-normal.devanagari.woff2");
    static_asset!("fonts/roboto/bold/normal/roboto-bold-normal.greek-ext.woff2");
    static_asset!("fonts/roboto/bold/normal/roboto-bold-normal.greek.woff2");
    static_asset!("fonts/roboto/bold/normal/roboto-bold-normal.latin-ext.woff2");
    static_asset!("fonts/roboto/bold/normal/roboto-bold-normal.latin.woff2");
    static_asset!("fonts/roboto/bold/normal/roboto-bold-normal.vietnamese.woff2");
    static_asset!("fonts/roboto/light/normal/roboto-light-normal.cyrillic-ext.woff2");
    static_asset!("fonts/roboto/light/normal/roboto-light-normal.cyrillic.woff2");
    static_asset!("fonts/roboto/light/normal/roboto-light-normal.devanagari.woff2");
    static_asset!("fonts/roboto/light/normal/roboto-light-normal.greek-ext.woff2");
    static_asset!("fonts/roboto/light/normal/roboto-light-normal.greek.woff2");
    static_asset!("fonts/roboto/light/normal/roboto-light-normal.latin-ext.woff2");
    static_asset!("fonts/roboto/light/normal/roboto-light-normal.latin.woff2");
    static_asset!("fonts/roboto/light/normal/roboto-light-normal.vietnamese.woff2");
    static_asset!("fonts/roboto/medium/normal/roboto-medium-normal.cyrillic-ext.woff2");
    static_asset!("fonts/roboto/medium/normal/roboto-medium-normal.cyrillic.woff2");
    static_asset!("fonts/roboto/medium/normal/roboto-medium-normal.devanagari.woff2");
    static_asset!("fonts/roboto/medium/normal/roboto-medium-normal.greek-ext.woff2");
    static_asset!("fonts/roboto/medium/normal/roboto-medium-normal.greek.woff2");
    static_asset!("fonts/roboto/medium/normal/roboto-medium-normal.latin-ext.woff2");
    static_asset!("fonts/roboto/medium/normal/roboto-medium-normal.latin.woff2");
    static_asset!("fonts/roboto/medium/normal/roboto-medium-normal.vietnamese.woff2");
    static_asset!("fonts/roboto/regular/normal/roboto-regular-normal.cyrillic-ext.woff2");
    static_asset!("fonts/roboto/regular/normal/roboto-regular-normal.cyrillic.woff2");
    static_asset!("fonts/roboto/regular/normal/roboto-regular-normal.devanagari.woff2");
    static_asset!("fonts/roboto/regular/normal/roboto-regular-normal.greek-ext.woff2");
    static_asset!("fonts/roboto/regular/normal/roboto-regular-normal.greek.woff2");
    static_asset!("fonts/roboto/regular/normal/roboto-regular-normal.latin-ext.woff2");
    static_asset!("fonts/roboto/regular/normal/roboto-regular-normal.latin.woff2");
    static_asset!("fonts/roboto/regular/normal/roboto-regular-normal.vietnamese.woff2");

    router
}

async fn handler(content_type: HeaderValue, data: &'static [u8]) -> impl IntoResponse {
    data
}

const fn content_type(filename: &str) -> HeaderValue {
    HeaderValue::from_static("application/octet-stream")
}
