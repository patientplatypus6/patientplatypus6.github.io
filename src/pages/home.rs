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


use super::super::components::flashlinks::Flashlinks;
use super::super::components::header::Header;
use super::super::components::navlinks::Navlinks;
use super::super::util::blogs;
use super::super::util::blogs::Blog;

// #[derive(Clone, PartialEq, Properties)]
pub struct Home{
  pub month: String,
  pub searchterm: String,
  _listener: LocationHandle,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Homeprops{
  pub month: String
}

pub enum Msg {
  SearchHandler(String),
  PageUpdated(Location)
}

impl Component for Home {
  type Message = Msg;
  type Properties = Homeprops;

  fn create(ctx: &Context<Self>) -> Self {
    log::info!("inside the create function for home; ");
    let link = ctx.link().clone();
    let listener = ctx.link()
    .add_location_listener(link.callback(move |e| Msg::PageUpdated(e)))
    .unwrap();

    Self{
      month: ctx.props().month.clone(),
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
        self.month = e.path().to_string();
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

            {
              if self.searchterm == "".to_string(){
                if self.month.clone() == "/april".to_string(){
                  html!{
                    <>
                      {self.apr20(ctx)}
                      {self.apr19(ctx)}
                      {self.apr18(ctx)}
                      {self.apr17(ctx)}
                      {self.apr16(ctx)}
                      {self.apr15(ctx)}
                      {self.apr14(ctx)}
                      {self.apr13(ctx)}
                      {self.apr12(ctx)} 
                      {self.apr11(ctx)}
                      {self.apr10(ctx)}
                      {self.apr9(ctx)}
                      {self.apr8(ctx)}
                      {self.apr7(ctx)}
                      {self.apr6(ctx)}
                      {self.apr5(ctx)}
                      {self.apr4(ctx)}
                      {self.apr3(ctx)}
                      {self.apr2(ctx)}
                      {self.apr1(ctx)}
                    </>
                  }
                }else if self.month.clone() == "/march".to_string(){
                  html!{
                    <>
                      {self.mar31(ctx)}
                      {self.mar30(ctx)}
                      {self.mar29(ctx)}
                    </>
                  }
                }else{
                  html!{
                    <>
                      {self.apr20(ctx)}
                      {self.apr19(ctx)}
                      {self.apr18(ctx)}
                      {self.apr17(ctx)}
                      {self.apr16(ctx)}
                      {self.apr15(ctx)}
                      {self.apr14(ctx)}
                      {self.apr13(ctx)}
                      {self.apr12(ctx)} 
                      {self.apr11(ctx)}
                      {self.apr10(ctx)}
                      {self.apr9(ctx)}
                      {self.apr8(ctx)}
                      {self.apr7(ctx)}
                      {self.apr6(ctx)}
                      {self.apr5(ctx)}
                      {self.apr4(ctx)}
                      {self.apr3(ctx)}
                      {self.apr2(ctx)}
                      {self.apr1(ctx)}
                      {self.mar31(ctx)}
                      {self.mar30(ctx)}
                      {self.mar29(ctx)}
                    </>
                  }
                }
              }else{
                html!{self.searchview(ctx)}
              }
            }

          </div>
        </div>
      </>
    }
  }
}

impl Home{

  fn blog_formatting(&self, ctx: &Context<Self>, blog: Blog) -> Html {
    let s = blog.route.clone();
    let mut parts = s.split("/");
    let month = parts.nth(1).unwrap().to_string();
    let day = parts.nth(0).unwrap().to_string();
    let time = parts.nth(0).unwrap().to_string();
    html!{
      <>
        <Link<Route> to={Route::Blogpage{ month, day, time}}>
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
        { Html::from_html_unchecked(AttrValue::from(blog.paragraphs.clone())) }
      </>
    }
  }

  fn searchview(&self, ctx: &Context<Self>) -> Html {
    let blogs = blogs::blogs().clone();
    let blogdisp = blogs::find_substring(&blogs.clone(), &self.searchterm.clone());
    log::info!("value of blogdisp: {:?}", blogdisp.clone().len().to_string());
    html!{
      <>
        {
          for blogdisp.iter().enumerate().map(|(i, blog)| {
            log::info!("value of blog paragraph {:?}", blog.clone().paragraphs);
            html!{{self.blog_formatting(ctx, blog.clone())}}
          })
        }
      </>
    }
  } 

