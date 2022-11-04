use perseus::{Html, RenderFnResultWithCause, SsrNode, Template};
use sycamore::prelude::{view, View};

#[perseus::make_rx(IndexPageStateRx)]
pub struct IndexPageState {
    pub greeting: String,
}

#[perseus::template_rx]
pub fn index_page(state: IndexPageStateRx) -> View<G> {
    view! {
        div(class="font-normal mx-auto mt-5 max-w-xl sm:flex sm:justify-center md:mt-8") {
            (state.greeting.get())
        }
        div(class="font-normal mx-auto mt-5 max-w-xl sm:flex sm:justify-center md:mt-8") {
            a(href = "about", type="button", id = "about-link", class="pointer-events-auto mt-8 rounded-md bg-primary-400 py-2 px-3 hover:text-gray hover:bg-secondary-700" ) { "About!" }
        }               
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index")
        .build_state_fn(get_build_state)
        .template(index_page)
        .head(head)
}

#[perseus::head]
pub fn head(_props: IndexPageState) -> View<SsrNode> {
    view! {
        title { "Index Page | Perseus Example – Basic" }
    }
}

#[perseus::autoserde(build_state)]
pub async fn get_build_state(
    _path: String,
    _locale: String,
) -> RenderFnResultWithCause<IndexPageState> {
    Ok(IndexPageState {
        greeting: "Hello World!".to_string(),
    })
}
