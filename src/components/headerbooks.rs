use yew::prelude::*;
use regex::Regex;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsValue;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, EventTarget, HtmlInputElement, HtmlImageElement, IntersectionObserver, IntersectionObserverEntry};
use web_sys::HtmlCanvasElement;
use web_sys::CanvasRenderingContext2d;
use yew::{Callback};
use reqwest::header::USER_AGENT;
use std::collections::HashMap;
use yew_style_in_rs::*;
use serde_json::{Value};
use reqwest;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use stdweb::js;
use chrono::prelude::*;
use yew_canvas::Canvas;

use crate::routes::routes::Route;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Header{
  is_mobile: bool, 
  open_modal: bool
}

pub enum Msg {
  ModalToggle
}

impl Component for Header {
  type Message = Msg;
  type Properties = ();


  fn create(ctx: &Context<Self>) -> Self {
    let is_mobile = js! {
        return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
    };
    Self { 
      is_mobile: is_mobile == true,
      open_modal: false
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {

  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg{
      Msg::ModalToggle => {
        if self.open_modal{
          self.open_modal = false;
        }else{
          self.open_modal = true;
        }
        log::info!("after toggling open modal and value: {:?}", self.open_modal);
      }
    }
    true
  }

  fn destroy(&mut self, ctx: &Context<Self>) {

  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let width = if self.is_mobile { "80" } else { "60" };
    html! {
      <>
        <style>
        {"
          .header1{
            text-decoration: none !important;
            font-size: 3em;
            text-align: center;
            font-weight: bold;
            padding-top: 0;
            padding-bottom: 0;
            margin-top: 0;
            margin-bottom: 0;
          }
        "}
        </style>
        <div>
          <h1 class="header1">
            <img loading="lazy" src={"../static/title.png"} class="imagetag"/>
            <p style="color: white; background: black; font-size: 0.8rem; padding: 5px; margin-bottom: 0; margin-top: -20px; font-weight: 900; text-decoration: underline; font-family: 'Creepster', cursive;">
              {"Me? I'm the idiot child of technology's conscience. Who the fuck are you?"}
            </p>
            <div style="color: white; background: black; font-size: 0.8rem; padding: 5px; margin-top: 0px; font-weight: 900; text-decoration: none; font-size: 0.8rem; font-family: 'Times New Roman', cursive; background-color: darkorange; color: purple;">
              {"The number of people who don't give a shit since forever - "}
              <script type={"text/javascript"} src={"//cdn.livetrafficfeed.com/static/hitcounterjs/live.js?sty=2&min=7&sta=1&uni=1&tz=Asia%2FTokyo&ro=0"}></script><noscript id={"LTF_hitcounter"}><a href={"https://livetrafficfeed.com/hit-counter"}>{"Website Hit Counter"}</a></noscript>
            </div>
          </h1>   
        </div>
      </>
    }
  }
}

impl Header{

}