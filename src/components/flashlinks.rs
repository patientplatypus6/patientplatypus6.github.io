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

// use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::history::History;

use crate::routes::routes::Route;
use yew_router::prelude::*;

// fn test_fn(){
//   log::info!("inside the the test_fn");
// }

async fn get_blog_html() -> String {
  log::info!("inside the get_blog_html folder");
  // let resp = reqwest::get("patientplatypus6.github.io")
  let resp = reqwest::get("http://localhost:8080/")
  .await
  .map_err(|err| {
      log::info!("there was an error: {:?}", err);
  }).expect("there was an error");
  let html = resp
    .text()
    .await
    .map_err(|err| {
      log::info!("there was an error: {:?}", err);
    }).expect("there was an error");
  log::info!("value of html: {:?}", html.to_string());
  html
}

#[derive(Clone, PartialEq, Properties)]
pub struct Flashlinks{
  is_mobile: bool, 
  open_modal: bool, 
  search_term: String,
  // forceupdate: Callback<()>
  // search_term_handler: Callback<((String))>,
}


#[derive(Clone, PartialEq, Properties)]
pub struct Flashlinksprops {
  // pub forceupdate: Callback<()>
  // pub search_term_handler: Callback<((String))>,
}

pub enum Msg {
  ModalToggle,
  // ForceUpdate,
  UpdateSearchTerm(InputEvent)
}

impl Component for Flashlinks {
  type Message = Msg;
  type Properties = Flashlinksprops;


