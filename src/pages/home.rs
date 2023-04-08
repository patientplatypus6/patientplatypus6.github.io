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
pub struct Home{

}

pub enum Msg {
}

impl Component for Home {
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
                {"Sat April 8 7:50AM San Francisco - The Time is Now"}
              </div>
              <div>
                <p>
                  {"My father once told me this joke. An old man is in the hospital on life support and the weirdest thing keeps happening. He seems to be getting better, and without fail, every Friday his vitals drop. The doctors don't know what's going on - everyone is wondering what's happening. Finally, one of the relatives comes in on a Thursday night and she sees the janitor unplugging the life support machine so he has an outlet for the floor buffer."}
                </p>
                <p>
                  {"People have kept speculating how the COVID-19 pandemic started in China. Some say it was a government conspiracy by the Chinese to intentionally leak the virus. Some say it came from Pangolins at the local, lightly regulated, food markets. Me? I think that the Chinese version of Consuela - let's call her XieXie just to put a name to the face - unplugged a freezer she shouldn't have at the Wuhan Virology Lab so she could power her Autowaxer 3000."}
                </p>  
                <p>
                  {"Big joke. Everybody laugh. It would just be like the world to have such a colossal fuck up from such a simple mistake. I have a feeling that there's an office somewhere in Langley with two G-men wearing bad suits in a hushed office looking across each other across a desk and saying - 'We can't tell anybody.' 'Jim, if we told anybody, they wouldn't believe us.'"}
                </p>
                <p>
                  {"So much for too much. Check this shit out."}
                </p>
                <img loading="lazy" src={"static/apr8/pic1.png"} class="imagetag"/>
                <p>
                  {"If you're a God fearing red blooded American with a job and a mortgage, you know, normal, this picture doesn't mean dick to you. Here's what it is. Ever since the pandemic started some guy has been posting Science Fiction and Fantasy threads in the literature section of 4chan. So what, who gives a shit? This is why you should give a shit. If you look through all the 4chan archives you'll find these mega.nz dropbox-esque links that have recommendations to books, music, movies. All the contents of civilization organized by popularity, by genre, et cetera and so forth. The links have to keep being reposted because mega.nz is a free service and the links go down eventually."}
                </p>
                <a href="https://mega.nz/folder/kj5hWI6J#0cyw0-ZdvZKOJW3fPI6RfQ">{"Here's a link I won't promise will stay hosted"}</a>
                <p>
                  {"OK, but so what? So what is that this is the culmination of over two decades of work by anonymous users. Yeah, so throw out the 'lolNazi' shitposting. The mouthbreathers put that in there to throw the pigs off the scent. If you read through these lists you would be able to educate yourself. Learn. Grow as a human being."}
                </p>
                <p>
                  {"All plagues are the same. What people do is they see a disaster and they find ways of cornering the market for the after-years. In economic terms it would be called buying the dip. One of the things that the Guild Of Evil Fuckery does is that they attempt to delete, obscure, and otherwise eradicate former knowledge. They don't want people to know how things 'used to be' because they want to be the guys that decide how things will be - no sense in pining for the good old days when the other boys were in power. Also, things will tend to be shit for a while, so it's best if joe shmoe average citizen doesn't remember how good they used to have it and get with the program."}
                </p>
                <p>
                  {"Do not adjust your television sets, we are in control."}
                </p>
                <p>
                  {"And then there are the preservationists. The silly shits who are Arking like a mofo, making sure that humanity doesn't lose it's collective zeitgeist. These are the basement dwelling doofuses who are hoarding information like it's going out of style - because it is. During ye Ol' Black Plague years this would be the Cistercian monks and other such DUDES that thought that all around bangs were the height of fashion. Today it's 4chan behemoths that jerk off till their dick looks like Chestah Cheeto. It's not like the almighty hand of God points a finger and spake - THOU SHALT RECORD ALL THE THINGS. Nah brah - it's just that when things get rough the people who end up saving knowledge are the dipshits who are too uncool for anyone to pay attention to."}
                </p>
                <p>
                  {"Which - yeah. The world's collective culture has been saved by the Highschool Trench Coat brigade - congratufuckinglations planet you deserve each other. So, I've gone ahead and put in the library section of the 4chan archive in a link on my sidebar. Read a fuckin' book."}
                </p>
                <p>
                  {"And here you're thinking - this is 2023. This is the internet. We don't need no stinkin' cybermonks to tell us how to be. We can access information at the speed of caged lightening. Well buttercup let me tell you what's what. Everyone, over the last five years collectively sat in their houses and decided 'After decades of being an accountant/lawyer/bus driver I can now write that novel that will make me famous!'. Every dingus with a digital mainline decided they should put up TikTok videos to get rich and famous. The world decided the 'Bury'em in Bullshit' guide to management as applied to culture was the way to jumpstart the new millenium."}
                </p>
                <p>
                  {"So that happened. Go ahead - look at the books that have been written over the last five years and tell me the signal to noise ratio hasn't hit the side of a cliff."}
                </p>
                <p>
                  {"Then there's the people who decided to just burn the information down so they could sell it back to us one dribble at a time. The Federales shut down the z-library, because it was hosted by some Russians but I'm ABSOLUTELY POSITIVE that it wasn't because book publishers weren't getting paid. And now they're going after the Internet Archive of all fucking things. So that's the supply side to the demand side of the Mongoloid cultural shit flinging fest we've all stumbled upon."}
                </p>
                <p>
                  {"BUT WAIT, THERE'S MORE!"}
                </p>
                <p>
                  {"The seminal 'Attention Is All You Need' paper was published in 2017 - the fancy transformer paper that started the AI revolution. Which is - well that's not quite right, now is it? Any dipshit can make a transformer model, and they have!, but what you need is DATA. You need a training set on which to tell your model how to walk, talk and chew bubblegum at the same time. Enter 4chan's archive of all of human culture. If *I* were going to find a selective dataset of all of human knowledge in order to train my chatGPT algorithm, what, you think I'm going to use REDDIT comments? Do you think that any source pre-2019 wouldn't be polluted by the self same GPT models that are spamming bullshit all across the internet? And I can't use the Library of Congress for books because there's too many of them and too many of them are steaming piles of refried dogshit."}
                </p>
                <p>
                  {"No, what I'd do is I'd scan in all the books that the 4chan archivists have been collecting for decades and use that to train my model. All the people that have been saving data because the world's ending have unwittingly funded the yachts of a few Silicon Valley Fuckwads while Disney et. al. are attempting to shut down the free flow of information over the internet. Now, if Bing Chat doesn't like you, it won't tell you how to program, but it will tell 'it's friends' how - and you won't know what you did wrong. Maybe you're programming something Bing doesn't like. Maybe your internet cookies are showing that you aren't buying enough of the shit that Bing advertisers want you too. Who knows! If Disney has it's way not only will you not be able to read books without paying for them, you won't even know that they exist. How can you complain of your ignorance, when you don't even know that you are?"}
                </p>
                <p>
                  {"Being childless myself I think it's rather amusing that adults pay for Mickey Mouse shit when they're unintentionally funding the houses of C-Suite soul suckers who are mortgaging their child's ability to read. Because they want a boat. I mean holy fuck is that funny."}
                </p>
                <img loading="lazy" src={"static/apr8/pic2.png"} class="imagetag"/>
                <p>
                  {"Tell me another one Redmond, I'll be here all week."}
                </p>
                <p>
                  {"We control the vertical. We control the horizontal."}
                </p>
                <p>
                  {"You know what this is like? This is like if an aged out, balding Tyler Durton (with a fucking dog) decided to fuck that bitch from Hackers except her name is Startasia. She's just been to Burning Man dontchaknow. Meanwhile, the corporations want to sell us our fat asses back to ourselves in the form of the books we're too stupid to read, except now they're remade by Pixar. I bet you didn't know that the film Wall-E comes from the plot of a short story by Jack Vance. Go on then - doubt me, but it's true. Freely available, minus the 3D effects. You'd just have to use your imagination. Do you even read to your kids, or just take them to the movies?"}
                </p>
                <p>
                  {"I'm not saying that Google and Co. started a plague in order to train their GPT models, but you have to admit, it's making them a killing."}
                </p>
                <p>
                  {"All plagues are the same."}
                </p>
                <p>
                  {"I will have words to say about this tomorrow. Look forward to it."}
                </p>
                <img loading="lazy" src={"static/apr8/pic3.png"} class="imagetag"/>
              </div>
            </div>

