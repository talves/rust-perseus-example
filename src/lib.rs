mod error_pages;
mod templates;

use perseus::{Html, PerseusApp, PerseusRoot};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .index_view(|| {
            sycamore::view! {
                // We don't need a `<!DOCTYPE html>`, that's added automatically by Perseus (though that can be overriden if you really want by using `.index_view_str()`)
                // We need a `<head>` and a `<body>` at the absolute minimum for Perseus to work properly (otherwise certain script injections will fail)
                head {
                    title { "Perseus Index View Example" }
                    link(rel ="stylesheet", href=".perseus/static/tailwind.css")
                }
                body {
                    // This creates an element into which our app will be interpolated
                    // This uses a few tricks internally beyond the classic `<div id="root">`, so we use this wrapper for convenience
                        div(class="min-h-screen") {
                            div(class="mx-auto h-4/5 w-auto px-4 pt-16 pb-8 sm:pt-24 lg:px-8") {
                                h1(class="mx-auto max-w-5xl text-center text-6xl font-extrabold leading-[1.1] text-gray-800 dark:text-gray-900 tracking-tighter text-white sm:text-7xl lg:text-8xl xl:text-8xl") {
                                span(class="inline-block bg-gradient-to-r from-primary-500 to-secondary-500 bg-clip-text text-transparent") {
                                    "Perseus & Tailwind Example" 
                                }
                            }
                        }
                        PerseusRoot()
                        // Because this is in the index view, this will be below every single one of our pages
                        // Note that elements in here can't be selectively removed from one page, it's all-or-nothing in the index view (it wraps your whole app)
                        // Note also that this won't be reloaded, even when the user switches pages
                        footer { "This is a footer!" }
                    }
                }
            }
        })
}
