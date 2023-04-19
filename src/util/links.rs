use regex::Regex;


#[derive(Clone)]
pub struct Link{
  pub category: &'static str, 
  pub tagline: &'static str, 
  pub href: &'static str, 
  pub notes: String
}

pub fn links() -> Vec<Link> {
  let mut returnlinks = vec![];
  returnlinks.push(
    Link{
      category: r###"Music"###,
      tagline: r###"Building A Religion"###,
      href: r###"https://www.youtube.com/watch?v=e2CliA8PuRM&ab_channel=BronxRadioLab"###,
      notes: r###"More true than we'd all like to admit"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Funny"###,
      tagline: r###"Ling's Cars"###,
      href: r###"https://www.lingscars.com/"###,
      notes: r###"Lings cars - you will never make a website this cool."###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"News"###,
      tagline: r###"YCombinator"###,
      href: r###"https://ohadravid.github.io/posts/2023-03-rusty-python/"###,
      notes: r###"YCombinator - the most fake Fake News on the Internet"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Blog"###,
      tagline: r###"Some Programmer Dude"###,
      href: r###"https://news.ycombinator.com"###,
      notes: r###"Optimizing Python With Rust"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Article"###,
      tagline: r###"Neal Stephenson what a guy"###,
      href: r###"https://www.vanityfair.com/news/2017/06/neal-stephenson-metaverse-snow-crash-silicon-valley-virtual-reality"###,
      notes: r###"Talking bullshit with Neal Stephenson"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Article"###,
      tagline: r###"Beat Someone To Death With A Dildo"###,
      href: r###"https://www.gamespot.com/articles/saints-row-iv-developers-respond-to-dildo-weapon-criticisms/1100-6412418/"###,
      notes: r###"Saints Row IV developers respond to dildo weapon criticisms  - the title raises more questions than it answers"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Blog"###,
      tagline: r###"Flame and Icicle Graphs"###,
      href: r###"https://www.polarsignals.com/blog/posts/2023/03/28/how-to-read-icicle-and-flame-graphs/"###,
      notes: r###"How flaming graphs can make you a better programmer"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Article"###,
      tagline: r###"Dipshit With A Sword"###,
      href: r###"https://www.wired.com/story/brandon-sanderson-is-your-god/"###,
      notes: r###"Living proof you can put words on paper and make a living without being capable of being a writer - an inspiration to us all"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Music"###,
      tagline: r###"Today Was a Good Day"###,
      href: r###"https://www.youtube.com/watch?v=5VtDM5jicRQ"###,
      notes: r###"Coolio keeping it copacetic for all us sinners out there"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Law"###,
      tagline: r###"San Francisco Legally Incompetent"###,
      href: r###"https://sfgov.org/mod/sites/default/files/FileCenter/Documents/2176-Sidewalk%20Landscape%20with%20SIRP.pdf"###,
      notes: r###"San Francisco's incompetence is written into the tax code"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Video"###,
      tagline: r###"Hose guy"###,
      href: r###"https://www.youtube.com/watch?v=9nWAic0lHVI&embeds_euri=https%3A%2F%2Fwww.bing.com%2F&embeds_origin=https%3A%2F%2Fwww.bing.com&feature=emb_logo&ab_channel=CNN"###,
      notes: r###"The guy spraying the homeless with a hose because he didn't know what the fuck else to do"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Music"###,
      tagline: r###"Bulls on Parade"###,
      href: r###"https://www.youtube.com/watch?v=3L4YrGaR8E4&ab_channel=RATMVEVO"###,
      notes: r###"Bulls on Parade - or as I like to call it 'Classical'"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Blog"###,
      tagline: r###"That Guy from Star Trek"###,
      href: r###"https://wilwheaton.net/2023/03/the-library-is-a-safe-place/"###,
      notes: r###"Bulls on Parade - or as I like to call it 'Classical'"###.to_string()
    }
  );
  returnlinks.push(
    Link{
      category: r###"Blog"###,
      tagline: r###"Humans Need Not Apply"###,
      href: r###"https://www.youtube.com/watch?v=7Pq-S557XQU&ab_channel=CGPGrey"###,
      notes: r###"The one guy that saw it all coming"###.to_string()
    }
  );

  return returnlinks;
}