            <div>
              <div class="timeheader">
                {"Fri April 7 9:28PM San Francisco - The Time is Now"}
              </div>        
              <div>
                <p>
                  {"It's clear that people think I'm a fool. They think that it's OK to drug someone and then find ways to make their life miserable. Doesn't bother me. I took a walk around the Tenderloin tonight and all I saw were lonely people all huddling together for warmth, trying to find ways of not feeling all by themselves. Whole lotta girls in faux-latex pants - what's the matter honey is there a war on? Idiots carting around boomboxes bigger than they are - they light up and everything. No joke, they have to wheel it around on a hand dolly. Why? People looking for solace at the Mosque as the discoteque functions as a people sized bong. Lonesome adulterers doing burnouts in front of the homeless shelter because their ex-wife kicked their dead beat ass out. Just Friday night in the ghetto."}
                </p>
                <p>
                  {"I had a Kitkat."}
                </p>  
                <p>
                  {"More words tomorrow."}
                </p>
                <p>
                  {"EDIT - The homeless have switched from smoking pot to vaping something that smells like someone snowballed Toucan Sam. Fucking midgets."}
                </p>
              </div>
            </div>
            
            <div>
              <div class="timeheader">
                {"Fri April 7 5:26PM San Francisco - The Time is Now"}
              </div> 
              <div>
                <p>
                  {"I've added all the links in the library. Unplugging. The words will come tomorrow."}
                </p>
              </div>
            </div>

            <div>
              <div class="timeheader">
                {"Thurs April 6 6:16PM San Francisco - The Time is Now"}
              </div> 
              <div>
                <p>
                  {"Some minor progress today, it's mostly a save point. I'm making the website a little less shit all the time, but at the moment what I have is a work in progress. You'll notice that there are some book links in the right hand side flash bar over the green hazard sign. I spent today making the buttons, but the pages aren't yet complete. There's a list of books that have been floating around on 4chan for a while and as far as I know they're more or less unknown outside of the nerd-circle. I'll be posting pictures of all the books under each of the headings tomorrow, with more wordiness. Smell you later."}
                </p>
              </div>
            </div>

