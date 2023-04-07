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

use super::super::super::components::flashlinksbooks::Flashlinks;


#[derive(Clone, PartialEq, Properties)]
pub struct Politics{
}

pub enum Msg {
}

impl Component for Politics {
  type Message = Msg;
  type Properties = ();


  fn create(ctx: &Context<Self>) -> Self {
    Self{}
  }

  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {

  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                background: url('../static/background2.jpg') !important;
                background-size: cover;
                height: 100vh;
                min-width: 100vw;
                text-align: center;
                padding-top: 5px;
                font-family: Helvetica, Arial, sans-serif;
                position: relative;
                overflow-y: auto;
                overflow-x: hidden;
              }
              .pic-background{
                margin-left: 5px;
                margin-top: 5px;
                margin-bottom: 5px;
                background-image: url('../static/portsky.jpg') !important;
                width: calc(20vw - 10px);
                height: auto;
                background-size: 100% 100%;
              }
              .timeheader{
                text-decoration: underline;
                width: calc(100% - 10px);
                font-weight: 900;
                color: rgba(200,200,200,0.9);
                padding: 5px;
                background: rgba(0,0,150,0.8);
              }
              .imagetag{
                width: 90%;
                padding-left: 5%;
              }

              @import url('https://fonts.googleapis.com/css2?family=Creepster&display=swap');

              @keyframes rainbow {
                0% { color: red; }
                14% { color: orange; }
                28% { color: yellow; }
                42% { color: green; }
                57% { color: blue; }
                71% { color: indigo; }
                85% { color: violet; }
                100% { color: red; }
              }

              .header1{
                background-image: linear-gradient(to top right, rgba(0,0,0,0.5) 0%,rgba(0,255,0,0.5) 100%), url('../static/banner.png') !important;
                font-family: 'Creepster', cursive;
                font-size: 3em;
                text-align: center;
                text-decoration: underline;
                font-weight: bold;
                animation: rainbow 5s infinite;
                padding-top: 0;
                padding-bottom: 0;
                margin-top: 0;
                margin-bottom: 0;
              }

              .contentcontainer{
                display: inline-block; 
                text-align: left; 
                background: rgba(123,163,153, 0.8); 
                min-height: 100vh;
                padding: 5px; 
                width: 60vw; 
                float: right;
                font-weight: bold; 
                font-size: 1.2rem; 
                line-spacing: 1.2rem;
              }

              .learnmorecontainer{
                display: inline-block;
                position: relative;
                color: black;
                font-weight: bold;
                float: right;
                text-align: center;
                width: 20vw;
              }

              .learnmore{
                display: inline-block;
                position: relative;
                background: #d67669;
                margin-top: 10vh;
                color: black;
                font-weight: bold;
                float: right;
                width: 50%;
                padding: 5px;
                margin-right: 5px;
                border: 10px groove rgba(100,200,150,0.8);
              }

              .skull{
                display: inline-block;

                background-size: 100% 100%;
                position: absolute;
                top: -11.5vh;
                left: -11.5vw;
                width: 12vw;
                height: 12vw;
                z-index: 20;
              }

              .octopus{
                display: inline-block;
                background-image: url('../static/octopusclean.png') !important;
                background-size: cover;
                position: absolute;
                top: 0;
                left: -5px;
                pointer-events:none;
                z-index: 5;
                height: 50vh; 
                width: calc(20vw - 5px);
              }

              .picturemapcontainer{
                width: calc(20vw - 10px); 
                margin: 5px;
              }
  
              .picturemap{
                width: 100%; 
                margin-top: 5px;
                height: auto; 
                filter: invert(0%);
                transition: 0.5s;
                cursor: pointer;
              }
  
              .picturemap:hover{
                filter: invert(100%);
                transition: 0.5s;
              }

              @media (max-width: 600px) {
                .contentcontainer {
                  width: calc(80vw - 10px);
                  float: right;
                }
                .learnmorecontainer{
                  height: 0;
                  width: 0;
                  border: 0;
                }
                .learnmore{
                  height: 0;
                  width: 0;
                  border: 0;
                  padding: 0;
                }
                .octopus{
                  width: calc(20vw + 5px);
                }
                .picturemapcontainer{
                  width: 0; 
                  height: 0;
                }
                .skull{
                  width: 0;
                  height: 0;
                }
              }