  fn find_blog(&self,time_to_find: &str, date_to_find: &str) -> Blog {
    let blogs = blogs::blogs().clone();
    let blogindex = blogs::find_index_by_time_date(&blogs.clone(), time_to_find, date_to_find).unwrap();
    blogs[blogindex].clone()
  }

  fn apr20(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>7:19PM</span>", "<span>Thur Apr 20</span>");
    let blog2 = self.find_blog("<span>11:28PM</span>", "<span>Thur Apr 20</span>");
    html!{
      <>
        {self.blog_formatting(ctx, blog.clone())}
        {self.blog_formatting(ctx, blog2.clone())}
      </>
    }
  }

  fn apr19(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>2:12PM</span>", "<span>Wed Apr 19</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }


  fn apr18(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>1:31PM</span>", "<span>Tue Apr 18</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr17(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>6:19PM</span>", "<span>Mon Apr 17</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr16(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>6:19PM</span>", "<span>Sun Apr 16</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr15(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>12:53PM</span>", "<span>Sat Apr 15</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr14(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>8:59PM</span>", "<span>Fri Apr 14</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr13(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>10:22PM</span>", "<span>Thur Apr 13</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr12(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>8:40PM</span>", "<span>Wed Apr 12</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr11(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>11:59PM</span>", "<span>Tues Apr 11</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr10(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>10:11AM</span>", "<span>Mon Apr 10</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr9(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>10:23AM</span>", "<span>Sun Apr 9</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr8(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>7:50AM</span>", "<span>Mon Apr 8</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr7(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>9:28PM</span>", "<span>Fri Apr 7</span>");
    let blog2 = self.find_blog("<span>5:26PM</span>", "<span>Fri Apr 7</span>");
    html!{
      <>
        {self.blog_formatting(ctx, blog.clone())}
        {self.blog_formatting(ctx, blog2.clone())}
      </>
    }
  }

  fn apr6(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>6:16PM</span>", "<span>Thur Apr 6</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr5(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>8:22AM</span>", "<span>Wed Apr 5</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr4(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>7:10PM</span>", "<span>Tue Apr 4</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr3(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>2:33PM</span>", "<span>Mon Apr 3</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn apr2(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>11:28PM</span>", "<span>Sun Apr 2</span>");
    let blog2 = self.find_blog("<span>11:03PM</span>", "<span>Sun Apr 2</span>");
    let blog3 = self.find_blog("<span>5:01PM</span>", "<span>Sun Apr 2</span>");
    let blog4 = self.find_blog("<span>1:11PM</span>", "<span>Sun Apr 2</span>");
    html!{
      <>
        {self.blog_formatting(ctx, blog.clone())}
        {self.blog_formatting(ctx, blog2.clone())}
        {self.blog_formatting(ctx, blog3.clone())}
        {self.blog_formatting(ctx, blog4.clone())}
      </>
    }
  }

  fn apr1(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>1:05PM</span>", "<span>Sat Apr 1</span>");
    html!{{self.blog_formatting(ctx, blog.clone())}}
  }

  fn mar31(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>7:40PM</span>", "<span>Fri Mar 31</span>");
    let blog2 = self.find_blog("<span>9:15PM</span>", "<span>Fri Mar 31</span>");
    html!{
      <>
        {self.blog_formatting(ctx, blog.clone())}
        {self.blog_formatting(ctx, blog2.clone())}
      </>
    }
  }

  fn mar30(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>2:27PM</span>", "<span>Thur Mar 30</span>");
    let blog2 = self.find_blog("<span>11:20AM</span>", "<span>Thur Mar 30</span>");
    html!{
      <>
        {self.blog_formatting(ctx, blog.clone())}
        {self.blog_formatting(ctx, blog2.clone())}
      </>
    }
  }

  fn mar29(&self, ctx: &Context<Self>) -> Html {
    let blog = self.find_blog("<span>7:03PM</span>", "<span>Wed Mar 29</span>");
    let blog2 = self.find_blog("<span>6:31PM</span>", "<span>Wed Mar 29</span>");
    let blog3 = self.find_blog("<span>5:39PM</span>", "<span>Wed Mar 29</span>");
    let blog4 = self.find_blog("<span>5:20PM</span>", "<span>Wed Mar 29</span>");
    html!{
      <>
        {self.blog_formatting(ctx, blog.clone())}
        {self.blog_formatting(ctx, blog2.clone())}
        {self.blog_formatting(ctx, blog3.clone())}
        {self.blog_formatting(ctx, blog4.clone())}
      </>
    }
  }

}


