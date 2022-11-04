use perseus::Template;
use sycamore::prelude::{view, Html, SsrNode, View};

#[perseus::template_rx]
pub fn about_page() -> View<G> {
    view! {
        div(class="font-normal mx-auto mt-5 max-w-xl sm:flex sm:justify-center md:mt-8") {
            p(class="bg-primary-400", id="my-paragraph", aria-label="About Paragraph") { "About this app." }
        }
        div(class="font-normal mx-auto mt-5 max-w-xl sm:flex sm:justify-center md:mt-8") {
            a(href = "", type="button", id = "home-link", class="pointer-events-auto mt-8 rounded-md bg-primary-400 py-2 px-3 hover:text-gray hover:bg-secondary-700" ) { "Home!" }
        }               
    }
}

#[perseus::head]
pub fn head() -> View<SsrNode> {
    view! {
        title { "About Page | Perseus Example – Basic" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}