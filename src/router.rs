use crate::blog::router_blog;
use crate::pages::blog::Blog;
use crate::pages::blogs::Blogs;
use crate::pages::home::Home;
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[redirect("/", || Route::BlogPost { child: router_blog::BookRoute::AnnouncingKevinRs {} })]
    #[layout(Blog)]
    #[child("/blogs")]
    BlogPost { child: router_blog::BookRoute },
    #[end_layout]
    #[route("/blogs")]
    Blogs {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-[var(--bg-primary)] text-[var(--text-primary)] flex flex-col items-center justify-center gap-6 font-['Lexend']",
            i { class: "fa-solid fa-triangle-exclamation text-5xl text-yellow-400" }
            h1 { class: "text-4xl font-bold", "404 - Page not found" }
            p { class: "text-[var(--text-secondary)] text-lg text-center max-w-md", "We are terribly sorry, but the page you requested doesn't exist." }
            Link {
                to: Route::Home {},
                class: "px-6 py-3 rounded-lg bg-green-600 text-white font-semibold hover:bg-green-700 transition-colors",
                "← Back to Home"
            }
        }
    }
}

#[server(endpoint = "static_routes")]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>())
}
