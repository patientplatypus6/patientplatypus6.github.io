
#[derive(Clone, Copy)]
pub struct Blog{
  pub time: &'static str, 
  pub date: &'static str, 
  pub location: &'static str, 
  pub title: &'static str, 
  pub paragraphs: &'static str
}

pub fn find_index_by_time_date(blog_vec: &Vec<Blog>, time_to_find: &str, date_to_find: &str) -> Option<usize> {
  for (i, blog) in blog_vec.iter().enumerate() {
      if blog.time == time_to_find && blog.date == date_to_find {
          return Some(i);
      }
  }
  None
}

pub fn blogs() -> Vec<Blog> {
  let mut returnblogs = vec![];
  
  returnblogs.push(
    Blog{
      date: r###"<span>Tue Apr 18</span>"###,
      time: r###"<span>1:31PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>More Work</span>"###,
      paragraphs: r###"
        <p>I'm working on a new blogging system. Or rather saving the data in a new way.</p>
        <p>This is a test picture of a kitten.</p>
        <img loading="lazy" src="static/apr18/kitten.png" class="imagetag"/>
        <p>
          There won't be much in the way of writing today. I found that in order to parse the blogs (for the search feature, and if I wanted to do anything else with them for that matter) I would have to put all the blogs in a way which, well, I could parse them. I thought about scraping the website, but that doesn't work because of how WASM loads the page with an event listener. Then I thought I would read the file and just parse the file in memory of the blogs, but that doesn't work, because yew on the frontend doesn't like the file reader API for reasons. So at the moment, I'm converting all the blog posts to Blog structs. Not the most interesting thing in the world, but on the other hand, incremental progress! So, that's it for updates today.
        </p>
      "###
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Mon Apr 17</span>"###,
      time: r###"<span>6:19PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Updates to the Website</span>"###,
      paragraphs: r###"
        <p>
          Early morning - I'm planning on some updates to the website today, we'll see how far I get.
        </p>
        <p>
          Ok, I componetized a bunch of stuff which doesn't show much on the blog UI (the navlinks and the header), and cleaned up how the Home page blogs are rendered. The biggest change was that I managed to put in a sidebar for accessing the blog posts by month. There's a search bar which currently doesn't do anything, but that will have to wait until tomorrow.
        </p>
      "###
    }
  );

  returnblogs
}