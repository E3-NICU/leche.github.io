use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::window;

use components::MainFooter;
use pages::*;
use cobul::*;

mod models;
mod constants;
mod components;
mod pages;

#[derive(PartialEq, Clone, Debug)]
pub enum Page {
    Info,
    Overview,
}

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub enum Msg {
    Page(Page),
    External(String),
}

pub struct Model {
    page: Page,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { page: Page::Overview }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Page(page) => {
                self.page = page;
                true
            }
            Msg::External(url) => {
                let _ = window().unwrap().location().set_href(&url);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onpage = ctx.link().callback(Msg::Page);
        let onexternal = ctx.link().callback(Msg::External);

        let page = match self.page {
            Page::Info => html! { <Info onpage={onpage.clone()} /> },
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
                    <MainFooter onpage={onpage} onexternal={onexternal}/>
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
