use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::item::Item;
mod pages;
mod model;


#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/items/{id}"]
    Item(u32),
}


struct Model {
}

impl Model {

}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                match switch {
                    AppRoute::Item(id) => html!{<Item id = id/>},
                }
                })
            />
        }
    }
}



fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
