use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::ColumnSize;

use crate::Page;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onpage: Callback<Page>,
}

#[function_component(Info)]
pub fn info(props: &Props) -> Html {
    let onclick = props.onpage.reform(|_| Page::Overview);
    html! {
        <Columns>
            <Column size={ColumnSize::Is6}>
                <Block>
                    <Button onclick={onclick}> <Icon icon="fas fa-arrow-left"/> <span> {"Back"} </span> </Button>
                </Block>
                <Content>
                    <h3> {"Over het project"} </h3>
                    <p class="has-text-justified"> {"Dit is een pilootproject dat streeft naar standardisatie van het \
                    opwarmproces van babymelk in de microgolf. Dit wordt bereikt door een simpele hulpapplicatie \
                    aan te bieden die berekent hoe lang melk in de microgolf moet."} </p>

                    <h3> {"Gebruik"} </h3>
                    <p class="has-text-justified"> {"Om een schatting te krijgen, moeten er eerst enkele parameters ingegeven worden. \
                    Het volume van de melk in de spuit is eenvoudig verstelbaar aan de hand van een slider. Voor de starttemperatuur kan men aangeven of de melk uit de normale koelkast komt, \
                    uit de koelkast met synthetische melk of op kamertemperatuur is. "} </p>

                    <p class="has-text-justified"> {"Indien de melk uit de koelkast warmer lijkt dan gewoonlijk, kan men \
                    best even de reëele temperatuur opmeten met een infrarood sensor en deze waarde ingeven onder 'gemeten'. Voor deze meting is het cruciaal dat het \
                    oppervlak van de spuit niet aangeraakt wordt, aangezien dit het resultaat sterk beïnvloed. "} </p>

                    <p class="has-text-justified"> {"De locatie van de spuit in de microgolf is enorm belangrijk. Dit programma gaat ervan uit \
                    dat het centrum van de melk (niet de spuit) in het midden van de microgolf ligt. \
                    Indien dit niet gaat, zorg dan zeker dat de volledige spuit op de draaiplaat past \
                    aangezien deze anders blijft hangen tegen de rand en de resultaten beïnvloedt."} </p>

                    <h3> {"Accuraatheid"} </h3>
                    <p class="has-text-justified"> {"Een schatting van de temperatuur wordt gegeven bij elke verandering van de parameters."} </p>
                    <p class="has-text-justified"> {"De berekening van het opwarmen zelf is accuraat tot ±2 graden, \
                    de grootste variatie komt door het verschil van temperatuur uit de koelkast."} </p>

                    <p class="has-text-justified"> {"Kleine volumes zijn minder accuraat omdat hier meerdere factoren een rol spelen \
                     die moeilijk te meten zijn. Daarom zal de aangegeven spreiding groter zijn."} </p>

                    <h3> {"Toekomst"} </h3>
                    <p class="has-text-justified"> {"Er zijn nog verschillende ideeën en optimalisaties die onderzocht kunnen worden."}</p>

                    <p class="has-text-justified"> {"Het is bijvoorbeeld niet zeker wat de beste ligging is van de spuiten in de microgolf. \
                    Er zijn onderling grote verschillen tussen de liggingen maar welke locatie zorgt voor het minste \
                    variantie is in dit project niet onderzocht. Zo kan het mogelijks zijn dat de spuit meer naar de \
                    buitenkant leggen accuratere resultaten oplevert."} </p>

                    <p class="has-text-justified"> {"Bij het opwarmen van materialen wordt er meestal van uitgegaan dat de opwarming lineair is, \
                    dit is immers niet volledig waar, door de opwarming verandert de structuur van de stof alsook de warmteconstante. \
                    Bij stoffen zoals water varieert deze waarde vrij weinig maar door de structuur van melk is dit niet verwaarloosbaar meer. \
                    In dit onderzoek werd een constante factor van 7% gehanteerd, hetgeen extreem kort door de bocht is."} </p>

                    <p class="has-text-justified"> {"Om tijd te besparen is het vaak nuttig om meerdere spuiten tesamen in de microgolf te leggen. \
                    De enige conclusie van dit project hierover is dat de manier waarop spuiten samen leggen momenteel \
                    gebeurt zorgt voor grote variatie. Met het idee dat spuiten niet centraal moeten liggen, \
                    zou het nuttig zijn om verder onderzoek te doen naar mogelijke opstellingen om meerdere spuiten \
                    wel mogelijk te maken. Nagaan op welke manier de tijd in de microgolf schaalt met het aantal spuiten \
                    is hierbij essentieel."} </p>

                </Content>
            </Column>
        </Columns>
    }
}