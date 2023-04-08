use yew_router::prelude::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/links")]
    Links, 
    #[at("/about")]
    About,
    #[at("/map")]
    Map, 
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