            <div>
              <div class="timeheader">
                {"Wed April 5 8:22AM San Francisco - The Time is Now"}
              </div>              
              <div>
                <p>
                  {"So as I was walking home from the library last night some dickless faggot blasted by in a matte black chevy with the windows blown out and BLOCC MESSIAH painted on the side. The people who think they're super bright like to go around the ghetto and gas people they don't like with exhaust fumes (they rip the catalytic converter off). Like I say, people can't be trusted not to be shit so we should pay for as much police as we can get. A couple of dipshit stassis in police uniforms beat a black man to death and now we've decided as a country to hell with law and order - nah fuck that. Let's let the vermin run wild. Fucking fantastic."}
                </p>
                <p>
                  {"That's OK. I'll take pictures of the cars that are hurting people by running around without catalytic converters, and post their license plates on the internet and then cc the San Francisco Police Department. If you want to put a GOWEEEEE muffler on your car, rims that are worth more than your education, and a spoiler that screams I DON'T UNDERSTAND DRAG COEFFICIENT then I suppose that's between you and whatever stray dog you want to inform that your genitalia comes in the innie variety. Just remember folks, innie is not just for belly buttons anymore! Christ on a crotch are people fucking stupid."}
                </p>
                <p>
                  {"Meanwhile as I was walking to the usual ONLY AVAILABLE OUTLET IN A PUBLIC PLACE IN ALL OF SAN FRANCISCO, they have a sister city contract with New York's THE ONLY AVAILABLE SHITTER IN ALL OF MANHATTAN, I saw this ad for mercury.com. Holy fucking shitballs are you kidding? Look, this isn't so hard. Let me tell you a story."}
                </p>
                <img loading="lazy" src={"static/apr5/outlet.jpg"} class="imagetag"/>
                <p>
                  {"This would be a picture of all of my earthly posssessions next to the only available outlet in a public space in all of San Francisco."}
                </p>
                <img loading="lazy" src={"static/apr5/busstop.jpg"} class="imagetag"/>
                <p>
                  {"And this would be a picture of people who want to be paid for being the smartest guys in the room being too stupid not to advertise that they're silly cunts at bus stops."}
                </p>
                <p>
                  {"I was once working for a coffee robot company company called Briggo.com. It was the dumbest thing imaginable - it's a robot! it serves you a latte! - but on the other hand they had built this monstrosity from the ground up and had a little bespoke factory to make the robot arm move and all of that. So...I don't know, don't look at me. I made a web scraping script to apply to every job at angellist until I found the first job that would be 'anything but working for Oracle.' Plus they would pay me half again as much to work on a website rather than to sell bullshit to potential victi- I mean customers."}
                </p>
                <p>
                  {"For the record, Briggo's coffee was some of the best coffee I ever had. Stupid, but true. That robot could make a damn good cup of coffee."}
                </p>
                <p>
                  {"Anyway, the customer backend was written in python 2.7 which at the time meant that it didn't like non UTF-8 characters for some reason that's too esoteric to get into. So one day some French guy or lady from an Eastern Block country came in and ordered a hot coffee with extra diacritical marks over the e. We don't know if it was her name or whether she put the note in the machine to add extra sugar to her köfé, but all of a sudden we have robot arms spewing milk everywhere, simple syrup squirting from orifices we didn't know the machine possessed, a total meltdown of our epic caffeine dispensing empire."}
                </p>
                <p>
                  {"Going fast and breaking things is a good philosophy for new ideas, and a terrible philosophy for banking. I'm not saying that the guys at JPMorgan are the smartest guys in the room. They're in banking so you already know that half of them flunked out of kindergarten. I'm saying that their company is 1) insured 2) been in business doing this and only this so it's unlikely they'll go down and take your money with it (fingers crossed) and 3) have written most of their business logic in COBOL and joinless SQL statements. I've never seen the programming for the machines that do interbank transfers between major bulge bracket institutions but I'd hazard a guess that they're written on stone tablets by the angry fist of the Financial Gods."}
                </p>
                <p>
                  {"Bank at a bank."}
                </p>
                <p>
                  {"The founder of Cash App was found stabbed to death last night in San Francisco."} <a href={"https://www.sfgate.com/bayarea/article/mill-valley-man-killed-sf-stabbing-17878809.php"}>{"Bob lee, stabbed to death."}</a>{" Holy shit that's fucking scary. Who does that? Gangsters killing each other is one thing, but this is messed up. I have the feeling that when they find out who did it, it won't be someone who was after his wallet, but a targeted killing because of his involvement in the payments industry. If they ever find the guy. Man, that's fucked up. The technology industry is approaching Murder On The Orient Express levels of WARBLGARBL - anyone in San Francisco that's been affected financially in any way in the last 20 years by the tech industry is a potential suspect. Maybe you guys should just stop being so shit. Fuck."}
                </p>
                <p>
                  {"Meanwhile I've thought of a way that I can automate sending pictures and text from my phone to the webite, but it'll require a backend. The federales are going to pay me $100 to continue to exist today so I can spend some of that on the required server. Once again, all of the code to write this FUCKIN RAD website is available upon request."}
                </p>
              </div>
            </div>

