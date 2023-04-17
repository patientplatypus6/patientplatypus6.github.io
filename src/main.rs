use yew::prelude::*;
use yew_router::prelude::*;
use yew::functional::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::router::BrowserRouter;

use yew_page::*;
use yew_page::pages::home::Home;
use yew_page::pages::links::Links;
use yew_page::pages::about::About;
use yew_page::pages::map::Map;
use yew_page::pages::wall::Wall;

use yew_page::pages::books::academic::Academic;
use yew_page::pages::books::classical::Classical;
use yew_page::pages::books::historybooks::Historybooks;
use yew_page::pages::books::literature::Literature;
use yew_page::pages::books::philosophy::Philosophy;
use yew_page::pages::books::politics::Politics;
use yew_page::pages::books::religion::Religion;
use yew_page::pages::books::horror::Horror;
use yew_page::pages::books::scififantasy::Scififantasy;


use yew_page::routes::routes::Route;

use gloo_net::http::Request;
use yew::{classes};
use std::collections::HashMap;

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <div> 
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { 
                < Home month={"".to_string()}/> 
            }
        },
        Route::HomeMonth {month} => {
            html! { 
                < Home month={month}/> 
            }
        },
        Route::Links => html! { < Links /> },
        Route::About => html! { < About /> },
        Route::Map => html! { < Map /> },
        Route::Academic => html! { < Academic /> },
        Route::Classical => html! { < Classical /> },
        Route::Historybooks => html! { < Historybooks /> },
        Route::Literature => html! { < Literature /> },
        Route::Philosophy => html! { < Philosophy /> },
        Route::Politics => html! { < Politics /> },
        Route::Religion => html! { < Religion /> },
        Route::Horror => html! { < Horror /> },
        Route::Scififantasy => html! { < Scififantasy /> },
        Route::Wall => html! { < Wall /> }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}