use yew::prelude::*;

use pbs::*;

use crate::Page;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onpage: Callback<Page>,
}

#[function_component(Docs)]
pub fn docs(props: &Props) -> Html {
    let onclick = props.onpage.reform(|_| Page::Overview);
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