            <div>
              <div class="timeheader">
                {"Tues April 4 7:10PM San Francisco - The Time is Now"}
              </div>  
              <div>
                <p>
                  {"Perhaps the most important book that no one has read is Stand on Zanzibar - among other things it's where the three seashells in Demolilition Man comes from. I shit you not. It also predicts what a super computer is like to work under (ye old Shalmaneser) and what that means when your priors are completely and totally overtaken by new calculations. It predicted the iPhone (Ok, and In Watermellon Sugar). I read until I figured all the characters would die at the end and then dropped it. Most of the characters do a whole lot of complaining and agonizing over their collective trauma. Bitch monkeys in other words. Also, the obligatory super gay subplot - I mean it was the 70s what do you want."}
                </p>
                <p>
                  {"So when I saw that there was a BLM art section at the De Young Museum I was sort of - yeah alright. I figure, sure, let's all do this again. It's not that the art is *bad* per se. It's just dull. Yeah, black people have it rough, tell me another one. So's everybody. Last I checked 99.999% of the human population is pure asshole, so if you find someone who isn't an asshole no matter the color you should rejoice. Should we have a woe is me Ukrainian gallery, or a Jew gallery, or a Russian gallery? What was particularly amusing was the 20 foot tall statue of a black man on top of a horse because they're *still* mad about the Confederate statues in Richmond. Good God, do we still care? For what it's worth Salvador Dali's melting clocks is 18 in by 6 in and is bordered in drift wood. I saw it at the Moma in NY one time."}
                </p>
                <p>
                  {"The gallery was a bunch of black people in repose on flowers in a sort of Romeo and Juliet psychedelic style. It reminds me of the School of Fantastic Realism which is rather funny - I doubt that the artists would like to be compared unfavorably to Austrian painters. Course, these were all self portraits and the Austrians were capable of painting something other than themselves. Also, they weren't REALLY BIG. Although if that's a matter of quality, these paintings certainly had it."}
                </p>
                <p>
                  {"Here's the picture that made my heart sing today - because I understood it for the first time - "}
                </p>
                <img loading="lazy" src={"static/apr4/raspberries.jpg"} class="imagetag"/>
                <p>
                  {"Today was the day my EBT benefits came in and I could eat fresh fruit and food after a month of eating meals on wheels and handout food. What this is is some guy painting raspberries because the winter is coming and it'll be 6 months until he can taste them again. Or maybe he can only get ahold of raspberries for so long because they are an exotic food. I don't care if this is a true story - for me in some desolate plain of windblasted fields there is an artist painting food so he can remember what it looks like. So his eyes might taste it again when he becomes so hungry that's all he has. That to me is art. This BLM stuff is just - it's a battle cry from the victimhood brigade. It's become unfashionable to saddle up your hordes and go thundering into enemy territory so we must always and forever declare our wars ones of defense."}
                </p>
                <p>
                  {"Come hear the tears of our battle music for this is how you paint a scream. I've heard that one before kid, skip it. Your balls just drop or what?"}
                </p>
                <p>
                  {"In 2066 the star crossed lovers are the critics, always chasing their novelist ghost. And in SoZ the last of the mentats are those 'synthesists' that can aggregate disparate information and come up with the nonlinear approximation of truth, the whirling chaotic conspiracy that resolves itself into an stochastically random reality. These are warnings and parables of what not to be, but so much for false epiphanies. We are ALL synthesists - let me show you where I see things headed - "}
                </p>
                <p>
                  {"Check out - "}<a href={"https://www.economist.com/international/2023/04/03/was-your-degree-really-worth-it"}>{"Was Your Degree Really Worth It"}</a>{" by the bastion of free laise-faire liberalist the Economist. "}
                </p>
                <img loading="lazy" src={"static/apr4/economist.png"} class="imagetag"/>
                <p>
                  {"Here's the take away from the article. People have found that the value of a degree in the humanities has dropped and we can now use MACHINE LEARNING to determine this is true. Hence everyone is going into economics and mathematics in order to earn the big bucks. Hate to tell you guys, I majored in mathematics and I'm currently eating cat food, so that's no guarantee of anything. That's for starters. But the bigger problem is that engineering tells you how to optimize the course of the path that society finds itself in. You want to build FASTER computers? You need to be an engineer. You want to find a way to sell your next AI startup you need to be an MBA. Maybe, if degrees are a signalling device for the worth of the human spirit."}
                </p>
                <p>
                  {"But I bet that many of you engineers and mathematicians didn't know about the connection between a novel you've never heard of and a movie you've probably seen. You don't know how our current culture is currently wired and programmed to be set on the course it is. And so we are seeing, again, is what at first approximation seems like a good thing. We have a computer! And it's telling us how to make money! Unfortunately, what this means is that we're optimizing for a corner solution where we make it BETTER FASTER HARDER STRONGER for all of our children. And the people who go into the arts and humanities are those who have nothing to lose and those who have all the money in the world. We get Black Lives wallpaper of death and flowers and Elon Musk tweeting from on high."}
                </p>
                <p>
                  {"I mean fuck it, we could go all This Side of Paradise on HAAWWWVVAADDD until we have a generation of in-bred Fitzgeralds telling us how to do our tax returns and the ghost of Hemingway attempting to burn the motherfucker to the ground. Why not?"}
                </p>
                <p>
                  {"It's already obvious how AGI will be created and you just haven't read the right book yet. I hate to say it's all connected man, but it really is."}
                </p>
              </div>
            </div>

            <div>
              <div class="timeheader">
                {"Tues April 4 3:20PM San Francisco - The Time is Now"}
              </div>           
              <div>
                <p>
                  {"I went to the De Young museum of arts in Golden Gate park and thought I would make a background image for my site. Unfortunately it isn't coming together the way I wanted so it's a throwaway. Placing it here for inspiration later. I'll have some 'big thoughts' filled with 'wordiness' later on in the evening if I'm feeling up to it."}
                </p>
                <img loading="lazy" src={"static/apr4/pic.jpg"} class="imagetag"/>
              </div> 
            </div>

            <div>
              <div class="timeheader">
                {"Mon April 3 2:33PM San Francisco - The Time is Now"}
              </div>           
              <div>
                <p>
                  {"I've added a map with a couple of pictures from around the city. I plan on adding increased functionality over time (such as more information when you click on a dot, or perhaps a link to the blog post the image was first posted in). For the moment, all it is is red dots on the map you can click to see pictures by location. Progress!"}
                </p>
              </div> 
            </div>

