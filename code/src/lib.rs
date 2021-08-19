mod info;
mod overview;
mod footer;
mod docs;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use info::Info;
use overview::Overview;
use docs::Docs;
use footer::MainFooter;

use pbs::*;
use yew::utils::NeqAssign;

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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.page.neq_assign(msg)
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onpage = self.link.callback(|x| x);
        let page = match self.page {
            Page::Info => html!{ <Info onpage={onpage.clone()} /> },
            Page::Docs => html!{ <Docs onpage={onpage.clone()} /> },
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
                    <MainFooter onpage={onpage}/>
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
