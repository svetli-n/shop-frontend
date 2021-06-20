use crate::pages::item::Item;
use yew::{html, Component, ComponentLink, Html, Properties, InputData, Bridge};
use yew::{events::KeyboardEvent, Classes};
use std::collections::{HashMap, BTreeMap};
use yew::format::{Json, Nothing};
use anyhow::Error;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use crate::model;
use crate::pages::basket::{Basket, LineItem};

pub enum Msg {
    Search(String),
    FetchReady(Result<Vec<BTreeMap<String, String>>, Error>),
    AddToBasket(BTreeMap<String, String>),
    RemoveFromBasket(String),
}


pub struct ItemList {
    list: Vec<BTreeMap<String, String>>,
    link: ComponentLink<Self>,
    _ft: Option<FetchTask>,
    basket: Basket,
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
           <div class="tile ml-3">
               <div class="tile is-6 box is-warning has-background-grey-light">
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
               <button class="button ml-3 is-info is-light" onclick=self.link.callback(move |_| Msg::AddToBasket(item.clone()))>
                { format!("Add to basket") }
               </button>
           </div>
       }
    }

    fn view_basket(&self, items: BTreeMap<String, LineItem>) -> Html {
        html! {
            <div class="tile is-vertical">
                <div>
                {
                        format!("Basket")
                }
                <ul>
                {
                    for items.into_iter().map(|item| {
                        html! {
                        <div class="tile">
                            <li>
                                { format!("id: {}, quantity: {}, price: {}", item.1.id, item.1.qty, item.1.price) }
                            </li>
                           <button class="button is-warning is-light" onclick=self.link.callback(move |_| {
                            Msg::RemoveFromBasket(item.1.id.clone())
                           })>
                            { format!("Remove") }
                           </button>
                        </div>
                        }
                    })
                }
                </ul>
                </div>
                <div>
                {
                    format!("Total: {}", self.basket.total())
                }
                </div>
               <button class="button is-primary is-light" >
                    { format!("Buy") }
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
            basket: Basket::new(),
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
                log::info!("Add to basket: {:?}", item);
                self.basket.add(item);
            },
            Msg::RemoveFromBasket(id) => {
                log::info!("Remove from basket: {}", id);
                self.basket.remove(id);
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        todo!()
    }


    fn view(&self) -> Html {

        html! {
            <div class="tile is-ancestor">
                <div class="tile is-vertical mt-3 ml-3">
                    <div class="tile is-parent">
                        <div class="tile is-child is-3">
                            <input
                                class="input"
                                type="text"
                                placeholder="Search chemicals"
                                oninput=self.link.callback(|e: InputData| Msg::Search(e.value))
                            />
                        </div>
                        <div class="tile is-6">
                        </div>
                        <div class="tile id-child ml-6">
                            { self.view_basket(self.basket.items.clone()) }
                        </div>
                    </div>
                    <div class="tile is-vertical">
                        {
                            for self.list.iter().map(|d| {
                                self.view_item(d.clone())
                            })
                        }
                    </div>
                </div>
            </div>
        }
    }
}
