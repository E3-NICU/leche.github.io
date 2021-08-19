use yew::prelude::*;
use pbs::*;
use crate::Page;

pub struct Docs {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onpage: Callback<Page>
}

impl Component for Docs {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick=self.props.onpage.reform(|_| Page::Overview);
        html! {
            <>
            <Block>
                <cbs::IconButton text="Back" icon="fas fa-arrow-left" onclick={onclick} />
            </Block>
            <Content>
                <h3> {"This page is still under development"} </h3>
            </Content>
            </>
        }
    }
}