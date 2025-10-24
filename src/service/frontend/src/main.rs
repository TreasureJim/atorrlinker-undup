mod pages;

use yew::prelude::*;

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
            <pages::home::QueuedBooks/>
        </main>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
