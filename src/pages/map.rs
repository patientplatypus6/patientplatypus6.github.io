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

use super::super::util::util;


#[derive(Clone, PartialEq, Properties)]
pub struct ImageCoords{
  pub month: String,
  pub image: String,
  pub notes: String,
  pub position: (i32, i32)
}

#[derive(Clone, PartialEq, Properties)]
pub struct Map{
  imagecoordsvec: Vec<ImageCoords>, 
  activeindex: String
}

pub enum Msg {
  Hovered(i32), 
  HoveredOff
}

impl Component for Map {
  type Message = Msg;
  type Properties = ();
  
  fn create(ctx: &Context<Self>) -> Self {
    Self { 
      imagecoordsvec: util::imagecoordshandler(), 
      activeindex: "".to_string()
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    if first_render{
    }
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::Hovered(indexval) => {
        log::info!("insided hovered");
        self.activeindex = indexval.to_string();
      }, 
      Msg::HoveredOff => {
        log::info!("insided hovered off");
        self.activeindex = "".to_string()
      }
    }
    true
  }

  fn destroy(&mut self, ctx: &Context<Self>) {

  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <>
        <style>
        {"
          body, html{
            padding: 0; 
            margin: 0;
          }
          .main{
            width: 100vw;
            height: 100vh;
            position: relative;
            overflow: auto;
          }
          .map{
            position: relative;
            height: 1473px;
            width: 1907px;
            float: right;
          }
          .mapimg{
            postion: absolute;
            z-index: 1;
            top: 0; 
            left: 0;
            right: 0;
            bottom: 0;
          }
          .mapcontainer{
            background: lightblue; 
            position: relative;
            display: inline-block;
            height: 1473px;
            width: calc(1907px + 20vw + 22px);
            margin-right: 200px;
          }
          .map-point {
            display: inline-block;
            z-index: 2;
            position: absolute;
            width: 10px;
            height: 10px;
            margin-top: -5px;
            margin-left: -5px;
            background-color: red;
            border-radius: 50%;
            box-shadow: 0px 0px 5px rgba(0, 0,200,1);
            cursor: pointer; 
            pointer-events: auto;
          }
          .map-point:hover{
            background-color: blue;
            box-shadow: 0px 0px 5px rgba(200,0,0,1);
            transition: 1s linear;
          }
          .map-point:active{
            background-color: orange;
            box-shadow: 0px 0px 5px rgba(0, 200,0,1);
            transition: 0.1s linear;
          }
          .map-sign{
            position: relative;
            margin-left: -200px;
            margin-top: -225px;
            opacity: 1;
            width: 400px;
            height: 200px;
            background: black;
            color: white;
            font-weight: bold;
            font-size: 1rem;
            text-align: center;
            border-radius: 5px;
            pointer-events:none;
          }
          .triangle {
            width: 0;
            height: 0;
            position: relative;
            margin-left: -45px;
            margin-top: 0px;
            opacity: 0.7;
            border-left: 50px solid transparent;
            border-right: 50px solid transparent;
            border-bottom: 25px solid #000;
            transform: rotate(180deg);
            pointer-events: none;
          }
          .menu{
            position: fixed;
            z-index: 10;
            top: 5px; 
            left: 5px; 
            width: 20vw; 
            height: 80vh;
            background: grey;
            padding: 5px;
            font-size: 1.75rem;
            overflow: scroll;
            text-align: left;
            font-weight: bold;
            border: 6px solid darkgreen;
          }
        "}
        </style>
        <div style="background: lightblue;">
          <div id="map" class="main">
            <div class="menu">
              {"This is a map of San Francisco where I've added some red dots with links to my pictures. You can find the pictures throughout my blog. Scroll around the map and look at the pics via location. I'll be adding to the map as I write more and take more pictures wandering around the city. Stay tuned!"}
            </div>
            <div class="mapcontainer">
              <div class="map">
                <img loading="lazy" src={"static/mapcompressed.jpg"} class="mapimg"/>
                {
                  for self.imagecoordsvec.iter().enumerate().map(|(index, note)| {
                    let indexval = index.clone();
                    html!{
                      <div 
                        key={index} 
                        class="map-point" style={format!("top:{}px; left:{}px;", note.position.0, note.position.1)}
                        onmouseenter={ctx.link().callback(move |_| Msg::Hovered(index.clone() as i32))}
                        onmouseleave ={ctx.link().callback(move |_| Msg::HoveredOff)}
                      >
                        {
                          if self.activeindex == indexval.clone().to_string() {
                            html!{
                              <>
                                <div class="map-sign">
                                  <div style="display: inline-block; position: absolute; top: 0; left: 0; height: 100%; width: 100%; z-index: 25;">
                                    <div style="display: inline-block; width: 100%; height: 100%;">
                                      <div style="display: inline-block; width: calc(50% - 10px); height: calc(100% - 10px); float: right; padding: 5px; background: black; opacity: 1; border-radius: 5px;">
                                        {note.clone().month} {" - "} {note.clone().notes}
                                      </div>
                                      <div style="display: inline-block; width: 50%; height: 100%; float: right;">
                                        <img loading="lazy" src={note.clone().image} style="display: inline-block; width: 100%; height: 100%; border-radius: 5px"/>
                                      </div>
                                    </div>
                                  </div>
                                </div>
                                <div class="triangle"/>
                              </>
                            }
                          }else{
                            html!{
                              <div/>
                            }
                          }
                        }
                      </div>
                    }
                  })
                }
              </div>
            </div>
          </div>
        </div>
      </>
    } 
  }


}