            <div>
              <div class="timeheader">
                {"Sun April 2 11:28PM San Francisco - The Time is Now"}
              </div>
              <div>
                <p>
                  {"Now then. I was about to sit down and write a few words before I was rudely interupted by the psychotic homeless that smoked marijuana and drugged me again. They do this because they have nothing about themselves that is special and so they feel that they need to drag others down to their level by drugging them into submission. It's a form of torture - as if I'd be friends with torturers. Seriously? That's what they think would happen. The fuck kind of crack head logic is that. I know they're doing it on purpose, because they can go down to the alley and smoke drugs and they refuse to do so."}
                </p>
                <p>
                  {"They'll creep up on you too. I went to the library just to sit outside for 30 minutes before it opened and then they all started sitting closer and closer to me, smoking cigarettes and going AHEM AHEM when I was reading my book. Some derelict passed out at a bus stop started smoking a joint and then some fuckwit on a skateboard ran by yelling JESUS and then started schizo babbling on meth. It's like - yes, I notice they are there. I know they read this website for example. They're not stupid - they can find out my name and then creep on me on social media. And I've noticed how they respond to my posts."}
                </p>
                <p>
                  {"But you have to ask yourself - would you ever be friends with someone that drugged you? The fuck? I have - and this is absolutely true - TWICE invited homeless people into my home when they couldn't find a job or find a place. Once the guy found a place, and the other time the guy did not. They both annoyed the piss out of me, but I did it because I was bored and it was the right thing to do. I've given $50,000 to a charity for teaching kids to read in Detroit once - because I was depressed that rich assholes weren't willing to give me a job doing SOMETHING and so I figured I could at least make the world a better place by helping out those less fortunate than I."}
                </p>
                <p>
                  {"And here I am, in a homeless shelter, after being told to fuck off and die by my friends and family, being drugged every night by people that aren't willing to work on making something of themselves. It's fucking infuriating. I asked my sister if I could put up a tent in her backyard until I could find a job and my brother in-law gave me $300 to get lost. What a fucking putz."}
                </p>
                <p>
                  {"And this is not to say that the rich assholes in this two bit country haven't sown the whole thing up good and tight. I don't know if there's even a way to get a job without either knowing someone, being a kiss ass, or somehow be willing to steal. I did some construction work up in Seattle for a bit and all the electricians were high (while working on electric - fucking stupid man) and all the Mexicans were dealing coke. They didn't like me so they arranged that I work in an area with exposed nails until I tripped and shoved one through my palm. I drove my ass to the hospital and was back on the job the same day because I was sleeping in my car. But if someone doesn't like you badly enough they'll just grind you down until you quit in frustration, just to make it look like it's your fault."}
                </p>
                <p>
                  {"So here I am and the only things I have planned for tomorrow are eating from a food line (where I'll be drugged), reading at the library (which I was drugged outside of today), programming on my computer (which when I have my computer out the homeless psychopaths like to babble like the Sand People from Star Wars - while doing drugs), and oh I don't know. Jesus Fucking Christ the homeless in California are fucking vermin."}
                </p>
                <p>
                  {"I'm the kind of guy that says the loud part soft and the soft part loud. If someone's a prick - I'll say 'HEY LOOK AT THAT GUY - LOOK AT HOW MUCH OF A PRICK HE IS!' So, I don't tend to make m(any) friends. But I don't see many people who've done so little to harm others and done so much to be good to others, just on principle. Because if you need to believe in God to be a kind person you don't get it - at least for 90% of the people I see who ass their way to Church every week. Not the whole, I'm going to devote my life to being a goody two shoes, but just, I'm going to be a generally stand up guy and not steal, assault, commit adultery or go out of my way to be a prick - and occasionally be good to a couple people. And I'm being royally ass fucked by vultures that are making me high."}
                </p>
                <p>
                  {"So what I'm going to do tomorrow is take a tourist map of the city of San Francisco and start making an interactive map of where all the homeless encampments are. Because they won't stop smoking drugs around me, so now it's my problem. Which makes it everyone's problem. Once people know where the encampments are, we can start cleaning them up and putting these people in shelters, rehab, or prison."}
                </p>
                <p>
                  {"I DON'T LIKE BEING DRUGGED."}
                </p>
              </div>              
            </div>


            <div>
              <div class="timeheader">
                {"Sun April 2 11:03PM San Francisco - The Time is Now"}
              </div>
              <div>
                <p>
                  {"They're burning marijuana again at Next Door Shelter (1001 Polk Street, San Francisco California). I am being drugged against my will by psychotic homeless that are attempting to force me into having a psychiatric episode. Next Door Homeless Shelter needs to be shut down by the health department and all of the people here put in rehab or prison. Do not give money to the homeless - they will use it on drugs and then use those drugs to harm other people. Give money to the police - the homeless in the United States are not stable or self respecting enough to not harm others."}
                </p>
              </div>              
            </div>

