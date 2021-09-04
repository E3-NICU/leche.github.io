use yew::prelude::*;

use pbs::prelude::*;

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
            <Button onclick={onclick}> <Icon icon="fas fa-arrow-left"/> <span> {"Back"} </span> </Button>
        </Block>
        <Content>
            <h3> {"This page is still under development"} </h3>
        </Content>
        </>
    }
}