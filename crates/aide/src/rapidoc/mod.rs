#[must_use]
pub struct Rapidoc {
    title: String,
    spec_url: String,
}

impl Rapidoc {
    /// Create a new [`Redoc`] wrapper with the given spec url.
    pub fn new(spec_url: impl Into<String>) -> Self {
        Self {
            title: "Rapidoc".into(),
            spec_url: spec_url.into(),
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }

    pub fn html(&self) -> String {
        format!(
            r#"
            <!doctype html> <!-- Important: must specify -->
            <html>
              <head>
                <meta charset="utf-8"> <!-- Important: rapi-doc uses utf8 characters -->
                <title>{title}</title>
                <script type="module">
                {rapidoc_js}
                </script>
              </head>
              <body>
                <rapi-doc spec-url = "{spec_url}"> </rapi-doc>
              </body>
            </html>
            <!DOCTYPE html>
            "#,
            rapidoc_js = include_str!("../../res/rapidoc/rapidoc-min.js"),
            title = self.title,
            spec_url = self.spec_url
        )
    }
}

#[cfg(feature = "axum")]
mod axum_impl {
    use crate::axum::{
        routing::{get, ApiMethodRouter},
        AxumOperationHandler,
    };
    use axum::response::Html;

    impl super::Rapidoc {
        pub fn axum_route(&self) -> ApiMethodRouter {
            get(self.axum_handler())
        }

        #[must_use]
        pub fn axum_handler<S, B>(&self) -> impl AxumOperationHandler<(), Html<String>, ((),), S, B>
        where
            B: axum::body::HttpBody + Send + 'static,
        {
            let html = self.html();
            move || async move { Html(html) }
        }
    }
}