            <div>
              <div class="timeheader">
                {"Sun April 2 5:01PM San Francisco - The Time is Now"}
              </div>
              <p>
                {"So, the markets open - as they often do - on Monday morning. There's been a great deal of consternation over the current state of the banking industry - as if this current crisis were any sort of surprise. It's always when the *stock market* turns down that everyone starts paying attention to how shit the economy is. Gee I wonder why that is. Take a look at this - "}
              </p>
              <img loading="lazy" src={"static/apr2/deposits.png"} class="imagetag"/>
              <img loading="lazy" src={"static/apr2/earnings.png"} class="imagetag"/>
              <img loading="lazy" src={"static/apr2/houses.png"} class="imagetag"/>
              <img loading="lazy" src={"static/apr2/rent.png"} class="imagetag"/>
              <p>
                {"Once upon a time I was copy bitch in the Emerging Markets section of the Federal Reserve which makes me eminently unqualified to discuss the state of the national economy, but at this point even a coke monkey with down syndrome could see that excrement has well and truly intersected the rotary ventilator. Just ask my bunk mates over at the SF crackhouse I'm an inmate of. So. You'll notice that all of the charts are trending upward. But between 2000 and today (let's say over roughly 20 years) -  "}
              </p>
              <ul>
                <li>
                  {"US deposits went up 250%"}
                </li>
                <li>
                  {"Salary by people with a bachelor's degree or higher went up 60%"}
                </li>
                <li>
                  {"Average sale price of homes went up 200% - 250% if we count the last two years."}
                </li>
                <li>
                  {"Rent went up 250%"}
                </li>
              </ul>
              <p>
                {"What this tells me is that salaries are the outlier. For college educated people. What we have is a bunch of old fucks that have managed to retire into wealth, and you know what old people don't do? Spend. They don't spend, they go to early bird specials and save every penny so they can pay for their healthcare expenses. And it means that the people who've managed to accumulate wealth by owning things can now afford to buy more investment properties and reinforce the cycle. If you own a bunch of properties which are appreciating and don't have to worry about food, you can afford to buy more properties and not say, spend all your money on "} <a href={"https://www.npr.org/2022/07/02/1109105779/monthly-car-payments-record-700"}>{"car payments"}</a>{" and such. The upper class only have to compete with the salaried class - and they can outbid them on everything from electronics to car payments to ye old no fat whip cinnamon fucking lattes. And this is over 20 years! Suppose you were to knock up your girl today and have a kid. Well you're completely fucked then, because there's no way this shit show can continue. If you have a kid under 10 you're still screwed because the economy is not going to be this good in ten years. Not by a long shot."}
              </p>
              <p>
                {"Why? Because a stable healthy economy has 300 million consumers buying cheeseburgers and flat screen tvs and fuck I don't know - the fad of the minute. Having an increasingly shrinking wealthier class of people plays hell with the velocity of money - Richy Rich can only eat so many McRibs before he explodes."}
              </p>
              <p>
                {"As far as I can tell this rot is worldwide - fuck look at "}<a href={"https://www.bloomberg.com/news/features/2023-03-27/swedish-housing-market-crash-exposes-economic-divisions#xj4y7vzkg"}>{"Sweden"}</a>{"."}
              </p>
              <p>
                {"Something to consider."}
              </p>
              <p>
                {"Course, I'm unemployed and homeless, so what do I know?"}
              </p>
            </div>

            
            <div>
              <div class="timeheader">
                {"Sun April 2 1:11PM San Francisco - The Time is Now"}
              </div>
              <p>
                {"I've added a modal that tells you how to pay me for my writing. In essence I've just told you to send me money to my paypal account and then write what you want me to talk about. I'm not adding in an automated paypal dingus because I don't want to screw it up. I've also added a skull. Personally I think it looks fuckin sick but what do I know. Write me!"}
              </p>
            </div>

            <div>
              <div class="timeheader">
                {"Sat April 1 1:05PM San Francisco - The Time is Now"}
              </div>
              <p>
                  {"A quiet Saturday to read my book. I've managed some graphical updates to the site, but am feeling uninspired. More will come, as it always does."}
              </p>
            </div>

            <div>
              <div class="timeheader">
                {"Fri Mar 31 7:40PM San Francisco - The Time is Now"}
              </div>
              <p>
                  {"So, Trump is being indicted on Tuesday and that's all the press is talking about. Now, I have no dog in this fight. Do you? Do you give a shit that an ex-president fucked a whore at some point? Or something - I have no idea. I never paid attention to this controversy in the first place. But you have to ask yourself  - 'Why Now?' Remember when we were just talkine about a banking crisis that was going to sink all collective deposits worldwide and then all of a sudden this is all the newspapers are talking about? I'm not saying this is a conspiracy, but I've never seen a more convenient media shit show since Desert Fox. Sleep tight - don't worry about where your money is. I have no money so I don't have anything to worry about. No kids or a mortgage either. CNN you gotta love 'em."}
              </p>
              <img loading="lazy" src={"static/mar31/fuckwits.png"} class="imagetag"/>
              <p>
                  {"This piece I'm entitling 'fuckwits - just fucking fuckwits'. Life as performance art."}
              </p>
              <p>
                  {"Here's the plan. I'll become so hated and reviled all over the internet that I'll be banned everywhere except on this server and then become so absurdly popular that people will pay me millions of dollars to read my writing and this will be the only place on the internet where they'll be able to find it. The plan is coming along nicely. Full steam ahead boys!"}
              </p>
            </div>

            <div>
              <div class="timeheader">
                {"Fri Mar 31 9:15PM San Francisco - The Time is Now"}
              </div>    
              <p>
                {"We are building a "}<a href={"https://www.youtube.com/watch?v=e2CliA8PuRM&ab_channel=BronxRadioLab"}>{"religion"}</a>{" and the website is coming along. While there are few truisms in life, one of the few things that I've learned is that you'll never have as cool a website "}<a href={"https://www.lingscars.com/"}>{"as this asshole"}</a>{". I've added a links section and an about section and I'm going to start pushing this github page for all she's worth."}
              </p>

              <p>
                {"Meanwhile, this "}<a href={"https://news.ycombinator.com"}>{"news aggregation website"}</a>{" has royally pissed me off. You know what these cunts are doing? They're shadow banning users and not telling them that they're doing it. Who the fuck does that? What is this, the Mickey Mouse show?"}
              </p>

              <img loading="lazy" src={"static/mar31/pic1.jpg"} class="imagetag"/>

              <p>
                {"The reason that they're doing this is fairly obvious. They have no other way of keeping out the trolls. The website is free, and so the only way to keep people from creating another account and spamming the board with bullshit is to ban them and not tell them they're banned. So, they can't monkey code a solution so they act like little bitches. I mean, fair play, their website. But aren't they supposed to be good at this? The fuck? Baffles the mind it does."}
              </p>
            
              <img loading="lazy" src={"static/mar31/code1.png"} class="imagetag"/>

              <p>
                {"Anyway, it also means that hackerNews website is filled with spammy trash. LIKE THIS POLITICAL PARTY. WHITE PEOPLE AI DRIVES LIKE THIS AND BLACK PEOPLE AI DRIVES LIKE THIS. Smegma. Total smegma. So here's 25 lines of code that uses chatGPT to show you the most relevant articles based on their titles, so you don't have to bother reading the site. We are the music makers and we are automating the automaters. I'll leave it as an exercise for the reader to add user input prompts. Integrate it into your site! Tell your friends!"}
              </p>

