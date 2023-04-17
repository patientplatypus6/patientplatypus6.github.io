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
pub struct Navlinks{
  is_mobile: bool, 
  open_modal: bool
}

pub enum Msg {
  ModalToggle
}

impl Component for Navlinks {
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
          .navmenu{
            display: inline-block;
            background-image: url('static/tiger.jpg') !important;
            float: left;
            width: 15vw;
            background: grey;
            border: 4px solid lightgreen;
            margin-left: 5px;
            margin-top: 5px;
          }
        "}
        </style>
        <div style="position: absolute; display: inline-block; top: 0; left: 0; width: calc(20vw - 10px);">
          <div class="navmenu">
            <br/>
            <Link<Route> to={Route::Home}>
              <div class="blueprint-button">
                {"Home"}
              </div>
            </Link<Route>>
            <br/>
            <br/>
            <Link<Route> to={Route::Links}>
              <div class="blueprint-button">
                {"Links"}
              </div>
            </Link<Route>>
            <br/>
            <br/>
            <Link<Route> to={Route::About}>
              <div class="blueprint-button">
                {"About"}
              </div>
            </Link<Route>>
            <br/>
            <br/>
            <br/>
          </div>
        </div>
      </>
    }
  }
}

impl Navlinks{

}