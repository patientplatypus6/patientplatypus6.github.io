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
    #[at("/academic_books")]
    Academic, 
    #[at("/classical_books")]
    Classical, 
    #[at("/history_books")]
    Historybooks, 
    #[at("/literature_books")]
    Literature, 
    #[at("/philosophy_books")]
    Philosophy, 
    #[at("/religion_books")]
    Religion, 
    #[at("/scififantasy_books")]
    Scififantasy  
}