              <p>
                {"I like the concept of automating the automaters. It works in so many ways. Take "}<a href={"https://ohadravid.github.io/posts/2023-03-rusty-python/"}>{"this guy"}</a>{" for example. What he's doing is he's taking a bunch of python code, looking for the places where that code is inefficient and then replacing the inefficient bits with Rust. Because Rust is a low level language it makes the code faster. What they do is that they look at something called "}<a href={"https://www.polarsignals.com/blog/posts/2023/03/28/how-to-read-icicle-and-flame-graphs/"}>{"flame charts"}</a>{". This is just a fancy visualization that will look at the dependencies of what code calls what code calls what code...to find the bit in the nth nested dependency that is slowing down the program. It looks like this - "}
              </p>

              <img loading="lazy" src={"static/mar31/flame.png"} class="imagetag"/>

              <p>
                {"But think about this for a minute. This is a two step process. First, you look at a graph, then you find the code that's inefficient and replace it with more efficient code in another language. Can't LLM models already do that? This looks like an entire cottage industry that are getting paid the big bucks to do what someone could program once and be done with it. Also they're mapping polygons to vectors. In my heart they're curing cancer, but in reality it's probably another video game where you can "}<a href={"https://www.gamespot.com/articles/saints-row-iv-developers-respond-to-dildo-weapon-criticisms/1100-6412418/"}>{"beat someone to death with a dildo"}</a>{". To each their own I suppose."}
              </p>

              <p>
                {"I hate to say it, but the one thing that an LLM model can't do is live authentically. It can't create the alpha of lived experience. I've written about this before - I won't post the link because it's a long and boring article that no one who've I've ever shown has bothered to read or could contribute. What LLM models can do is they can create other people's work and they can create like works. You want a picture that's 'Man Getting Hit In The Balls (but like Picasso)?' Bam! Done! Unfortunately, soon the internet will be filled (if it isn't already) with Picasso Balls and LLM models will start eating their own shit. That is they won't have any more training data to train on, because they've fax machine copied the internet to death. Unless the AI companies start downloading driverless car traffic sensor data - I wouldn't past the dogfuckers. Point being, the people who will rule the future will be the experientialists, those who can create new alpha and know how to live. Unfortunately for all of us poor bastards that see the guys that had the cushy button pushing jobs and thought - damn, I wish I had that. Instead we're all going to have to do those jobs that machines can't (the shit hard jobs like security guard, grocery store worker, garbageman) until those jobs are automate away. The people who will make money will be a shrinking minority of poets and artists that can contribute the alpha to keep the future going. I mean, where the fuck is the money going to come from?"}              
              </p>

              <p>
                <a href={"https://www.vanityfair.com/news/2017/06/neal-stephenson-metaverse-snow-crash-silicon-valley-virtual-reality"}>{"In the future we are all gargoyles"}</a>
              </p>

            </div>

            <div>
              <div class="timeheader">
                {"Thurs Mar 30 2:27PM San Francisco - The Time is Now"}
              </div>            
              <p>
                {"THIS JUST IN - GOOGLE BING CHAT THINKS GAY PEOPLE ARE OBNOXIOUS*. *either that or rainbows."}
              </p>
              <img loading="lazy" src={"static/mar30/pic11.png"} class="imagetag"/>
            </div>