  fn create(ctx: &Context<Self>) -> Self {
    let is_mobile = js! {
        return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
    };
    Self { 
      is_mobile: is_mobile == true,
      open_modal: false, 
      search_term: "".to_string(),
      // forceupdate: ctx.props().forceupdate.clone()
      // search_term_handler: ctx.props().search_term_handler.clone()
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    if first_render{
      spawn_local(async move {
        let returnhtml = get_blog_html().await;
        log::info!("value of returnhtml: {:?}", returnhtml);
      });
    }
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
      },
      // Msg::ForceUpdate => {
      //   ctx.props().forceupdate.emit(());
      // }
      Msg::UpdateSearchTerm(e)=>{
        let letter = e.data().unwrap_or("backspace".to_string()).to_string();
        if letter != "backspace"{
          self.search_term = self.search_term.to_string() + letter.as_str();
        }else{
          let teststr = &self.search_term[0..&self.search_term.len() - 1];
          self.search_term = teststr.to_string();
        }
        log::info!("value of self.search_term: {:?}", self.search_term);
        // test_fn();
        // let returnhtml = get_blog_html().await;
        // log::info!("value of returnhtml: {:?}", returnhtml);
        // let handle = std::thread::spawn(|| {
        //   let returnhtml = get_blog_html().await;
        //   log::info!("value of returnhtml: {:?}", returnhtml);
        // });
        // handle.join().unwrap();

        // ctx.props().search_term_handler.emit((self.search_term));
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
          body, html{
            padding: 0;
            margin: 0;
          }
          .main{
            background: url('static/background2.jpg') !important;
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
            background-image: url('static/portsky.jpg') !important;
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
            background-image: linear-gradient(to top right, rgba(0,0,0,0.5) 0%,rgba(0,255,0,0.5) 100%), url('static/banner.png') !important;
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
            background-image: url('static/octopusclean.png') !important;
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

          .webringcontainer{
            display: inline-block;
            text-align: center; 
            width: 20vw;
            background: orange;
            color: blue;
          }

          .wildlyinappropriate{
            display: inline-block;
            text-align: center; 
            width: calc(20vw - 20px);
            background: lightgreen;
            color: darkbrown;
            margin: 5px;
            padding: 5px;
          }

          .archivebuttonscontainer{
            display: inline-block;
            position: relative;
            text-align: left;
            padding: 5px;
            width: 20vw;
            filter: invert(0%);
          }

          .archivebuttons{
            filter: invert(0%);
            transition: 0.5s;
            cursor: pointer;
          }

          .archivebuttons:hover{
            filter:invert(100%);
            transition: 1s;
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
              opacity: 0;
            }
            .learnmore{
              height: 0;
              width: 0;
              border: 0;
              padding: 0;
              opacity: 0;
            }
            .octopus{
              width: calc(20vw + 5px);
            }
            .picturemapcontainer{
              width: 0; 
              height: 0;
              opacity: 0;
            }
            .skull{
              width: 0;
              height: 0;
              opacity: 0;
            }
            .webringcontainer{
              width: 0;
              height: 0;
              opacity: 0;
            }
            .wildlyinappropriate{
              width: 0;
              height: 0;
              opacity: 0;
              margin: 0;
              padding: 0;
            }
            .archivebuttons{
              width: 0;
              height: 0;
              opacity: 0;
              margin: 0;
              padding: 0;
            }
            .archivebuttonscontainer{
              width: 0;
              height: 0;
              opacity: 0;
              margin: 0;
              padding: 0;
            }
          }

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

          .blueprint-button {
            display: inline-block;
            width: calc(100% - 27px); 
            padding: 4px 8px;
            font-size: 14px;
            font-weight: 600;
            line-height: 1.5;
            text-align: center;
            white-space: nowrap;
            vertical-align: middle;
            cursor: pointer;
            margin-bottom: 5px;
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

        <div class="learnmorecontainer">
          {
            if self.open_modal{
              html!{
                <>
                  {self.buy_writing_modal(ctx)}
                </>
              }
            }else{
              html!{
                <div/>
              }
            }
          }
          <div class="learnmore">
            {
              if !self.open_modal{
                html!{
                  <>
                    <div class="skull">
                      <img loading="lazy" src={"static/skull.png"} style="height: 100%; width: 100%;"/>
                    </div>
                  </>
                }
              }else{
                html!{<div/>}
              }
            }
            <h3>
              {"Buy my writing!"}
            </h3>
            <div 
              class="blueprint-button" 
              style="width: calc(100% - 16px); margin-left: 0px;"
              onclick={ctx.link().callback(|_| Msg::ModalToggle)}
            >
              {"Learn More"}
            </div>
          </div>
          <div style="
            display: inline-block; background: black; color: lightgreen; font-weight: bold;
            width: 100%; margin-top: 5px; font-size: 10px; text-align: left; padding-left: 5px;
          ">
            <p>
              {"Display blog posts by month: "}
            </p>

            <Link<Route> to={Route::HomeMonth{ month: "".to_string()}}>
              <div class="blueprint-button">
                {"All Time"}
              </div>
            </Link<Route>>

            <Link<Route> to={Route::HomeMonth{ month: "april".to_string()}}>
              <div class="blueprint-button">
                {"April, 2023"}
              </div>
            </Link<Route>>

             <Link<Route> to={Route::HomeMonth{month: "march".to_string()}}>
              <div class="blueprint-button">
                {"March, 2023"}
              </div>
            </Link<Route>>
            
            <p>
              {"Input a search term to search the blog: "}
            </p>
            <input type="text" 
              style="margin-bottom: 5px; width: calc(100% - 22px); padding-right: 5px;"
              value={self.search_term.clone()}
              autocomplete="off"
              oninput={ctx.link().callback(|e| Msg::UpdateSearchTerm(e))}
            />
          </div>
          <div class="picturemapcontainer">
            <Link<Route> to={Route::Map}>
              <img loading="lazy" src={"static/picturebutton.png"} class="picturemap"/>
            </Link<Route>>
          </div>
          <div class="webringcontainer">
            <a href={"https://xn--sr8hvo.ws/%F0%9F%90%B0%F0%9F%8C%A6%F0%9F%9B%A3/previous"}>{"←"}</a>
            <a href={"https://xn--sr8hvo.ws"}>{" Indie Webring "}</a>
            <a href={"https://xn--sr8hvo.ws/%F0%9F%90%B0%F0%9F%8C%A6%F0%9F%9B%A3/next"}>{"→"}</a>
          </div>
          <div class="wildlyinappropriate">
            {"And without further ado, here's a link to my neocities account - which is just an ascii art image of a cat's asshole. IN LIVING COLOR - "}<a href={"https://wildlyinappropriate.neocities.org/"}>{"wildlyinappropriate.neocities.org/"}</a>
          </div>
          <div class="wildlyinappropriate" style="background: orange;">
            {"And here's a bunch of cool gifs from some nutter on IRC, the internet's asshole: "}<a href={"https://cantelope.org/recentGifs/"}>{"shiny"}</a>
          </div>
          <div class="archivebuttonscontainer">         
            <div
            style="width: calc(100% - 10px); text-align: left;
              position: absolute; top: 4px; left: 0; right: 0; z-index: 3"
            >
              <div style="background: rgba(0,0,0,0.7); color: black; font-weight: bold; font-size: 0.8rem; color: white; padding: 5px; margin-bottom: 2px;">
                {"A list of archived material from 4chan"}
                <br/>
                {"books, microcode, and high speed pizza delivery"}
              </div>
              <div class="archivebuttons" style="width: 80px; height: 31px; background: rgba(0,0,0,0); margin-left: 5px; display: inline-block;">
                <Link<Route> to={Route::Scififantasy}>
                  <img src={"static/buttons/scififantasy.png"}
                    style="width: 100%; height: 100%;"
                  />
                </Link<Route>>
              </div>
              <div class="archivebuttons" style="width: 80px; height: 31px; background: rgba(0,0,0,0); margin-left: 5px; display: inline-block;">
                <Link<Route> to={Route::Classical}>
                  <img src={"static/buttons/classical.png"}
                    style="width: 100%; height: 100%;"
                  />
                </Link<Route>>
              </div>
              <div class="archivebuttons" style="width: 80px; height: 31px; background: rgba(0,0,0,0); margin-left: 5px; display: inline-block;">
                <Link<Route> to={Route::Literature}>
                  <img src={"static/buttons/literature.png"}
                    style="width: 100%; height: 100%;"
                  />
                </Link<Route>>
              </div>
              <div class="archivebuttons" style="width: 80px; height: 31px; background: rgba(0,0,0,0); margin-left: 5px; display: inline-block;">
                <Link<Route> to={Route::Historybooks}>
                  <img src={"static/buttons/history.png"}
                    style="width: 100%; height: 100%;"
                  />
                </Link<Route>>
              </div>
              <div class="archivebuttons" style="width: 80px; height: 31px; background: rgba(0,0,0,0); margin-left: 5px; display: inline-block;">
                <Link<Route> to={Route::Philosophy}>
                  <img src={"static/buttons/philosophy.png"}
                    style="width: 100%; height: 100%;"
                  />
                </Link<Route>>
              </div>
              <div class="archivebuttons" style="width: 80px; height: 31px; background: rgba(0,0,0,0); margin-left: 5px; display: inline-block;">
                <Link<Route> to={Route::Politics}>
                  <img src={"static/buttons/politics.png"}
                    style="width: 100%; height: 100%;"
                  />
                </Link<Route>>
              </div>
              <div class="archivebuttons" style="width: 80px; height: 31px; background: rgba(0,0,0,0); margin-left: 5px; display: inline-block;">
                <Link<Route> to={Route::Religion}>
                  <img src={"static/buttons/religion.png"}
                    style="width: 100%; height: 100%;"
                  />
                </Link<Route>>
              </div>
              <div class="archivebuttons" style="width: 80px; height: 31px; background: rgba(0,0,0,0); margin-left: 5px; display: inline-block;">
                <Link<Route> to={Route::Academic}>
                  <img src={"static/buttons/academic.png"}
                    style="width: 100%; height: 100%;"
                  />
                </Link<Route>>
              </div>
              <div class="archivebuttons" style="width: 80px; height: 31px; background: rgba(0,0,0,0); margin-left: 5px; display: inline-block;">
                <Link<Route> to={Route::Horror}>
                  <img src={"../static/buttons/horror.png"}
                    style="width: 100%; height: 100%;"
                  />
                </Link<Route>>
              </div>
            </div>  
            <div style="position: absolute; display: inline-block; top: 0; left: 0; width: 20vw; z-index: 1;">
              <img src={"static/hazard.png"}
                style="width: 100%; height: 100%;"
              />     
            </div>
          </div>
        </div>
      </>
    }
  }
}

impl Flashlinks{
  fn buy_writing_modal(&self, ctx: &Context<Self>) -> Html {
    html!{
      <>
        <div style="display: inline-block; position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.8); text-align: center; z-index: 20;">
          <div class="learnmore" style="margin-top: 20vh; width: 60vw; text-align: left; margin-right: calc(20vw - 10px);">
            <h3>
              {"Request some writing"}
            </h3>
            <p>
              {"If you want to request some writing and would like me to pay me for it then send me an email at peterweyand0@gmail.com. My paypal email is patientplatypus6@gmail.com - I'll have you send me a couple bucks over paypal after we discuss via email what you want me to write about. It is what it is - I'm just a guy that will sell you the unvarnished truth as I see it on a subject of your choice. No promises you're going to like it - my integrity is not for sale. I'm not going to set up a paypal automated link here because I wouldn't trust me to not fuck it up and I wouldn't trust you not to trust me not fuck it up. No jabronis."}
            </p>
            <div style="width: 100%; text-align: right;">
              <div 
                class="blueprint-button" 
                style="width: calc(10% - 16px); margin-left: 0px;"
                onclick={ctx.link().callback(|_| Msg::ModalToggle)}
              >
                {"Close"}
              </div>
            </div>
          </div>
        </div>
      </>
    }
  }
}