              .navmenu{
                display: inline-block;
                background-image: url('../static/tiger.jpg') !important;
                float: left;
                width: 15vw;
                background: grey;
                border: 4px solid lightgreen;
                margin-left: 5px;
                margin-top: 5px;
              }

              .blueprint-button {
                display: inline-block;
                width: calc(90% - 16px); 
                padding: 4px 8px;
                font-size: 14px;
                font-weight: 600;
                line-height: 1.5;
                text-align: center;
                white-space: nowrap;
                vertical-align: middle;
                cursor: pointer;
                user-select: none;
                border-radius: 3px;
                border: none;
                color: #fff;
                background-color: #106ba3;
              }
          
              .blueprint-button:hover {
                background-color: #FFBA08;
                transition: 0.5s;
              }
          
              .blueprint-button:active {
                background-color: #D00000;
                transition: 0.1s;
              }

          "}
        </style>
        <div class="main">
          <Flashlinks/>
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
          
          <div class="octopus"/>

          <div class="contentcontainer">
            <h1 class="header1">
              <img loading="lazy" src={"../static/title.png"} class="imagetag"/>
            </h1>      
            <div>
              <div class="timeheader">
                {"Places to go, things to be"}
              </div>    
              <div>
                <p>
                  {"Here's a shit unordered list of the links I've used so far. I'll unfuck this later. Enjoy!"}
                </p>
              </div>
              <ul>  
                <li>
                  <a href={"https://www.youtube.com/watch?v=e2CliA8PuRM&ab_channel=BronxRadioLab"}>{"religion"}</a>
                </li>
                <li>
                  <a href={"https://www.lingscars.com/"}>{"as this asshole"}</a>
                </li>
                <li>
                  <a href={"https://news.ycombinator.com"}>{"news aggregation website"}</a>
                </li>
                <li>
                  <a href={"https://ohadravid.github.io/posts/2023-03-rusty-python/"}>{"this guy"}</a>
                </li>
                <li>
                  <a href={"https://www.vanityfair.com/news/2017/06/neal-stephenson-metaverse-snow-crash-silicon-valley-virtual-reality"}>{"In the future we are all gargoyles"}</a>
                </li>
                <li>
                  <a href={"https://www.gamespot.com/articles/saints-row-iv-developers-respond-to-dildo-weapon-criticisms/1100-6412418/"}>{"beat someone to death with a dildo"}</a>
                </li>
                <li>
                  <a href={"https://www.polarsignals.com/blog/posts/2023/03/28/how-to-read-icicle-and-flame-graphs/"}>{"flame charts"}</a>
                </li>
                <li>
                  <a href={"https://www.wired.com/story/brandon-sanderson-is-your-god/"}>{"dipshit with a sword"}</a>
                </li>
                <li>
                  <a href={"https://www.youtube.com/watch?v=5VtDM5jicRQ"}>{"today was a good day"}</a>
                </li>
                <li>
                  <a href={"https://sfgov.org/mod/sites/default/files/FileCenter/Documents/2176-Sidewalk%20Landscape%20with%20SIRP.pdf"}>{"According to Article 15, Section 706 of the Public Works Code"}</a>
                </li>
                <li>
                  <a href={"https://www.youtube.com/watch?v=9nWAic0lHVI&embeds_euri=https%3A%2F%2Fwww.bing.com%2F&embeds_origin=https%3A%2F%2Fwww.bing.com&feature=emb_logo&ab_channel=CNN"}>{"this guy"}</a>
                </li>
                <li>
                  <a href={"https://www.youtube.com/watch?v=3L4YrGaR8E4&ab_channel=RATMVEVO"}>{"Bulls on Parade"}</a>
                </li>
                <li>
                  <a href={"https://wilwheaton.net/2023/03/the-library-is-a-safe-place/"}>{"That guy from Star Trek"}</a>
                </li>
                <li>
                  <a href={"https://www.youtube.com/watch?v=7Pq-S557XQU&ab_channel=CGPGrey"}>{"Humans Need Not Apply"}</a>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </>
    }
  }
}

impl Politics{
}