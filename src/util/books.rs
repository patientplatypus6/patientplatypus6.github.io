use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct BookIDS{
  pub bookid: String,
  pub chartdesc: String, 
  pub kind: String
}

pub fn bookidshandler() -> Vec<BookIDS>{
  let mut bookidsvec = vec![];
  bookidsvec.push(
    BookIDS{
      bookid: "1HfdwocDxllAkBCQLFHQhyD5sj1ywIeRy".to_string(),
      chartdesc: "Academic Manga Guides".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec
}