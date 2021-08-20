use yew::prelude::*;

use pbs::*;

use crate::Page;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onpage: Callback<Page>,
}

pub struct Info {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for Info {
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
        let onclick = self.props.onpage.reform(|_| Page::Overview);
        html! {
            <>
            <Block>
                <cbs::IconButton text="Back" icon="fas fa-arrow-left" onclick={onclick} />
            </Block>
            <Content>
                <h3> {"This page is still under development"} </h3>
                <h3> {"Over het project"} </h3>
                <p> <strong> {"Leche"} </strong> {" is een project dat streeft naar standardisatie van het \
                opwarmproces van babymelk in de microgolf. Dit wordt bereikt door een simpele hulpapplicatie\
                 aan te bieden die berekent hoe lang melk in de microgolf moet."} </p>

                <h3> {"Accuraatheid"} </h3>
                <p> {"De bereking is opgesteld uit honderden experimenten "} </p>

                <h3> {"Interface"} </h3>
                <p> {"De berekening is opgesteld uit honderden experimenten "} </p>
            </Content>
            </>
        }
    }
}