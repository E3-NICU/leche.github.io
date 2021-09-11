use yew::prelude::*;

use cobul::*;

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
            <Button onclick={onclick}> <Icon icon="fas fa-arrow-left"/> <span> {"Terug"} </span> </Button>
        </Block>
        <Content>
            <h3> {"Deze pagina is nog niet afgewerkt."} </h3>
        </Content>
        </>
    }
}