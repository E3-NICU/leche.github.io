use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::utils::NeqAssign;

use docs::Docs;
use footer::MainFooter;
use info::Info;
use overview::Overview;
use pbs::*;
use yew::web_sys::window;

mod info;
mod overview;
mod footer;
mod docs;
mod model;
mod boxes;

#[derive(PartialEq, Clone, Debug)]
pub enum Page {
    Info,
    Docs,
    Overview,
}

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub enum Msg {
    Page(Page),
    External(String)
}

pub struct Model {
    link: ComponentLink<Self>,
    page: Page,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, page: Page::Overview }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Page(page) => self.page.neq_assign(page),
            Msg::External(url) => {
                let _ = window().unwrap().location().set_href(&url);
                false
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onpage = self.link.callback(Msg::Page);

        let page = match self.page {
            Page::Info => html! { <Info onpage={onpage.clone()} /> },
            Page::Docs => html! { <Docs onpage={onpage.clone()} /> },
            Page::Overview => html! { <Overview /> },
        };

        html! {
            <main>
                <div class="columns is-flex-direction-column" style="height: 100vh">
                    <Section extra="column">
                        <Container>
                            { page }
                        </Container>
                    </Section>
                    <MainFooter onpage={onpage} onexternal={self.link.callback(Msg::External)}/>
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
