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

#[derive(PartialEq, Clone, Debug)]
pub enum Page {
    Info,
    Docs,
    Overview,
}

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub struct Model {
    link: ComponentLink<Self>,
    page: Page,
}

impl Component for Model {
    type Message = Page;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, page: Page::Overview }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let page = match self.page {
            Page::Info => html!{ <Info /> },
            Page::Docs => html!{ <Docs /> },
            Page::Overview => html!{ <Overview /> },
        };

        html! {
            <main>
                <div class="columns is-flex-direction-column" style="height: 100vh">
                    <Section extra="column">
                        <Container>
                            { page }
                        </Container>
                    </Section>
                    <MainFooter onpage={self.link.callback(|x| x)}/>
                </div>
            </main>
        }
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
    Ok(())
}
