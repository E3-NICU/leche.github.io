use yew::prelude::*;
use pbs::*;

pub struct Info {
    link: ComponentLink<Self>,
}

impl Component for Info {
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
            <Content>
                <h3> {"Over het project"} </h3>
                <p> <strong> {"Leche"} </strong> {" is een project dat streeft naar standardisatie van het \
                opwarmproces van babymelk in de microgolf. Dit wordt bereikt door een simpele hulpapplicatie\
                 aan te bieden die berekent hoe lang melk in de microgolf moet."} </p>

                <h3> {"Accuraatheid"} </h3>
                <p> {"De bereking is opgesteld uit honderden experimenten "} </p>

                <h3> {"Interface"} </h3>
                <p> {"De bereking is opgesteld uit honderden experimenten "} </p>
            </Content>
        }
    }
}