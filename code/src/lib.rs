mod info;
mod overview;
mod not_found;
mod footer;
mod docs;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use info::Info;
use overview::Overview;
use docs::Docs;
use not_found::NotFound;
use footer::MainFooter;
use pbs::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/leche.github.io/info")]
    Info,

    #[at("/leche.github.io/docs")]
    Docs,

    #[at("/leche.github.io/")]
    Overview,

    #[not_found]
    #[at("/404")]
    NotFound,
}

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <div class="columns is-flex-direction-column" style="height: 100vh">
                    <Section extra="column">
                        <Container>
                            <Router<Route> render={Router::render(switch)} />
                        </Container>
                    </Section>
                    <MainFooter />
                </div>
            </main>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Info => html!{ <Info /> },
        Route::Docs => html!{ <Docs /> },
        Route::Overview => html!{ <Overview /> },
        Route::NotFound => html!{ <NotFound /> },
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
    Ok(())
}
