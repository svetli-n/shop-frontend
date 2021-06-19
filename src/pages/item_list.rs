use crate::pages::item::Item;
use yew::{html, Component, ComponentLink, Html, Properties, InputData, Bridge};
use yew::{events::KeyboardEvent, Classes};
use std::collections::{HashMap, BTreeMap};
use yew::format::{Json, Nothing};
use anyhow::Error;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use crate::model;

pub enum Msg {
    Search(String),
    FetchReady(Result<Vec<BTreeMap<String, String>>, Error>),
    AddToBasket(BTreeMap<String, String>),
}


pub struct ItemList {
    list: Vec<BTreeMap<String, String>>,
    link: ComponentLink<Self>,
    _ft: Option<FetchTask>,
}

impl ItemList {

    fn fetch_json(&mut self, query: String) -> FetchTask {
        let callback = self.link.batch_callback(
            move |response: Response<Json<Result<Vec<BTreeMap<String, String>>, Error>>>| {
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

    fn view_item(&self, item: BTreeMap<String, String>) -> Html {
       html! {
           <div class="tile is-ancestor">
               <div class="tile is-7 box is-warning has-background-grey-light">
                    <div class="tile">
                    {"Specification"}
                    </div>
                    <div class="tile">
                    <ul>
                       {
                           for item.iter().map(|entry| {
                             html! {
                                    <li>
                                        {format!("{}: {}", entry.0, entry.1) }
                                    </li>
                                }
                           })
                        }
                    </ul>
                    </div>
                </div>
               <button class="button is-info is-light" onclick=self.link.callback(move |_| Msg::AddToBasket(item.clone()))>
                { format!("Add to basket") }
               </button>
           </div>
       }
    }

}

impl Component for ItemList {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        log::info!("ItemList");
        ItemList {
            list: Vec::new(),
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
                self.list = response.ok().unwrap();
                self.list.sort_by(|a, b| {
                    let x = a.get("price").unwrap().split(".").collect::<Vec<_>>()[0].parse::<i32>().unwrap();
                    log::info!("x: {}", x);
                    let y =  b.get("price").unwrap().split(".").collect::<Vec<_>>()[0].parse::<i32>().unwrap();
                    log::info!("y: {}", y);
                    x.cmp(&y)
                });
                let mut sorted: Vec<BTreeMap<String, String>> = Vec::new();
                for entry in &self.list {
                    let mut v: Vec<_> = entry.iter().collect();
                    v.sort_by(|a, b| a.0.cmp(b.0));
                    let e: BTreeMap<String, String> = v.iter().map(|a| {
                        (a.0.clone(), a.1.clone())
                    }).collect();
                    sorted.push(e);
                }
                self.list = sorted;
            },
            Msg::AddToBasket(item) => {
                log::info!("In basket: {:?}", item);
            }
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
                />
                <div class="tile is-ancestor is-vertical mt-6 ml-3">
                    {
                        for self.list.iter().map(|d| {
                            self.view_item(d.clone())
                        })
                    }
                </div>
            </div>
        }
    }
}
