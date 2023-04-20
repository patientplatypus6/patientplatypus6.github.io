use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/xml")]
    Xml,
    #[at("/:month")]
    HomeMonth{ month: String },
    #[at("/:month/:day/:time")]
    Blogpage{ month: String, day: String, time: String },
    #[at("/links")]
    Links, 
    #[at("/about")]
    About,
    #[at("/map")]
    Map, 
    #[at("/wall")]
    Wall, 
    #[at("/books/academic")]
    Academic, 
    #[at("/books/classical")]
    Classical, 
    #[at("/books/history")]
    Historybooks, 
    #[at("/books/literature")]
    Literature, 
    #[at("/books/philosophy")]
    Philosophy, 
    #[at("/books/politics")]
    Politics, 
    #[at("/books/religion")]
    Religion, 
    #[at("/books/scififantasy")]
    Scififantasy, 
    #[at("/books/horror")]
    Horror  
}