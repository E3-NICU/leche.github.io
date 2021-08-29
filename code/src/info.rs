use yew::prelude::*;

use pbs::*;

use crate::Page;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onpage: Callback<Page>,
}

#[function_component(Info)]
pub fn info(props: &Props) -> Html {
    let onclick = props.onpage.reform(|_| Page::Overview);
    html! {
        <>
        <Block>
            <cbs::IconButton text="Back" icon="fas fa-arrow-left" onclick={onclick} />
        </Block>
        <Content>
            <h3> {"This page is still under development"} </h3>
            <h3> {"Over het project"} </h3>
            <p> {" Dit is een pilootproject dat streeft naar standardisatie van het \
            opwarmproces van babymelk in de microgolf. Dit wordt bereikt door een simpele hulpapplicatie \
             aan te bieden die berekent hoe lang melk in de microgolf moet."} </p>

            <h3> {"Gebruik"} </h3>
            <p> {"Om een schatting te krijgen moeten er eerst enkele parameters ingegeven worden.\
            Het volume van de melk in de spuit is eenvoudig verstelbaar. Vervolgens moet de starttemperatuur \
            ingegeven worden. Hier kan men kiezen tussen de temperatuur van de normale koelkast, \
            de koelkast met synthetische melk of kamertemperatuur. "} </p>

            <p> {"Indien bijvoorbeeld melk uit de koelkast warmer lijkt dan gewoonlijk kan men voor de beste \
            resultaten best even meten met een infrarood sensor. Voor deze meting is het belangrijk dat het \
            oppervlak van de meting niet aangeraakt wordt aangezien dit het resultaat sterk beïnvloed. "} </p>

            <p> {"De locatie van de spuit in de microgolf is enorm belangrijk, dit programma gaat ervan uit \
            dat de het centrum van de melk (niet de spuit) in het midden van de microgolf ligt. \
            Indien dit niet gaat met de 60ml spuit, zorg dan zeker dat de volledige spuit op de draaiplaat past \
            aangezien deze andere blijft hangen tegen de rand en andere resultaten geeft."} </p>

            <h3> {"Accuraatheid"} </h3>
            <p> {"Een schatting van de temperatuur wordt gegeven bij elke verandering."} </p>
            <p> {"De berekening van het opwarmen zelf is accuraat tot ±2 graden, \
            de grootste variatie komt door het verschil van temperatuur uit de koelkast."} </p>

            <p> {"Kleine volumes zijn minder accuraat omdat hier meerdere factoren een rol spelen \
             die moeilijk te meten zijn. Daarom zal de aangegeven spreiding groter zijn."} </p>

            <h3> {"Toekomst"} </h3>
            <p> {"Er zijn nog verschillende ideeën en optimalisaties die onderzocht kunnen worden."}</p>

            <p> {"Het is bijvoorbeeld niet zeker wat de beste ligging is van de spuiten in de microgolf. \
            Er zijn onderling grote verschillen tussen de liggingen maar welke locatie zorgt voor het minste \
            variantie is in dit project niet onderzocht. Zo kan het mogelijks zijn dat de spuit meer naar de \
            buitenkant leggen accuratere resultaten oplevert."} </p>

            <p> {"Bij het opwarmen van materialen wordt er meestal van uitgegaan dat de opwarming lineair is, \
            dit is immers niet volledig waar, door de opwarming verandert de structuur van de stof alsook de warmteconstante. \
            Bij stoffen zoals water varieert deze waarde vrij weinig maar door de structuur van melk is dit niet verwaarloosbaar meer. \
            In dit onderzoek werd een constante factor van 7% gehanteerd, hetgeen extreem kort door de bocht is."} </p>

            <p> {"De verpleging steekt ook graag meerdere spuiten samen in de microgolf. \
            De enige conclusie van dit project hierover is dat de manier waarop spuiten samen leggen momenteel \
            gebeurt zorgt voor grote variatie. Met het idee dat spuiten niet centraal moeten liggen, \
            zou het nuttig zijn om verder onderzoek te doen naar mogelijke opstellingen om meerdere spuiten \
            wel mogelijk te maken. Nagaan op welke manier de tijd in de microgolf schaalt met het aantal spuiten \
            is hierbij essentieel."} </p>

            </Content>
        </>
    }
}