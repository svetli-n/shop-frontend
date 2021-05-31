use crate::pages::item::Item;
use yew::{html, Component, ComponentLink, Html, Properties};

pub struct ItemList {}

impl Component for ItemList {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ItemList {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        todo!()
    }

    fn view(&self) -> Html {
        let items = (1..3).map(|i| {
            html! {
                <div class="tile is-ancestor">
                    <div class="tile is-2">
                        <p></p>
                    </div>
                    <div class="tile is-3 box is-warning has-background-grey-light">
                        <Item id=i />
                    </div>
                    <button class="button is-dark is-large ml-1">{"Buy"}</button>
                    <div class="tile">
                        <p></p>
                    </div>
                </div>
            }
        });
        html! {
                <div class="tile is-ancestor is-vertical mt-6 ml-3">
                    {for items}
                </div>

        }
    }
}
