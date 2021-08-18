use yew::prelude::*;
use pbs::*;
use crate::Route;
use yew::utils::window;

pub struct MainFooter {
    link: ComponentLink<Self>,
}

pub enum Msg {
    GitHub,
    Docs,
    Info,
}

impl Component for MainFooter {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GitHub => {
                window().location().set_href("https://github.com/");
            },
            Msg::Docs => {
                yew_router::push_route(Route::Docs);
            }
            Msg::Info => {
                yew_router::push_route(Route::Info);
            }
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Footer extra="column is-narrow pb-3">
                <Columns>
                    <Column extra="pr-6">
                        <Title> {"About"}</Title>
                        <Block>
                            <p style="hyphens:auto"> <strong>{"Leche"} </strong> {" is een project dat streeft naar standardisatie \
                            van het opwarmproces van babymelk in de microgolf."} </p>
                        </Block>
                        <Buttons>
                            <cbs::IconButton icon="fab fa-github" text="GitHub" onclick={self.link.callback(|_| Msg::GitHub)} />
                            <cbs::IconButton icon="fas fa-align-left" text="docs" onclick={self.link.callback(|_| Msg::Docs)}/>
                        </Buttons>
                    </Column>
                    <Column extra="pr-6">
                        <Title> {"Creator"}</Title>
                        <Block>
                            <p style="hyphens:auto"> {"Thomas Dooms is een masterstudent informatica aan UAntwerpen. \
                            Dit project is gemaakt in kader van het honoursprogramma."} </p>
                        </Block>
                        <Block>
                            <Icon icon="fas fa-envelope" text={"thomas@dooms.eu"} />
                        </Block>
                    </Column>
                    <Column extra="pr-6">
                        <Title> {"Info"}</Title>
                        <Block>
                             <p style="hyphens:auto"> {"Meer info over dit project en uitleg bij de bij het \
                             opwarmproces en functionaliteit van deze hulpsite kan u hier vinden."} </p>
                        </Block>
                        <Block>
                            <cbs::IconButton icon="fas fa-question" text="info" onclick={self.link.callback(|_| Msg::Info)}/>
                        </Block>
                    </Column>
                </Columns>
            </Footer>
        }
    }
}