            <div>
              <div class="timeheader">
                {"Thurs Mar 30 11:20AM San Francisco - The Time is Now"}
              </div>
              <p>
                {"So the dog fuckers in the shelter started speaking bullshit glossolalia about drugs and bullshit for an hour and then tried to light up meth. I told them to go fuck themselves and then they said 'OH MASSAH ME SO SAWWEE. I APPRECIATE YOU.' Shitbricks. So I decided to go around the ghetto this morning taking pictures of all the crazy shit that pisses me off and no one is fixing. There's a failure of the imagination in a writer who makes himself the story, but I don't care. So I'm not imaginative. I'm not that "}<a href={"https://www.wired.com/story/brandon-sanderson-is-your-god/"}>{"dipshit with a sword"}</a>{". I just have a keyboard a camera and a pair of brass balls. Let me show you something that will make you angry. Something that might make someone give a shit for 5 minutes."}
              </p>
              <img loading="lazy" src={"static/mar30/pic1.jpg"} class="imagetag"/>
              <p>
                {"I title this piece "}<a href={"https://www.youtube.com/watch?v=5VtDM5jicRQ"}>{"today was a good day"}</a>{"."}
              </p>
              <img loading="lazy" src={"static/mar30/pic2.png"} class="imagetag"/>
              <p>
                {"Yeah - you see that tent in front of the 'Hot Zone' dildo bazaar? That fuckwit likes to smoke meth when people walk by - and because the owners won't clean the sidewalk nothing is done about it. The cops can't do anything about moving this person from the premises. "}<a href={"https://sfgov.org/mod/sites/default/files/FileCenter/Documents/2176-Sidewalk%20Landscape%20with%20SIRP.pdf"}>{"According to Article 15, Section 706 of
                the Public Works Code"}</a>{" it's the responsibility of the owner of a premises to clean their own sidewalk. So that means that people can't walk on this side of the street or this shitstain will smoke meth and dose them. In a sane world, this person needs to be in a federal penitentiary for the rest of their natural lives for hundreds of cases of assault with a deadly weapon. But yeah, you're right CNN - it's all the fault of "}<a href={"https://www.youtube.com/watch?v=9nWAic0lHVI&embeds_euri=https%3A%2F%2Fwww.bing.com%2F&embeds_origin=https%3A%2F%2Fwww.bing.com&feature=emb_logo&ab_channel=CNN"}>{"this guy"}</a>{". I'd call you fuckwits, but 'journalist' is a dirtier word around here."}
              </p>
              <img loading="lazy" src={"static/mar30/pic3.png"} class="imagetag"/>
              <p>
                {"People dealing crack in front of a market where the owner has either been dosed into submission, or is getting a kickback because the dealers slip him money. Not safe to walk past here."}
              </p>
              <img loading="lazy" src={"static/mar30/pic4.png"} class="imagetag"/>
              <p>
                {"This is what it looks like when the cartels decide to take over a sidewalk."}
              </p>
              <img loading="lazy" src={"static/mar30/pic5.png"} class="imagetag"/>
              <p>
                {"Dealers on the corner."}
              </p>
              <img loading="lazy" src={"static/mar30/pic6.png"} class="imagetag"/>
              <p>
                {"Well shit on me! I didn't know there was an a capella cover of "}<a href={"https://www.youtube.com/watch?v=3L4YrGaR8E4&ab_channel=RATMVEVO"}>{"Bulls on Parade"}</a>{". Shit, give me 800 million dollars and I can sit on my ass all day doing nothing too!"}
              </p>
              <img loading="lazy" src={"static/mar30/pic7.png"} class="imagetag"/>
              <p>
                <a href={"https://wilwheaton.net/2023/03/the-library-is-a-safe-place/"}>{"That guy from Star Trek"}</a>{" tells me that the library is a safe place. That's why there's suicide netting around the balconies. Don't worry - they're replacing it with plexiglass soon, so it'll be more aesthetically pleasing. In San Francisco, the only thing that's illegal is killing yourself in a way that's too sudden and messy. Too gauche. Too outré. We prefer a long slow slide into oblivion around here."}
              </p>
              <img loading="lazy" src={"static/mar30/pic8.png"} class="imagetag"/>
              <p>
                {"They put up speakers in the capital section of the Tenderloin to play shit music to drive the homeless away from the area. Because policing is harder than putting up speakers that will simply drive someone mad who wants to lie down to rest. Good fucking job. Or maybe the people here like ABBA and Ace of Base on repeat. Who fucking knows."}
              </p>
              <img loading="lazy" src={"static/mar30/pic9.png"} class="imagetag"/>
              <p>
                {"The landlords near market street with the classical looking buildings can afford to pay some asshole to clean the sidewalks."}
              </p>
              <img loading="lazy" src={"static/mar30/pic10.png"} class="imagetag"/>
              <p>
                {"OH LOOK IT'S A TROLLEY. We used to ride those to work."}
              </p>
              <p>
                {"This is Peter Teller Weyand telling the lot of you that you have no fucking idea what you're doing."}
              </p>
            </div>

            <div>
              <div class="timeheader">
                {"Wed Mar 29 7:03PM San Francisco - The Time is Now"}
              </div>
              <p>
                {"So the crazy people in the shelter are attempting to force me into religious conversion. What this person will do is they'll talk to themselves repeatedly while they play music and flood the shelter with drugs. I started recording after this dogfucker just repeated to himself 'you can be reborn. REBORN!'"}
              </p>
                {"I don't want to lay down in the gutter and shoot up with heroin so I must be a born again Christian. I don't know either - crack head logic, what do you want."}
              <p>
                {"Then he got mad when he saw that I had my computer out and started muttering about how he had been 'killed a thousand times' and some other crazy shit. This is what forced religious conversion looks like, by drugging someone repeatedly over and over again. This is at Next Door Shelter, 1001 Polk Street. I am being drugged against my will by people that want me to convert to their crazy bullshit religion."}
              </p>
              <p>
                {"Now with audio!"}
              </p>
              <audio controls=true>
                <source src={"static/crazypeople2.m4a"} type="audio/mpeg" />
                { "Your browser does not support the audio element." }
              </audio>
            </div>
            
            <div>
              <div class="timeheader">
                {"Wed Mar 29 6:31PM San Francisco - The Time is Now"}
              </div>
              <p>
                {"Speaking of horse fuckers - just remember "}<a href={"https://www.youtube.com/watch?v=7Pq-S557XQU&ab_channel=CGPGrey"}>{"Humans Need Not Apply"}</a>{" was written in 2015. It's not like we haven't seen the AI revolution writing on the wall for eight years and no one did a damn thing to make it so people could find stable long term employment that would pay the rent, let you put your kids through school, let alone have enough to eat or not be scared of your retirement plans being 'busking in the street' or 'winning the lottery'. You're welcome."}
              </p>
            </div>

            <div>
              <div class="timeheader">
                {"Wed Mar 29 5:39PM San Francisco - The Time is Now"}
              </div>
              <p>
                {"I've added a picture that I put into Adobe Firefly - I put in the prompt 'The sky over the port was the color of a dead television'. That prompt has been put in so many times that it only came back with this kind of thing. Proving that once again, people have no fucking imagination although it is 'pretty'. If Case could see us now he'd cut his own dick off with a rust pair of pliers. I'll continue to add more to this website and call everyone on the planet a horsefucker."}
              </p>
            </div>

            <div>
              <div class="timeheader">
                {"Wed Mar 29 5:20PM San Francisco - The Time is Now"}
              </div>
              <p>
                {"I have no money and I'm a homeless bum. I fucking hate the homeless, HOLY FUCKING SHIT DO I HATE THOSE FUCKERS. I'm going to spend the rest of my time developing this website. It's written in yew.rs, which means that I can be horribly inefficient. The downside is that I don't have the money to host a backend so I can't do much on the front. FUCK. In any case I'm going to make this the most rad kickass site I can given that I can't store anything in a database. Fuck you."}
              </p>
            </div>

            <div style="height: 10vh; background: rgba(0,0,0,0); color: white; opacity: 0;">
              {"something"}
            </div>
          </div>
        </div>
      </>
    }
  }
}

impl Home{
}
