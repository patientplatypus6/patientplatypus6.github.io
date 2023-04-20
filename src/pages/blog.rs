use yew::prelude::*;
use regex::Regex;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsValue;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, Element, EventTarget, HtmlInputElement, HtmlImageElement, IntersectionObserver, IntersectionObserverEntry, HashChangeEvent};
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
use yew::Html;
use yew::AttrValue;
use yew::virtual_dom::vnode::VNode;


// use super::super::components::flashlinks::Flashlinks;
// use super::super::components::header::Header;
// use super::super::components::navlinks::Navlinks;

use super::super::components::flashlinksblog::Flashlinks;
use super::super::components::headerblog::Header;
use super::super::components::navlinksblog::Navlinks;


use super::super::util::blogs;
use super::super::util::blogs::Blog;

// #[derive(Clone, PartialEq, Properties)]
pub struct Blogpage{
  pub month: String,
  pub day: String, 
  pub time: String,
  pub location: String,
  pub searchterm: String,
  _listener: LocationHandle,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Blogpageprops{
  pub month: String,
  pub day: String, 
  pub time: String
}

pub enum Msg {
  SearchHandler(String),
  PageUpdated(Location)
}

impl Component for Blogpage {
  type Message = Msg;
  type Properties = Blogpageprops;

  fn create(ctx: &Context<Self>) -> Self {
    log::info!("inside the create function for home; ");
    let link = ctx.link().clone();
    let listener = ctx.link()
    .add_location_listener(link.callback(move |e| Msg::PageUpdated(e)))
    .unwrap();

    Self{
      month: ctx.props().month.clone(),
      day: ctx.props().day.clone(),
      time: ctx.props().time.clone(),
      location: "".to_string(),
      _listener: listener,
      searchterm: "".to_string()
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg{
      Msg::SearchHandler(e)=>{
        log::info!("Inside SearchTerm in home and value; {:?}", e);
        self.searchterm = e.to_string();
      }, 
      Msg::PageUpdated(e) => {
        log::info!("Inside the page updated handler!");
        log::info!("Value of self.month: {:?}", self.month.clone());
        log::info!("value of location: {:?}", e.path().to_string());
        self.location = e.path().to_string();
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

            .hitcounter{
              background: darkpurple !important;
              color: lightorange !important;
              padding: 0px;
              padding-top: 5px;
              margin: 0px;
              font-style: none;
              font-weight: bold;
              width: 100%;
              font-size: 0.8rem;

            }
            
            .LTF_hitcounter{
              margin-bottom: 0px;
            }

          "}
        </style>
        <div class="main">
          <Flashlinks
            search_handler = {ctx.link().callback(|data| Msg::SearchHandler(data))}
          />
          <Navlinks/>
          
          <div class="octopus"/>

          <div class="contentcontainer">

            <Header/>

            {self.display_blog(ctx)}

          </div>
        </div>
      </>
    }
  }
}

impl Blogpage{

  fn blog_formatting(&self, ctx: &Context<Self>, blog: Blog) -> Html {
    html!{
      <>
        <Link<Route> to={Route::Blogpage{ month: self.month.clone(), day: self.day.clone(), time: self.time.clone()}}>
          <div class="timeheader">
            { Html::from_html_unchecked(AttrValue::from(blog.date.clone()))} {" "}
            { Html::from_html_unchecked(AttrValue::from(blog.time.clone()))} {" "}
            { Html::from_html_unchecked(AttrValue::from(blog.location.clone()))}
            {" - The Time Is Now"}
          </div>
        </Link<Route>>
        <h3>
          { Html::from_html_unchecked(AttrValue::from(blog.title.clone()))}
        </h3>
        { Html::from_html_unchecked(AttrValue::from(blog.paragraphs.clone().replace("static/", "../../../static/"))) }
      </>
    }
  }

  fn display_blog(&self, ctx: &Context<Self>) -> Html {
    let blogs = blogs::blogs().clone();
    let route = "/".to_string() + self.month.as_str() + "/" + self.day.as_str() + "/" + self.time.as_str();
    log::info!("value of route: {:?}", route);
    log::info!("value of location: {:?}", self.location);
    let blog = blogs::find_blog_by_route(&blogs, &route);
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }
}


