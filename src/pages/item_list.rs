use crate::pages::item::Item;
use yew::{html, Component, ComponentLink, Html, Properties, InputData};
use yew::{events::KeyboardEvent, Classes};
use std::collections::HashMap;
use yew::format::{Json, Nothing};
use anyhow::Error;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use crate::model;

pub enum Msg {
    Search(String),
    FetchReady(Result<Vec<HashMap<String, String>>, Error>),
}


pub struct ItemList {
    list: Option<Vec<HashMap<String, String>>>,
    link: ComponentLink<Self>,
    _ft: Option<FetchTask>,
}

impl ItemList {

    fn fetch_json(&mut self, query: String) -> FetchTask {
        let callback = self.link.batch_callback(
            move |response: Response<Json<Result<Vec<HashMap<String, String>>, Error>>>| {
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

        let url = format!("http://localhost:5000/search/{}", query);
        let request = Request::get(url).body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }

}

impl Component for ItemList {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        log::info!("ItemList");
        ItemList {
            list: None,
            link,
            _ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Search(value) => {
                log::info!("msg: {}", value);
                let task = self.fetch_json(value);
                self._ft = Some(task);
            },
            Msg::FetchReady(response) => {
                log::info!("FetchReady");
                self.list = response.ok();
                log::info!("Got: {:?}", self.list);
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        todo!()
    }

    fn view(&self) -> Html {

        html! {
            <div class="tile is-vertical mt-6 ml-3">
                <input
                    class="tile is-3 box"
                    placeholder="Search chemicals"
                    // value=&self.state.value
                    oninput=self.link.callback(|e: InputData| Msg::Search(e.value))
                    // onkeypress=self.link.batch_callback(|e: KeyboardEvent| {
                    //     if e.key() == "Enter" { Some(Msg::Search(e.value)) } else { None }
                    // })
                />
                <div class="tile is-ancestor is-vertical mt-6 ml-3">
                    {
                        for self.list.iter().map(|d| {
                            format!("{:?}", d)
                        })
                    }
                </div>
            </div>
        }

    }
}
