use crate::model;
use anyhow::Error;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: u32,
}

pub enum Msg {
    FetchData,
    FetchReady(Result<model::Item, Error>),
}

pub struct Item {
    item: Option<model::Item>,
    _ft: Option<FetchTask>,
    link: ComponentLink<Item>,
}

impl Item {
    fn fetch_json(&mut self, id: u32) -> FetchTask {
        let callback = self.link.batch_callback(
            move |response: Response<Json<Result<model::Item, Error>>>| {
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

        let url = format!("http://localhost:5000/item/{}", id);
        let request = Request::get(url).body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}

impl Component for Item {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut item = Item {
            link,
            item: Some(model::Item {
                id: props.id,
                description: String::from("desc"),
                price: 1,
            }),
            _ft: None,
        };
        item.update(Msg::FetchData);
        item
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                log::info!("FetchData");
                let task = self.fetch_json(self.item.as_ref().unwrap().id);
                self._ft = Some(task);
                true
            }
            Msg::FetchReady(response) => {
                log::info!("FetchReady");
                self.item = response.ok();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        todo!()
    }

    fn view(&self) -> Html {
        let item = self.item.as_ref().unwrap();
        html! {
            <p> {  format!("id: {} description: {} price: {}", item.id, item.description, item.price) } </p>
        }
    }
}
