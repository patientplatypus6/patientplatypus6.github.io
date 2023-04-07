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

use super::super::components::flashlinks::Flashlinks;

#[derive(Clone, PartialEq, Properties)]
pub struct About{

}

pub enum Msg {

}

impl Component for About {
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
            min-height: 100vh;
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
              <img loading="lazy" src={"static/title.png"} class="imagetag"/>
            </h1>      
            <div>
              <div class="timeheader">
                {"This is the about page. This is about me."}
              </div>    
              <p>
                {"My name is Peter Teller Weyand. I'm 37, I'm near sighted, have bad teeth and I'm homeless. I'm of slightly above average intelligence and slightly below average looks. I'm 6 feet tall when I don't slouch. I've been abandoned by all my friends and family because I couln't find a job. I asked my sister if I could put up a tent in her yard until I could find a job and she paid me $300 to go away. My father says he wished I would die of the plague."}
              </p>
              <p>
                {"My father once told me that Weyands don't lie, cheat or steal. So I don't. It's kept me in good stead - it means that I have kept my self respect enough to go tell the rest of you dog fuckers that you suck. The people in the Tenderloin think this makes me either a policeman or a priest. Because when I think open mindedness and expansive world view I think the Tenderloin in San Francisco."}
              </p>
              <p>
                {"Oh. By the way. I have used generative AI art throughout this piece without attribution and will continue to do so! If you can prove that you own the rights to the works that created the content that your AI algorithm created, then I will take down this art. Or if you can prove that your use of AI is novel art, and somehow my repurposing of your work is NOT novel art, then I will be happy to take down the images and replace them. Otherwise go fuck yourself."}
              </p>
              <p>
                {"I'm not going to post my resume, because none of you dog fuckers would hire me. And if you did I doubt you'd be able to keep in business for 6 months as the economy goes down in flames. I won't post a picture because I'm not a whore and how I look shouldn't matter. You've passed me on the street and kept on walking. I won't post a link to social media, because social media is a symptom of a much larger disease."}
              </p>
              <p>
                {"This is my email. peterweyand0@gmail.com. If you have something to say type it out and push send. Otherwise suck your own dick. Yoga is your friend."}
              </p>
            </div>
          </div>
        </div>
      </>
    }
  }
}

impl About{

}