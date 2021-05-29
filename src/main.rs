use anyhow::Error;
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Json, Nothing};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Item {
    id: u32,
    description: String,
    price: u32,
}

enum Msg {
    AddOne,
    FetchReady(Result<Item, Error>),
    FetchData,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Model>,
    value: i64,
    _ft: Option<FetchTask>,
    data: Option<Item>,
}

impl Model {

    fn fetch_json(&mut self) -> FetchTask {
        let callback = self.link.batch_callback(
            move |response: Response<Json<Result<Item, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                log::info!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    Some(Msg::FetchReady(data))
                } else {
                    log::info!("{:?}", meta);
                    None // FIXME: Handle this error accordingly.
                }
            },
        );

        let request = Request::get("http://localhost:5000/item/1").body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            _ft: None,
            data: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                let task = self.fetch_json();
                self._ft = Some(task);
                log::info!("FetchData");
                true
            }
            Msg::FetchReady(response) => {
                log::info!("FetchReady");
                self.data = response.ok();
                true
            }
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let mut it= String::from("boo");
        if let Some(item) = self.data.as_ref() {
            it = format!("{} {} {}", item.id, item.description, item.price);
        }
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::FetchData)>{ "+1" }</button>
                <p>{ self.value }</p>
                <p> { it } </p>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
