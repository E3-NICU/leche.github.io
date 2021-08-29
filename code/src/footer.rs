use yew::prelude::*;

use pbs::*;

use crate::Page;

pub enum Msg {
    GitHub,
    Docs,
    Info,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onpage: Callback<Page>,
    pub onexternal: Callback<String>,
}

#[function_component(MainFooter)]
pub fn footer(props: &Props) -> Html {
    let ongithub = props.onexternal.reform(|_| "https://github.com/E3-NICU/leche.github.io".to_owned());
    let ondocs = props.onpage.reform(|_| Page::Docs);
    let oninfo = props.onpage.reform(|_| Page::Info);

    html! {
        <Footer extra="column is-narrow pb-3">
            <Columns>
                <Column extra="pr-6">
                    <Title> {"About"}</Title>
                    <Block>
                        <p style="hyphens:auto"> {"Dit is een pilootproject dat streeft naar standardisatie \
                        van het opwarmproces van babymelk in de microgolf."} </p>
                    </Block>
                    <Buttons>
                        <cbs::IconButton icon="fab fa-github" text="GitHub" onclick={ongithub} />
                        <cbs::IconButton icon="fas fa-align-left" text="docs" onclick={ondocs}/>
                    </Buttons>
                </Column>
                <Column extra="pr-6">
                    <Title> {"Creator"}</Title>
                    <Block>
                        <p style="hyphens:auto"> {"Thomas Dooms is een masterstudent informatica aan UAntwerpen. \
                        Als u vragen hebt kan u deze altijd sturen naar op mijn e-mail."} </p>
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
                        <cbs::IconButton icon="fas fa-question" text="info" onclick={oninfo}/>
                    </Block>
                </Column>
            </Columns>
        </Footer>
    }
}