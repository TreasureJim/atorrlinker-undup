mod pages;
pub mod config;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    // TODO
    // #[not_found]
    // #[at("/404")]
    // NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <pages::home::HomePage /> }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="light">
            <Header />
            <MainContent />
        </div>
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header class="header">
            <div class="container">
                <img 
                    src="images/icon.jpg" 
                    class="logo"
                    alt="Audiobook Torrent Linker Logo"
                    width=90px height=90px
                />
                <h1 class="title">
                    { "Audiobook Torrent Linker" }
                </h1>
                <nav class="header__nav">
                    // Add navigation items here if needed
                </nav>
            </div>
        </header>
    }
}

#[function_component(MainContent)]
fn main_content() -> Html {
    html! {
        <main class="main light">
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </main>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
