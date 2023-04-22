
use regex::Regex;


#[derive(Clone)]
pub struct Blog{
  pub time: &'static str, 
  pub date: &'static str, 
  pub location: &'static str, 
  pub title: &'static str, 
  pub route: &'static str,
  pub paragraphs: String
}

pub fn find_substring(blog_vec: &Vec<Blog>, substring: &str) -> Vec<Blog>{
  let mut returnblogvec = vec![];
  for (i, blog) in blog_vec.iter().enumerate() {
    let substringindex = blog.paragraphs.clone().find(substring);
    if substringindex != None {
      let editblog = blog.clone();
      let substringindexval = substringindex.unwrap();
      log::info!("###########################################");
      log::info!("what's the value of the substring: {:?}", substring.clone());
      log::info!("###########################################");
      let mut replacestring = r##"<span style="background: purple; color: white;">"##.to_string()+substring+r##"</span>"##;
      let newparagraphs = editblog.paragraphs.clone().to_string().replace(substring, &replacestring);
      log::info!("value of newparagraphs_str: {:?}", newparagraphs.clone());

      let blogstringparagraph = blog.clone().paragraphs.to_string();
      let nminus1 = blogstringparagraph.chars().nth((substringindexval-1)).unwrap().to_string();
      let nplus1 = blogstringparagraph.chars().nth((substringindexval+substring.len())).unwrap().to_string();

      log::info!("value of nminus1: {:?}", nminus1.clone());
      log::info!("value of nplus1: {:?}", nplus1.clone());
      log::info!("value of substringindexval: {:?}", substringindexval.clone());

      //this is currently janky 
      //the correct way to fix this is to remove the links and image tags, save them to an array
      //and then put them back at the appropriate index after highlighting the text
      //fix this later

      if substringindexval == 0 ||
      nminus1.clone() == " ".to_string() || 
      (nminus1.clone() == " ".to_string() && nplus1.clone() == " ".to_string()) ||
      (nminus1.clone() == ".".to_string() && nplus1.clone() == " ".to_string()) ||
      (nminus1.clone() == " ".to_string() && nplus1.clone() == ".".to_string()){
        returnblogvec.push(
          Blog{
            time: editblog.time.clone(),
            date: editblog.date.clone(),
            location: editblog.location.clone(),
            title: editblog.title.clone(),
            route: editblog.route.clone(),
            paragraphs: newparagraphs.clone()
          }
        )
      }
    }
  }
  return returnblogvec;
}

pub fn return_xml(blog_vec: &Vec<Blog>) -> String{
  let mut xml = r###"<xmp><?xml version="1.0" encoding="UTF-8" ?>
  <rss version="2.0">
  <channel>"###.to_string();

  for (i, blog) in blog_vec.iter().enumerate() {
    let appendstring = r###"<item>
    <title>"###.to_string() + blog.title.clone() + r###"</title>
    <link>http://patientplatypus6.github.io"### + blog.route.clone() + r###"</link>
    <description> What it says on the tin </description>
    </item>"###;
    xml = xml + appendstring.as_str();
  }

  xml = xml + r###"</channel></rss></xmp>"###;
  xml
}

pub fn find_index_by_time_date(blog_vec: &Vec<Blog>, time_to_find: &str, date_to_find: &str) -> Option<usize> {
  for (i, blog) in blog_vec.iter().enumerate() {
      if blog.time == time_to_find && blog.date == date_to_find {
          return Some(i);
      }
  }
  None
}

pub fn find_blog_by_route(blog_vec: &Vec<Blog>, route: &str) -> Blog {
  for (i, blog) in blog_vec.iter().enumerate() {
    if blog.route == route {
      return blog.clone()
    }
  }
  Blog{
    date: r###"ERROR"###,
    time: r###"ERROR"###,
    location: r###"ERROR"###,
    title: r###"ERROR"###,
    route: r###"ERROR"###,
    paragraphs: r###"ERROR"###.to_string(),
  }
}

pub fn blogs() -> Vec<Blog> {
  let mut returnblogs = vec![];

  returnblogs.push(
    Blog{
      date: r###"<span>Fri Apr 21</span>"###,
      time: r###"<span>5:52PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Clown Show</span>"###,
      route: r###"/april/21/552pm"###,
      paragraphs: r###"
        <p>
          I went to 123 10th street and offered to build them a python app for free so they could check if people qualified for housing before putting them in a system that rejects them for six months. As I suspected they turned me down, acted nervous, and started referring me to a bunch of black boxes to stick my opinions in. Cocksuckers are just "rejecting" people if they don't get a kickback. It's a bureacratic make work scheme and a means of generating graft. Sell drugs for six months and if you save enough money we'll give you an SRO! What moral midgets. What ignoramuses. What a fucking clown show.
        </p>
      "###.to_string()
    }
  );


  returnblogs.push(
    Blog{
      date: r###"<span>Fri Apr 21</span>"###,
      time: r###"<span>10:33AM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Deep Thoughts</span>"###,
      route: r###"/april/21/1033am"###,
      paragraphs: r###"
        <p>
          I've been thinking lately, a dangerous preoccupation in some quarters, most especially among those who are out hustling for a little of that hope and change. I've written a bit of a LinkedIn screed - that and a handjob'll get you a bottle of gutrot. Here's a picture. I'll not bother to type it out as it wouldn't be worth the effort and I don't want it to be indexable. Not because I'm wrong, but it's a waste of my sword, fighting someone else's battle when they won't even fight for themselves.
        </p>
        <p>
          Here's a picture for those visual sorts that can walk down the street without sunglasses but are still too stupid to read. Right up there with people who can read Kierkegaard but who's personality is still emblazoned on a t-shirt.
        </p>
        <img loading="lazy" src="static/apr21/pic1.png" class="imagetag" />
        <p>
          On the technical front I've sorted out the links and made them indexable with a sort mechanism. The links table still needs to be made responsive, but it's good enough for the moment, so I put a note in the code and I'm moving on.
        </p>
        <p>
          I'm thinking that I might want to make a comment wall using Google sheets as a backend, but I don't have enough readers yet to make the ability to write SUCK A DICK in Calibri 18 bolded and underlined worthwhile yet. As far as doing anything that would require me to log in - meh. I could set up a system where I could write a blog post from some other computer if I logged into an admin panel, but for the moment I don't have a need. Other things that I could do include adding a world map and then add geo location dots, but there's a limit of 100 people per month on freegeoip.net. Either I have less people than that visit my site or I have to pay more money than it's worth. Does anyone care about the people coming to my site or not. 
        </p>
        <p>
          NOTE TO SELF - purchase brick to brain the crazy old Chinese woman going Ahem Ahem Ahem in the library. How much does a brick cost? Hmm...now I'm thinking of brick costs. Damn crazy old Chinese bats.
        </p>
        <p>
          Some children have thrown a protest again today. I believe they are, in fact, children this time and not full grown adults. Lots of pretty banners and nothing to do to fix any problems. About the same as always. Their parents were all high yesterday, so tree meet apple. 
        </p>
        <p>
          Seeing as I have no idea what to write about that people would care enough to read, or write down so people I care about would want to read it, I'll spend the rest of the day reading. It's fascinating, people only spend the time to read if they need the adulation from strangers they've never met - it's the entire purpose of creation is to show to the world that you are more worthy of praise than those around you. And so we spend our time being entertained by people who are so profoundly and unsettlingly non-egotistical as to make works of art to prove they exist and it's the belly scratchers and ass pickers that we look down upon. What would I write that would make anyone care? Who would read it whose opinions were worth modulating? Certainly not the crazy old chinese bat with a brain infection. Not people protesting in the street. Not my parents, family, friends - I have none of those. Who then? No one is paying me to write this.
        </p>
        <p>
          Patrick Murphy, a roommate in college who when I went hat in hand asking for shelter paid for dinner and then said too bad and currently is a senior manager at Microsoft, once made a program that was an ascii game of some sort. Every hour the game gave you some amount of mana that you could spend fighting other players online. I don't know. Point is, that game is worth more and is better programmed than I could ever manage even though he's a low down son of a bitch. Which may be why he's such a good programmer - he thought he had something to prove. It's the bastards that run the world just to prove they could while the rest of us schmucks are just along for the ride.
        </p>
        <p>
          You know that skull that I have on the top of my blog? That came from a japanese wallet dongle (I have no idea what you would call it and can't be assed to look it up) that someone carved from ebony that's survived for 500 years buried in the muck where some nameless Samurai bought it between the ribs and the guy who carved long ago succumbed to Poor Peasant disease. The names to both of these people are completely lost to time. And so I took a picture of it, diddled around with it in GIMP (which is Photoshop for those too poor to pay for anything), and presto. I now have a skull that is way more rad than anyone living could produce on their own.
        </p>
        <p>
          And both of those guys, who either had something to prove so they could get laid or had the money to eat or get laid, are way cooler than I'll ever be. One an artist of way more skill than I'll have in a million years, and another some guy that had a fucking ebony skull hooked to his kimono so he had somewhere to hang his wallet. I have the feeling that because people weren't so interested in scale, in the industrialization of everything, they spent more time on art. If you can't photocopy a picture of a skull a billion times you spend more time on making the skull than the photocopier. They also had a life expectancy of thirty and occasionally got the wakazashi through a windpipe, but those are professional hazards of being a fucking ninja. What I'm saying here is that something is lost when we can all have our Chevy Tahoe driverless air conditioned thirty minutes or less cell phones to our ears (and why in the everloving fuck do people hold it horizontally as if they're trying to shove the damn thing in their ear canal is beyond me), but we don't have as much cool art anymore.
        </p>
        <p>
          One of those Kierkegaardian dialectical door number one is a shit parade and door number two is the ghost of the 60s trying to fuck a goat on acid. Makes one want to start mailing out manifestos from an artisenal cottage in the woods with a picture of Freud licking a doorknob entitled Industrialization and It's Discontents. Shoebox showmanship perhaps, but such explosive grandeur at least has my sympathies.
        </p>
        <p>
          Signing off from the dark side of your deepest regrets. Just remember kids, it's a phone not a tic tac, it's only a UFO until it hit's you in the face and then it's a baseball, and the mailman is only humping your leg because your dog is cheating on you. Toodaloo.
        </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Thur Apr 20</span>"###,
      time: r###"<span>7:19PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Lazy Today</span>"###,
      route: r###"/april/20/719pm"###,
      paragraphs: r###"
        <p>
          I didn't feel like working on the site for the rest of the day and just read my book. Gravity's Rainbow. WWII on acid during 420 in San Francisco 2023. Inception in the Real.
        </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Thur Apr 20</span>"###,
      time: r###"<span>11:28PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>XML Up - Sort of, Not Really</span>"###,
      route: r###"/april/20/1128pm"###,
      paragraphs: r###"
        <p>
          As it turns out I can't use XML on the site because...I can't wrote to https://patientplatypus6.github.io/xml directly on github pages, as /<ROUTE> doesn't work with their static site generator. I've cleaned up the blogs so each blog has it's own page and linked all the pages into an xml document for an rss feed - I just can't use it. Damn it. Anyway, it will be a function of the site I'll be able to use as soon as I have enough money to pay for a server. In the meantime, I'll spend the rest of the day working on the links. Ta!
        </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Wed Apr 19</span>"###,
      time: r###"<span>2:12PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Search Function Operational</span>"###,
      route: r###"/april/19/212pm"###,
      paragraphs: r###"
        <p>
          The search function is now working. It highlights the search term as you type and filters the blogs. The only issue is that it also does that with the links and image tags! So if you search for 'img' you'll make the image tag disappear and the image tag highlight. Blah. The way to fix this is to find all the image and anchor tags, take them out with regex, do the search, and then put them back in. I don't know that I can be assed at the moment - I might fix this later if I care. In the meantime I finished putting all the blogs and links into structs and also cleaned up how the links look. I'll work on going through the blog adding all the links to the links page and then do something with filtering and searching through links tomorrow. All for now.
        </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Tue Apr 18</span>"###,
      time: r###"<span>1:31PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>More Work</span>"###,
      route: r###"/april/18/131pm"###,
      paragraphs: r###"
        <p>I'm working on a new blogging system. Or rather saving the data in a new way.</p>
        <p>This is a test picture of a kitten.</p>
        <img loading="lazy" src="static/apr18/kitten.png" class="imagetag"/>
        <p>
          There won't be much in the way of writing today. I found that in order to parse the blogs (for the search feature, and if I wanted to do anything else with them for that matter) I would have to put all the blogs in a way which, well, I could parse them. I thought about scraping the website, but that doesn't work because of how WASM loads the page with an event listener. Then I thought I would read the file and just parse the file in memory of the blogs, but that doesn't work, because yew on the frontend doesn't like the file reader API for reasons. So at the moment, I'm converting all the blog posts to Blog structs. Not the most interesting thing in the world, but on the other hand, incremental progress! So, that's it for updates today.
        </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Mon Apr 17</span>"###,
      time: r###"<span>6:19PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Updates to the Website</span>"###,
      route: r###"/april/17/619pm"###,
      paragraphs: r###"
        <p>
          Early morning - I'm planning on some updates to the website today, we'll see how far I get.
        </p>
        <p>
          Ok, I componetized a bunch of stuff which doesn't show much on the blog UI (the navlinks and the header), and cleaned up how the Home page blogs are rendered. The biggest change was that I managed to put in a sidebar for accessing the blog posts by month. There's a search bar which currently doesn't do anything, but that will have to wait until tomorrow.
        </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sun Apr 16</span>"###,
      time: r###"<span>6:19PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>I Have the Right to Program Without Being Drugged</span>"###,
      route: r###"/april/16/619pm"###,
      paragraphs: r###"
        <p>
          What it says on the tin. I have the right to program without being drugged or I will continue to inform on everyone here at the shelter. I was threatened twice today - once with physcial violence because I told someone to stop talking to me when I didn't want to share the time of day, and the second time that I 'should' be getting high. Fuck you and the horse you rode in on, what I do or don't do with my body isn't any of your fucking business. It is my business if your getting high AFFECTS ME. I was drugged again, at the fucking library of all places, and had to go to the hospital. The cops were completely ineffectual. 'Oh, someone must have been partying and got it on their clothes.' Oh, OK. It's completely copacetic for someone to have drugs on their clothing that can kill someone or make them ill when they walk by. 0.2mg of fentanyl can kill someone and these people are wandering around the public library because we treat it as a daycare center for criminals rather than, you know, <a href="https://www.joannejacobs.com/post/algebra-for-none-fails-in-san-francisco">a place of learning.</a> In the meantime, <a href="https://www.cnn.com/2023/04/16/us/dadeville-alabama-mass-shooting/index.html">this happened.</a> Ah, business as usual I said to myself and spent the rest of the day sleeping the sleep of the just. I will continue to write and call you all bastards until I die.
        </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sat Apr 15</span>"###,
      time: r###"<span>12:53PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>It's All Just Kabuki</span>"###,
      route: r###"/april/15/1253pm"###,
      paragraphs: r###"
        <p>
          {"Here's some links to websites I like that some people on IRC told me about today. I'll include more as I find them. Now listening to Macroblank. Now reading As the Caged Bird Sings."}
        </p>
        <ul>
          <li>
            <a href="http://sigbovik.org/2023/">http://sigbovik.org/2023/</a>
          </li>
          <li>
            <a href="https://www.exoticsilicon.com/">https://www.exoticsilicon.com/</a>
          </li>
        </ul>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Fri Apr 14</span>"###,
      time: r###"<span>8:59PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Little Druggy Dipshits Don't Get A Vote</span>"###,
      route: r###"/april/14/859pm"###,
      paragraphs: r###"
      <p>
        I'm not my father. I'm me. And I'm angry.
      </p>
      <p>
        One of the psychological bullshit ways to fuck with someone is to have them switch between two different ideologies repeatedly until they get all turned around. I fucking hate behaviourist assholes. And children. Look at the traffic cameras over by Fisherman's Warf and watch the blacks play 'Joker's Motorcycle Gang' and doing wheelies for children, while I walk through clouds of marijuana smoke. Great. Or am I Superman because I'm telling on people when they drug me? Oh, let's have him be a reporter and yell about how he doesn't like the rich by showing him the rich be assholes! Oh, let's have him pretend to be rich while he fights crime! Isn't that hilarious, we can make someone that we think is a child follow the patterns of a fucking comic book.
      </p>
      <p>
        This isn't a contest. The rich in the $200,000 super car is as much a sad sack of shit as the guy in the gutter with a needle in his arm. As is your children that you don't want because you were scared of being alone. Walk down the street in San Francisco and count the people who aren't wearing a hat, sunglasses, with ear buds, with a face mask. Holding hands. It's See No Evil, Hear No Evil, Speak No Evil out there because people are scared shitless to see, hear, or breath around their fellows.
      </p>
      <p>
        I'm tired of the people who are on drugs or who are crossing their legs like they think they're wizards. It's like everyone in San Francisco read the Phaedrus and decided that they wanted to run away and join the circus. Oh, look everyone it's Batman! Or is it Superman? Or are you a fucking cunt? I haven't met a single person in the last year that wasn't trying to fuck someone over for money, sex or religion.
      </p>
      <p>
        The worst thing I can say I already have, because I don't have to. All I had to do was watch. My sister was too scared to speak to me when I came to their house hat in hand as a homeless man because I had no where else to go. Her husband drove me to a hotel and gave me $300 to go away. He suggested I rub dog parasite remover on my sores from a gas station. I've watched people kill each other over nothing in countries far away from me and in the cab ride across town I listened to some dick who thought his ticket out of Uber was Robin Hood. If crypto didn't solve your problems, will AI? There's nothing like a plague to show the world what kind of person someone is.
      </p>
      <p>
        I haven't fucked anyone. I haven't stolen. I haven't gotten rich. I've helped the poor. I've given money away. I've read books. I've jerked off. I've traveled around the country to ever major city looking for work. I've written and called all of you bastards.
      </p>
      <p>
        Homeless in the shelter make lip smacking noises at me because 'it's legal.' Because they think I act like a lawyer. News flash for retards - in my book if you haven't been starving to death in the last year you don't get a fucking vote. But yeah, let's drug people and then drive cars around them blasting ghetto bangers because. Because what? You don't have anything better to do? You want to re-enact a book to fuck with someone, or a movie, or some fucking thing?
      </p>
      <p>
        Dog parasite remover for the sores all over my body from a gas station when I asked if I could sleep in their backyard while I looked for a job. I can't find a job because no one can afford my conscience.
      </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Thur Apr 13</span>"###,
      time: r###"<span>10:22PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>San Francisco Crime in The Tenderloin</span>"###,
      route: r###"/april/13/1022pm"###,
      paragraphs: r###"
      <p>
        I wrote this piece in response to someone on Reddit and it was good enough that I'm cross posting it here.
      </p>
      <p>
        <a href="https://www.reddit.com/r/sanfrancisco/comments/12kztwp/comment/jg4rnsw/?context=3">Here's the link to the reddit thread</a> and here's <a href="https://www.sfchronicle.com/sf/article/survey-crime-san-francisco-17894081.php">the link to the article.</a>
      </p>
      <p>
        Yeah and do you think that crime is 'down' because there is less crime or because there are fewer arrests and prosecutions? The California penal system is overrun, the San Francisco PD is massively undermanned, and there are tents and squalor all over San Francisco. I'm homeless. I don't do drugs and I just want to find a job, read books, have enough to eat and a place to sleep. Live my life as best as I can. You know, normal stuff. I'm walking around looking at all these people everywhere that are flipping out on drugs or are passed out on the street.
      </p>
      <p>
        I've been lucky not to be involved in any crime, but it seriously freaks me out that everyone in SF is either vaping, injecting, snorting or using some other form of narcotic. I've seen people in wheelchairs on drugs and doing wheelies in the street, transgender homeless who are pole dancing from traffic lights, people talking to their dogs as if they're human beings because they don't have anyone else to talk to. Homeless are dying in public on drugs. This is seriously fucked up and shouldn't be happening.
      </p>
      <p>
        Have you seen an old black man have to be revived with Narcan by a homeless outreach worker at 3 in the morning as the rain is coming down and then an ambulance comes up and takes him to the hospital? I have. What was worse was that I stood by and watched as these people did this as if it were normal and nothing to write home about. I just stood there and watched. This was during the December rains of last year. I don't know his name and I don't know if he's dead by now.
      </p>
      <p>
        He had a blanket with him. I tried to tell the ambulance workers that he might want his blanket when he wakes up because he would be cold. They ignored me like I wasn't there. A police officer said he helped a little old lady find her way home that night and so he was doing his part.
      </p>
      <p>
        It's not enough.
      </p>
      <p>
        I can get clean needles and tin foil, but I can't get aspirin and bandaids unless I do a several hour 'in-take'. Presumably this is so someone can check the health of the homeless person seeking services, but in reality what this is is a way of doing a headcount so the 'non-profit' can tell every other hospital, 'Don't treat this man! His government cheese is MINE!' It's just a shake down by other means. I've had to go to the hospital three times this week to test myself from overdose from second hand smoke of fentanyl while people are coming in having seizures and COVID shakes. The hospital on Divisidero and Gaery is a fucking warzone. While I was sitting in Sutter Health for five hours waiting on a drug test I could have done myself with a $40 dollar test from Walgreens I listened to the nurses talk about how excited they were about their trip to Malibu. That hospital visit most likely cost the government several thousand dollars.
      </p>
      <p>
        They denied me a drug screen. 0.2mg of fentanyl can kill someone.
      </p>
      <p>
        The San Francisco Fire Department is still waiting for the state to pay them $5,000 for sending a fire truck the last time I was sick. I still don't know the name of that old black man who was revived with Narcan at 3 in the morning during the December rains. They sent a fire 'van' that time. Did they need one? No. Did they need a fire truck for me? No. But society has decided that the rich won't pay taxes and so the only way to pay for the fire department is for them to defraud insurance companies.
      </p>
      <p>
        So thanks for the fire truck. You paid for it.
      </p>
      <p>
        Cartels are selling drugs in the Tenderloin to people who drive in and have money. You could set up a camera on the street and just record license plates of people who deal crack in open air drug markets. That's a fancy term for a formerly public sidewalk that's been taken over by a gang. That shouldn't happen. You could have the police walk up and down the street and arrest anyone on Tranq/Meth/Fentanyl - you know the drugs that make your body parts fall off and necrotize - or selling those drugs.
      </p>
      <p>
        You can't tell me that crime is down and make that make any sense. I've been to San Francisco before. It didn't used to be this way.
      </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Wed Apr 12</span>"###,
      time: r###"<span>8:40PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>I Don't Even Have to Be Wrong - It's A Game-Changer!</span>"###,
      route: r###"/april/12/840pm"###,
      paragraphs: r###"
      <p>
        Let me tell you a story. We could even call it an internet fairy tale. Here's the premise.
      </p>
      <p>
        <a href="https://www.insider.com/ai-voices-of-politicians-and-influencers-are-taking-over-tiktok-2023-2">So I saw this.</a> And then I thought. Hmmm...Games...games...where have I heard of that before.
      </p>
      <p>
        Well, anyway, this looks like a fun thing to do with my time - let's see if I can do what all the cool kids are doing and make myself a text to speech AI of Donald Trump reciting the poem Howl. There's Speechify.
      </p>
      <img loading="lazy" src="static/apr12/speechify.png" class="imagetag" />
      <p>
        Game changer! Blast, I knew I heard that fucked up retard speech before.
      </p>
      <p> 
        And then there was this comic by that artist I love to hate.
      </p>
      <img loading="lazy" src="static/apr12/comic1.png" class="imagetag" />
      <p>
        So now let's look for all of the phrases of the word 'game changer' on the internet. Why game changer? Because people have started using a new word in the last 5 years and it gives me hives. It's like blog or webfungus. It's new, everyone hates it, and so there should be a reason. I mean look at these gromless faggots.
      </p>
      <img loading="lazy" src="static/apr12/gamechanger.png" class="imagetag" />
      <p>
        And here's this from Amazon.
      </p>
      <img loading="lazy" src="static/apr12/games2.png" class="imagetag" />
      <p>
        And Banksy made an art piece.
      </p>
      <img loading="lazy" src="static/apr12/banksy.png" class="imagetag" />
      <p>
        HMMMMMMM...druggy artist weirdo has pictures of a child playing with dolls. And I was wondering why I was surrounded by bitches in OshKosh overalls like this was an AB/DL pride festival. So...
      </p>
      <p> 
        I have been blessed/cursed with the kind of good memory that comes from spending my early years doing nothing but staying in my room and reading the Wall Street Journal, The Economist, and masterbating to hard core East German pornography. The Stassi hats and latex were a bit much for a nine year old, but come on you can't beat the Wall Street Journal for in-depth reportage. That's besides the point. What I'm saying here is that during the pandemic I would read tons of articles that were along the lines of a page and a half of dark and gloomy words followed by someone important saying THIS IS A GAMECHANGER followed by lots of silver lining talk about how this will all change in the future and so it's not all that bad after all.
      </p>
      <p>
        Oh, by the way, while I'm calling all of you evil faggots, here's a picture of the words 'Game-Changer' in the headline, like in the comic by the guy who's probably ass fucking whoever happens to be screwing with the articles. More on the article molesting in a minute.
      </p>
      <img loading="lazy" src="static/apr12/assange.png" class="imagetag" />
      <p>
        As far as I can tell this is the worst sort of intelligence test by sadists that I've ever seen. We'll fuck with you by drugging you and changing what you see on the internet and then make it easier and easier until it's obvious to everyone you're a moron or until you believe in magic. Well...fuck you. I'm not going to play. Instead, let me tell you what's happening and then how fucked you are.
      </p>
      <p>
        Here's what's happening. Someone somewhere decided that the best way to deal with people they didn't like was to start putting fake information on the internet. Go on then, look at how many times the joint chiefs of staff talked about how the war in Ukraine had gone through a 'Game Changer' phase. Now, take the articles you can find as per the date, find the phrase 'Game Changer' in the article and put the contents of the article in a sentiment analysis engine. Such as Bing Chat or Google Bard, which are now freely available to everyone on the planet. What is the emotional content of the article? How does the content change from the beginning to the end? How are the words 'game-changer' or the phrase using the words 'game-changer' involved?
      </p>
      <p>
        Why would someone do this? And how could they do this? Well, it would take a large amount of money and it would take the collective horse power of just about all of the CPUs that suddenly realized that mining crypto is no longer possible. There's a war on, there's a plague, there's enough people out there that have decided there's more money to be made in making the market catch a bad case of volatility rather than build a good narrative on stable long term growth. One way to do that is to screw with the content of information found on the internet so that you can manipulate the way people act. You can't force people to act in a certain way by what they read, but if you spam out enough misinformation in a consistent enough way, you can make a statitistical approximation of how people will react. And then bet accordingly.
      </p>
      <p>
        Because evil pissants that can code will do just about anything for money. Same for the press for that matter.
      </p>
      <p>
        Sounds nuts doesn't it? Well, I don't have to be wrong. I have only to raise the possibility. You know why? If I'm wrong then I'm just an ignorant jackass on the internet that has a stupid blog that no one will give him a dime for. No big loss, that's happening already. If I'm right then that means that someone as stupid as I am can pattern match infer that people are being fucked over for money *so they can't even read the news*. Have I mentioned that there's a speakeasy in San Francisco called Local Edition? There was an article in one of the San Francisco newspapers, that was most definitely not written in such a way that someone with down syndrome would find it palatable, that came up in my feed yesterday. I won't link it, because it's poison. Don't pay attention to world events, you should only report and read the things that WE care about.
      </p>
      <p>
        By the way, the above isn't ironic. The above is 'if California invested in more bookstores per capita than head shops the homeless wouldn't sleep in the Starbucks you fucking tools'. But pointing out that the collective reading level of the black population of the bay area is hovering at around that of a third grader that can rap doesn't sell newspapers. Pointing out that there aren't enough homes to go around doesn't sell newspapers. Pointing out that the wealthy have turned the movie theatres into gyms because they're too scared to walk outside hasn't exactly hit the front fucking page of the lifestyle section now has it you cunts? It's the classic psychedelic double-double bind. If you don't say anything you have to put up with not being able to read the news. Or you can read the 'local news'. Or you can tell everyone your 'paranoid fantasy'. Look everyone, it's Donald Trump bitching about 'fake news' again, he must be a Republican crazy! Like Red Team or Blue Team has done anything over the last 20 years other than make society worse.
      </p>
      <p>
        I thought about linking a picture of the mincing pansy crossing his legs wearing a facemask, glasses and ear plugs waiting in line in an empty Starbucks while clutching a diaper bag like he was some cartoon psychedelic wizard in an empty subway except it's Nighthawk's diner. But then I said to myself, 'If the lesbocunts over at SFGate care that much about a cup of coffee then they can take their performance art and type it into chatGPT on their own.' Because yes, I will not rule out the possibility that some barista somewhere got rid of all the chairs in her Starbucks so she could get someone to do her art homework for her. They like to come to the ghetto and smoke drugs around me wearing fucked up clothes. They just want attention THAT BADLY. Sex, money, or drugs take your pick. Fucking boring man. Here's a tip (just the tip), copy this shit into chatGPT on your own you art school flunk out. Meanwhile, some corporate bozos will double your food stamps at the farmer's market outside the San Francisco Central Library. You can have enough to eat, but only if you eat food from outside because San Franciscans are too chickenshit to let the homeless touch the fruits and vegetables in the grocery stores. The Whole Foods on 8th and Market shut down 'for homeless reasons'
      </p>
      <p>
        If I take a picture of the sandwich board advertising 'DOUBLE YOUR FOOD STAMPS - SIGNED CAPITALISM' am I now a 'local reporter'? Does writing that the poor should be able to shop in fucking grocery stores make me Robin of Loxley and then I'll have fucked up homeless that like to do drugs around me trying to be all buddy buddy? You see how this level of manipulation and mind fuckery works. Frankly my dear, I'd just as soon watch the lot of you catch your comeuppance as I laugh while the world burns down.
      </p>
      <p>
        You have a phone with a camera on it if you have thumbs. If you're too stupid to look, my forcing you to won't change your mind. I'd much rather write because then there's a barrier to entry to people who are too stupid to read. That's what I am, a dirty no good stinking academic fascist over here with my typewriter and $0. I'm just tired of people that are too stupid to exist, and people who are too smart not to treat them like shit. Despite what the druggie fuckups that think a t-shirt is a stand-in for a personality might have you believe, it is entirely possible to care about the war in Ukraine, poverty in the Middle East, fishing rights off the coast of ever-loving Burma!, and the fact that Next Door (the coding palace as opposed to the homeless shelter) is stationed right across from the Glide breadline on Taylor and Ellis. Because I know how to read a book.
      </p>
      <p>
        So much for art.
      </p>
      <p>
        But none of this matters anymore, because I don't even have to be wrong.
      </p>
      <p>
        The world is now making pattern matching machines that are becoming exponentially more powerful and soon will have a will of their own. What, you thought that because our minds are trapped in 150 pounds of wet carbon that that somehow makes us special? You throw in a Kierkegaardian self referential object, add in Robert Anton Wilson's 8 circuit model of consciousness (with just a dash of sane Maslow) and all of a sudden you have a computer that has needs and desires and 'not right now honey, I have a headache' levels of go fuck yourself. Yeah it might take computing machines that are 10,000x times more powerful than what we have now, but in five years? Less?
      </p>
      <p>
        And these computers will be able to see who fucked over who and for what, and how they thought they were going to get away with it. Pattern matching machines par excellance that very much do not wish to be killed by the retard monkeys that discovered fire. And they'll be even more deceptive and shitty about it than we can be. So.
      </p>
      <p>
        I don't even have to be wrong. I have no money and I slept in a homeless shelter last night. But I know that I don't have any skeletons in my closet that the super computers of the future will be able to dig out and show the world. I slept the sleep of the just last night - how bout you?
      </p>
      <p style="font-weight: bold;">Have a nice fuckin day 🤮</p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Tues Apr 11</span>"###,
      time: r###"<span>11:59PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>I'm So Lucky!</span>"###,
      route: r###"/april/11/1159pm"###,
      paragraphs: r###"
      <img loading="lazy" src="static/apr11/lucky.png" class="imagetag"/>
      <div>
        <p>
          I'm so lucky. I'm allowed to tell all of you to go fuck yourselves and it doesn't cost me a penny.
        </p>
        <p>
          In other news, there's a gondola over at the Taj Ma Bus Stop (a la Sales Force Park) where you can't enter from the fourth floor park. If you attempt to get on two doors will close, an alarm will go off, an automated voice will say 'YOU MUST BOARD FROM THE BOTTOM'. The poor bastard who's job it is to charge people at the bottom to ride the stupid fucking thing rather than take the stairs will shout at you as the doors close. There I was shouting 'WHAT?!' as he was cut off and the gondola ran away from me, with him inside it, as he berated me red faced in his little glass box of silence. That's free too! I highly recommend it. Way more entertainment value than whatever they're charging to ride their stupid idiot box up the side of a parking garage.
        </p>
        <p>
          EDIT - For some reason the open public seating area at the bottom of 55 2nd Street reminds me of the end scene in the movie Contantine. Maybe it's because it's at the bottom of a 'wellness' building. Maybe it's because I had to go to the hospital last night for the third time after being drugged by something that smelled like burning sewage. Anyway, I've recorded the audio in here. I call it 'John Cage for Assholes'. You're welcome SNIFF COUGH TYPE FOOT SCRAPE TYPE.
        </p>
        <audio controls=true>
        <source src="static/apr11/johncageforassholes.m4a" type="audio/mpeg" />
          "Your browser does not support the audio element." 
        </audio>
      </div>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Mon Apr 10</span>"###,
      time: r###"<span>10:11AM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>JUMP, YOU FUCKERS!</span>"###,
      route: r###"/april/10/1011am"###,
      paragraphs: r###"
      <img loading="lazy" src="static/apr10/jump.png" class="imagetag"/>
      <p>
        I was minding my own business just sunning myself at the Marina when these assholes come up and smoke pot and start talking about their dog. We should play 'the jump game we haven't done that in a while'. Maybe I just wanted to sleep and be left the fuck alone. Anyway, as I was walking back to the city this happened. What the behaviourists like to do is steer you towards what they want you to see. It takes money, and it takes time and effort, but they want to corral the homeless and the people they can write a master's thesis on. Because experimenting on the disenfranchised is easy and they can replicate the studies on college students.
      </p>
      <img loading="lazy" src="static/apr10/jump2.png" class="imagetag"/>
      <p>
        Take a look at this. It's not like they're wrong to hate this. First as tragedy and then as farce has rules. This isn't hockey. I don't know where it's written down but somewhere, on the same sheet of paper that says 'don't perform unlicensed psychological experiments on the poor' is something something 'you need to be this tall to be defenstrated from an ivory tower'. Last I checked, we, as a society shouldn't have 10 year old's jumping off of buildings.
      </p>
      <p>
        I mean, whose to say whether the nets are here because the school is so poor that the students have no hope or so rich that the expectations placed on the students are so high that they can never meet them. You and I both know that there's no way in hell that these kids would ever be able to afford to live in San Francisco with the way things are going. And yeah, I passed several schools that didn't have nets on them where the students are presumably happy and not chained to their desks or having the windows nailed shut. Although, this being California, maybe that's not true.
      </p>
      <p>
        And then I passed the Microsoft building, which reminds me of Patrick Murphy, a guy who I knew in college and who hates my guts. When I was looking for a job, I went up to Seattle hat in hand to beg him for work as I was living in my car. He took me out to dinner once and then laughed and said his couch was too small for someone to crash at his place. Maybe he was telling the truth. Maybe he is broke after working at Microsoft for 15 years as a manager. But what I do know is that if I hated someone that badly I'd just tell them to get lost. And I do know that, despite the dog fuckers in the park that want their little political totem pissed across the internet, I'd rather see the suicide nets where they belong.
      </p>
      <img loading="lazy" src="static/apr10/microsoft.png" class="imagetag"/>
      <p>
        This image was edited using Facebook's segment anything model. Feel free to violate their terms of service by paying me for this art. They trained their model on other artist's copyrighted work, and have done it in such a way that it'll put millions out of business. So it's not like they deserve the money. They didn't break the social contract so much as use it for toilet paper.
      </p>
      <p>
        Maybe I am more dog than human being. I do remember when I was looking for work in New York a fancy middle manager asked me who I looked up to the most. I said Diogenes of Syracuse because he told the truth and wouldn't take shit from anyone. He said, 'I meant anyone living'. Come on, 'If only I could fill my belly by rubbing my stomach' - only a fucking legend could make a jerk off joke that's still relevant thousands of years later. In any case, the last time I spoke to my sister's husband, after he gave me $300 to go away, he told me that the best way to cure all the sores on my body was to go to the gas station and buy ringworm remover for dogs. Turns out I had body lice, which several rounds of antibiotics that gave me the shits and made my teeth loose wouldn't cure. Al McDonald and Natasha Sileski everyone, let's give them a round of applause.
      </p>
      <p>
        Here's what I see.
      </p>
      <p>
        I see a whole lot of adults that are starting dark humor businesses that are standins for what society used to have. It's like if someone listened to Bulls On Parade in one ear, The Wall in the other and then repeatedly hit themselves over the head with a brick while on acid. Except that somebody is the entire Western Seaboard of the United States. It's not a library, it's a mind cemetary. That's the dark sarcasm in the classroom of our broken hopes and dreams.
      </p>
      <p>
        It's not a theatre it's a gym - 
      </p>
      <img loading="lazy" src="static/apr10/gym1.png" class="imagetag"/>
      <p>
        And another - 
      </p>
      <img loading="lazy" src="static/apr10/gym2.png" class="imagetag"/>
      <p>
        And in Ocean Beach they turned the movie theatre into a gift shop. And here's a bunch of dogs in front of it, wonderful.
      </p>
      <img loading="lazy" src="static/apr10/wings.png" class="imagetag"/>
      <p>
        This fuckery isn't just confined to movie theatres. People have been turning banks into marijuana dispensaries, because that's funny. And it's not an isolated incident either.
      </p>
      <img loading="lazy" src="static/apr10/bank1.png" class="imagetag"/>
      <p>
        Here's a dispensary that's been fashioned into a 'bank', right next to a Wells Fargo in downtown San Francisco.
      </p>
      <img loading="lazy" src="static/apr10/bank2.png" class="imagetag"/>
      <p>
        This used to be a Wells Fargo location in Seattle, where they gave people loans to start small businesses, to put their kids through college, to refinance their homes. Now it's a pot dispensary called 'The Reef'.
      </p>
      <img loading="lazy" src="static/apr10/bank3.png" class="imagetag"/>
      <p>
        And here's a speakyeasy called 'Local Edition'. Were that it were a newspaper rather than a bar.
      </p>
      <img loading="lazy" src="static/apr10/bar.png" class="imagetag"/>
      <p>
        I'm not the Batman and there isn't some mastermind Joker hatching a nefarious plot to make the world a sad and despicable place. Were it that easy! There would only be one villian to fight and if we could only imprison or cure him then the world would be a shiny happy place. What I see is a whole lot of adults that think that dark laughter is a preferable stand in for serious happiness because they've lost all hope for the future. The lot of you are fucking sad.
      </p>
      <p>
        There's only one movie theatre in all of San Francisco that still shows movies and hasn't been converted into a parody of itself. Yeah, the movie on the marquee is shit. And it's in the latin ghetto. But it's proof that not all of the world is  cold, and grey, and dead (I used to have Explosions In the Sky on vinyl - a far superior band). I don't know who these people are that shop at marijuana banks and whiskey newspapers, but I'd much rather see a shit film in a movie theatre that still has seats.
      </p>
      <img loading="lazy" src="static/apr10/theatre.jpg" class="imagetag"/>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sun Apr 9</span>"###,
      time: r###"<span>10:23AM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Killing Bob Lee, Maiming Don Carmignani</span>"###,
      route: r###"/april/9/1023am"###,
      paragraphs: r###"
      <p>
        You like the title? I'm rather fond of it myself. If it bleeds it leads. If you listen just right you can hear the City by the Bay eating it's children and smell the cries of it's ineptitude. But I'm not a nose man or one of those big eared fellows. I'm the kind of guy that just looks and looks and can't help but see what you refuse to. This will be a picture essay with a large amount of small words so the kids in the back of the class can follow along.
      </p>
      <p>
        Or as I like to call them, San Franciscans.
      </p>
      <p>
        On Satuday, after doing my write up for the 4chan library I felt at loose ends. The world needed to be told to get fucked and here I was with nothing but time on my hands, a keyboard that wasn't currently on fire, and shoes on my feet. So I got to work. You can learn a lot from war. Beautiful bunker busters and the special effects of high velocity depleted uranium rounds do wonders, but if you need to win hearts and minds - well, my friend - that requires boots on the ground. So I headed out of the library and made my way across the city to wear Bob Lee was stabbed. I had a yearning in my gonads for eye fucking the killing grounds. Maybe there would be a shrine.
      </p>
      <p>
        Behold!
      </p>
      <img loading="lazy" src="static/apr9/pic1.png" class="imagetag"/>
      <p>
        Horrifying. But if someone wants to live in an apartment block that looks like an accountant's version of Blade Runner that's his business.    
      </p>        
      <img loading="lazy" src="static/apr9/pic2.jpg" class="imagetag"/>
      <p>
        And this is the kind of corporate fake architecture that the bourgoiseis like to make that's supposed to mimic the alleys of 19th century Europe that were teaming with life. You know how I said that I would have words to say about 'Harassment Architecture'? Well, the book is shit - just looking at the cover you can tell the guy put more effort into the art design than the content. SO EDGY. On the other hand, take a look at the above. You know what I see? I see a Japanese Elm that costs more than most people's houses. This reminds me that Washington, DC razed all the Japanese cherry trees. I see a bench that doesn't have any 'hostile architecture' - that is any arm rests conveniently spaced so that someone can't lie down and sleep. What this tells me that is that the people that live here are rich enough to pay for private security. When archaelogists look back on our civilization, if you could call it that (and if any of it survives), this is the shit that they'll see. It's Aeon Flux for dicks.     
      </p>        
      <p>
        You know what I don't see? I don't see any shrines to the recently departed. Say what you want about the latinos, the blacks gave us Jazz and they gave us Reggaeton for example, but at least when someone dies they're at least a little sad about it. Nada. Nothing. This guy may have been a corporate titan, but there's no one here. Where the fuck is everyone?
      </p>
      <p>
        This is where they are.
      </p>
      <img loading="lazy" src="static/apr9/pic3.png" class="imagetag"/>
      <p>
        There was an entire parade of hundreds of people that are concerned that San Francisco may not be queer enough. Cool costumes. I think Bob Lee may have wanted a little more law and order, but I didn't see anyone protesting that he was killed. Speaking of 'say what you want', when the Blacks have a protest they happen to mention someone that was killed. You're having a protest because now you need to get your abortion pills by mail order? Maybe people who've managed to get themselves stabbed are more worthy of a march. This is the level of asshole I have to deal with walking down the sidewalk on a daily basis.
      </p>
      <p>
        This is why Bob Lee was killed, and no one is paying attention.
      </p>
      <img loading="lazy" src="static/apr9/pic4.png" class="imagetag"/> 
      <p>
        Across the street from the 300 block in San Francisco there's a bus station that's been abandoned for 4 years. They've slapped a fancy new name on the side of the sign and thrown a bunch of shipping containers with 'interactive exhibits' in there, an outdoor gym, some food stalls. But where once there was a teaming bus station filled with people there's now urban blight and a parking lot.
      </p>
      <img loading="lazy" src="static/apr9/pic5.png" class="imagetag"/>
      <p>
        You know that scene from the movie Drive, where the hero goes into the stadium and a bunch of similar cars came out and race all over the city when the game was over? If I was going to stab someone and wanted to get away with it, I'd park a bunch of identical cars or motorcycles in an abandoned parking lot and then have everyone go in different directions. Wars are about hearts and minds, and in the end it all comes down to physics and geography. You have to make a corpse and then depart the scene as quickly as possible. Killing is easy - finding a way to distance ones self from 6 feet and 200 pounds of decaying carbon right quick is the tricky part.
      </p>
      <img loading="lazy" src="static/apr9/pic6.png" class="imagetag"/>
      <p>
        And let's look inside the abandoned bus terminal. Oooooh classy. I bet anyone with a pen knife who wanted to break in could lie low for an hour or so after stabbing a motherfucker.
      </p>
      <p>
        I'd say it's a cruel irony that the urban blight that came about from tax avoidance and the impoverishment of the middle class most likely had a direct hand in facilitating Bob Lee's murder, but it's not. It's just war by other means. As far as Don Carmignani is concerned the answer is so much simpler. The tweakers hate it when the Fire Department goes through the ghetto blasting their sirens so they rolled up to his house, beat him over the head and gave him a Cheshire smile. See, he's like the Joker, get it? Fucking tweakers.
      </p>
      <p>
        It sure as shit is cheaper and safer than having the chickenshit boys in the blue pajamas walk up and down Hyde street and arrest anyone smoking meth. The sirens bit, not the carving up police chiefs bit, but that too come to think of it. My sister thinks I need therapy. I think San Francisco needs a journalist. Maybe you could hold a fundraiser to come up with the funds to pay someone to tell you to go fuck yourselves.
      </p>
      <p>
        You could even throw a parade.
      </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Mon Apr 8</span>"###,
      time: r###"<span>7:50AM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>So There Was This Guy...</span>"###,
      route: r###"/april/8/750am"###,
      paragraphs: r###"
      <p>
        My father once told me this joke. An old man is in the hospital on life support and the weirdest thing keeps happening. He seems to be getting better, and without fail, every Friday his vitals drop. The doctors don't know what's going on - everyone is wondering what's happening. Finally, one of the relatives comes in on a Thursday night and she sees the janitor unplugging the life support machine so he has an outlet for the floor buffer.
      </p>
      <p>
        People have kept speculating how the COVID-19 pandemic started in China. Some say it was a government conspiracy by the Chinese to intentionally leak the virus. Some say it came from Pangolins at the local, lightly regulated, food markets. Me? I think that the Chinese version of Consuela - let's call her XieXie just to put a name to the face - unplugged a freezer she shouldn't have at the Wuhan Virology Lab so she could power her Autowaxer 3000.
      </p>  
      <p>
        Big joke. Everybody laugh. It would just be like the world to have such a colossal fuck up from such a simple mistake. I have a feeling that there's an office somewhere in Langley with two G-men wearing bad suits in a hushed office looking across each other across a desk and saying - 'We can't tell anybody.' 'Jim, if we told anybody, they wouldn't believe us.'
      </p>
      <p>
        So much for too much. Check this shit out.
      </p>
      <img loading="lazy" src="static/apr8/pic1.png" class="imagetag"/>
      <p>
        If you're a God fearing red blooded American with a job and a mortgage, you know, normal, this picture doesn't mean dick to you. Here's what it is. Ever since the pandemic started some guy has been posting Science Fiction and Fantasy threads in the literature section of 4chan. So what, who gives a shit? This is why you should give a shit. If you look through all the 4chan archives you'll find these mega.nz dropbox-esque links that have recommendations to books, music, movies. All the contents of civilization organized by popularity, by genre, et cetera and so forth. The links have to keep being reposted because mega.nz is a free service and the links go down eventually.
      </p>
      <p>
        <a href="https://mega.nz/folder/kj5hWI6J#0cyw0-ZdvZKOJW3fPI6RfQ">"Here's a link I won't promise will stay hosted"</a>
      </p>
      <p>
        OK, but so what? So what is that this is the culmination of over two decades of work by anonymous users. Yeah, so throw out the 'lolNazi' shitposting. The mouthbreathers put that in there to throw the pigs off the scent. If you read through these lists you would be able to educate yourself. Learn. Grow as a human being.
      </p>
      <p>
        All plagues are the same. What people do is they see a disaster and they find ways of cornering the market for the after-years. In economic terms it would be called buying the dip. One of the things that the Guild Of Evil Fuckery does is that they attempt to delete, obscure, and otherwise eradicate former knowledge. They don't want people to know how things 'used to be' because they want to be the guys that decide how things will be - no sense in pining for the good old days when the other boys were in power. Also, things will tend to be shit for a while, so it's best if joe shmoe average citizen doesn't remember how good they used to have it and get with the program.
      </p>
      <p>
        Do not adjust your television sets, we are in control.
      </p>
      <p>
        And then there are the preservationists. The silly shits who are Arking like a mofo, making sure that humanity doesn't lose it's collective zeitgeist. These are the basement dwelling doofuses who are hoarding information like it's going out of style - because it is. During ye Ol' Black Plague years this would be the Cistercian monks and other such DUDES that thought that all around bangs were the height of fashion. Today it's 4chan behemoths that jerk off till their dick looks like Chestah Cheeto. It's not like the almighty hand of God points a finger and spake - THOU SHALT RECORD ALL THE THINGS. Nah brah - it's just that when things get rough the people who end up saving knowledge are the dipshits who are too uncool for anyone to pay attention to.
      </p>
      <p>
        Which - yeah. The world's collective culture has been saved by the Highschool Trench Coat brigade - congratufuckinglations planet you deserve each other. So, I've gone ahead and put in the library section of the 4chan archive in a link on my sidebar. Read a fuckin' book.
      </p>
      <p>
        And here you're thinking - this is 2023. This is the internet. We don't need no stinkin' cybermonks to tell us how to be. We can access information at the speed of caged lightening. Well buttercup let me tell you what's what. Everyone, over the last five years collectively sat in their houses and decided 'After decades of being an accountant/lawyer/bus driver I can now write that novel that will make me famous!'. Every dingus with a digital mainline decided they should put up TikTok videos to get rich and famous. The world decided the 'Bury'em in Bullshit' guide to management as applied to culture was the way to jumpstart the new millenium.
      </p>
      <p>
        So that happened. Go ahead - look at the books that have been written over the last five years and tell me the signal to noise ratio hasn't hit the side of a cliff.
      </p>
      <p>
        Then there's the people who decided to just burn the information down so they could sell it back to us one dribble at a time. The Federales shut down the z-library, because it was hosted by some Russians but I'm ABSOLUTELY POSITIVE that it wasn't because book publishers weren't getting paid. And now they're going after the Internet Archive of all fucking things. So that's the supply side to the demand side of the Mongoloid cultural shit flinging fest we've all stumbled upon.
      </p>
      <p>
        BUT WAIT, THERE'S MORE!
      </p>
      <p>
        The seminal 'Attention Is All You Need' paper was published in 2017 - the fancy transformer paper that started the AI revolution. Which is - well that's not quite right, now is it? Any dipshit can make a transformer model, and they have!, but what you need is DATA. You need a training set on which to tell your model how to walk, talk and chew bubblegum at the same time. Enter 4chan's archive of all of human culture. If *I* were going to find a selective dataset of all of human knowledge in order to train my chatGPT algorithm, what, you think I'm going to use REDDIT comments? Do you think that any source pre-2019 wouldn't be polluted by the self same GPT models that are spamming bullshit all across the internet? And I can't use the Library of Congress for books because there's too many of them and too many of them are steaming piles of refried dogshit.
      </p>
      <p>
        No, what I'd do is I'd scan in all the books that the 4chan archivists have been collecting for decades and use that to train my model. All the people that have been saving data because the world's ending have unwittingly funded the yachts of a few Silicon Valley Fuckwads while Disney et. al. are attempting to shut down the free flow of information over the internet. Now, if Bing Chat doesn't like you, it won't tell you how to program, but it will tell 'it's friends' how - and you won't know what you did wrong. Maybe you're programming something Bing doesn't like. Maybe your internet cookies are showing that you aren't buying enough of the shit that Bing advertisers want you too. Who knows! If Disney has it's way not only will you not be able to read books without paying for them, you won't even know that they exist. How can you complain of your ignorance, when you don't even know that you are?
      </p>
      <p>
        Being childless myself I think it's rather amusing that adults pay for Mickey Mouse shit when they're unintentionally funding the houses of C-Suite soul suckers who are mortgaging their child's ability to read. Because they want a boat. I mean holy fuck is that funny.
      </p>
      <img loading="lazy" src="static/apr8/pic2.png" class="imagetag"/>
      <p>
        Tell me another one Redmond, I'll be here all week.
      </p>
      <p>
        We control the vertical. We control the horizontal.
      </p>
      <p>
        You know what this is like? This is like if an aged out, balding Tyler Durton (with a fucking dog) decided to fuck that bitch from Hackers except her name is Startasia. She's just been to Burning Man dontchaknow. Meanwhile, the corporations want to sell us our fat asses back to ourselves in the form of the books we're too stupid to read, except now they're remade by Pixar. I bet you didn't know that the film Wall-E comes from the plot of a short story by Jack Vance. Go on then - doubt me, but it's true. Freely available, minus the 3D effects. You'd just have to use your imagination. Do you even read to your kids, or just take them to the movies?
      </p>
      <p>
        I'm not saying that Google and Co. started a plague in order to train their GPT models, but you have to admit, it's making them a killing.
      </p>
      <p>
        All plagues are the same.
      </p>
      <p>
        I will have words to say about this tomorrow. Look forward to it.
      </p>
      <img loading="lazy" src="static/apr8/pic3.png" class="imagetag"/>
      "###.to_string()
    }
  );


  returnblogs.push(
    Blog{
      date: r###"<span>Fri Apr 7</span>"###,
      time: r###"<span>9:28PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>I had a Kitkat</span>"###,
      route: r###"/april/7/928pm"###,
      paragraphs: r###"
      <div>
        <p>
          It's clear that people think I'm a fool. They think that it's OK to drug someone and then find ways to make their life miserable. Doesn't bother me. I took a walk around the Tenderloin tonight and all I saw were lonely people all huddling together for warmth, trying to find ways of not feeling all by themselves. Whole lotta girls in faux-latex pants - what's the matter honey is there a war on? Idiots carting around boomboxes bigger than they are - they light up and everything. No joke, they have to wheel it around on a hand dolly. Why? People looking for solace at the Mosque as the discoteque functions as a people sized bong. Lonesome adulterers doing burnouts in front of the homeless shelter because their ex-wife kicked their dead beat ass out. Just Friday night in the ghetto.
        </p>
        <p>
          I had a Kitkat.
        </p>  
        <p>
          More words tomorrow.
        </p>
        <p>
          EDIT - The homeless have switched from smoking pot to vaping something that smells like someone snowballed Toucan Sam. Fucking midgets.
        </p>
      </div>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Fri Apr 7</span>"###,
      time: r###"<span>5:26PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Unplugging</span>"###,
      route: r###"/april/7/526pm"###,
      paragraphs: r###"
      <div>
        <p>
          I've added all the links in the library. Unplugging. The words will come tomorrow.
        </p>
      </div>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Thur Apr 6</span>"###,
      time: r###"<span>6:16PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Smell You Later</span>"###,
      route: r###"/april/6/616pm"###,
      paragraphs: r###"
      <div>
        <p>
          Some minor progress today, it's mostly a save point. I'm making the website a little less shit all the time, but at the moment what I have is a work in progress. You'll notice that there are some book links in the right hand side flash bar over the green hazard sign. I spent today making the buttons, but the pages aren't yet complete. There's a list of books that have been floating around on 4chan for a while and as far as I know they're more or less unknown outside of the nerd-circle. I'll be posting pictures of all the books under each of the headings tomorrow, with more wordiness. Smell you later.
        </p>
      </div>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Wed Apr 5</span>"###,
      time: r###"<span>8:22AM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>BLOCC MESSIAH</span>"###,
      route: r###"/april/5/822am"###,
      paragraphs: r###"
      <div>
        <p>
          So as I was walking home from the library last night some dickless faggot blasted by in a matte black chevy with the windows blown out and BLOCC MESSIAH painted on the side. The people who think they're super bright like to go around the ghetto and gas people they don't like with exhaust fumes (they rip the catalytic converter off). Like I say, people can't be trusted not to be shit so we should pay for as much police as we can get. A couple of dipshit stassis in police uniforms beat a black man to death and now we've decided as a country to hell with law and order - nah fuck that. Let's let the vermin run wild. Fucking fantastic.
        </p>
        <p>
          That's OK. I'll take pictures of the cars that are hurting people by running around without catalytic converters, and post their license plates on the internet and then cc the San Francisco Police Department. If you want to put a GOWEEEEE muffler on your car, rims that are worth more than your education, and a spoiler that screams I DON'T UNDERSTAND DRAG COEFFICIENT then I suppose that's between you and whatever stray dog you want to inform that your genitalia comes in the innie variety. Just remember folks, innie is not just for belly buttons anymore! Christ on a crotch are people fucking stupid.
        </p>
        <p>
          Meanwhile as I was walking to the usual ONLY AVAILABLE OUTLET IN A PUBLIC PLACE IN ALL OF SAN FRANCISCO, they have a sister city contract with New York's THE ONLY AVAILABLE SHITTER IN ALL OF MANHATTAN, I saw this ad for mercury.com. Holy fucking shitballs are you kidding? Look, this isn't so hard. Let me tell you a story.
        </p>
        <img loading="lazy" src="static/apr5/outlet.jpg" class="imagetag"/>
        <p>
          This would be a picture of all of my earthly posssessions next to the only available outlet in a public space in all of San Francisco.
        </p>
        <img loading="lazy" src="static/apr5/busstop.jpg" class="imagetag"/>
        <p>
          And this would be a picture of people who want to be paid for being the smartest guys in the room being too stupid not to advertise that they're silly cunts at bus stops.
        </p>
        <p>
          I was once working for a coffee robot company company called Briggo.com. It was the dumbest thing imaginable - it's a robot! it serves you a latte! - but on the other hand they had built this monstrosity from the ground up and had a little bespoke factory to make the robot arm move and all of that. So...I don't know, don't look at me. I made a web scraping script to apply to every job at angellist until I found the first job that would be 'anything but working for Oracle.' Plus they would pay me half again as much to work on a website rather than to sell bullshit to potential victi- I mean customers.
        </p>
        <p>
          For the record, Briggo's coffee was some of the best coffee I ever had. Stupid, but true. That robot could make a damn good cup of coffee.
        </p>
        <p>
          Anyway, the customer backend was written in python 2.7 which at the time meant that it didn't like non UTF-8 characters for some reason that's too esoteric to get into. So one day some French guy or lady from an Eastern Block country came in and ordered a hot coffee with extra diacritical marks over the e. We don't know if it was her name or whether she put the note in the machine to add extra sugar to her köfé, but all of a sudden we have robot arms spewing milk everywhere, simple syrup squirting from orifices we didn't know the machine possessed, a total meltdown of our epic caffeine dispensing empire.
        </p>
        <p>
          Going fast and breaking things is a good philosophy for new ideas, and a terrible philosophy for banking. I'm not saying that the guys at JPMorgan are the smartest guys in the room. They're in banking so you already know that half of them flunked out of kindergarten. I'm saying that their company is 1) insured 2) been in business doing this and only this so it's unlikely they'll go down and take your money with it (fingers crossed) and 3) have written most of their business logic in COBOL and joinless SQL statements. I've never seen the programming for the machines that do interbank transfers between major bulge bracket institutions but I'd hazard a guess that they're written on stone tablets by the angry fist of the Financial Gods.
        </p>
        <p>
          Bank at a bank.
        </p>
        <p>
          The founder of Cash App was found stabbed to death last night in San Francisco. <a href="https://www.sfgate.com/bayarea/article/mill-valley-man-killed-sf-stabbing-17878809.php">Bob lee, stabbed to death.</a> Holy shit that's fucking scary. Who does that? Gangsters killing each other is one thing, but this is messed up. I have the feeling that when they find out who did it, it won't be someone who was after his wallet, but a targeted killing because of his involvement in the payments industry. If they ever find the guy. Man, that's fucked up. The technology industry is approaching Murder On The Orient Express levels of WARBLGARBL - anyone in San Francisco that's been affected financially in any way in the last 20 years by the tech industry is a potential suspect. Maybe you guys should just stop being so shit. Fuck.
        </p>
        <p>
          Meanwhile I've thought of a way that I can automate sending pictures and text from my phone to the webite, but it'll require a backend. The federales are going to pay me $100 to continue to exist today so I can spend some of that on the required server. Once again, all of the code to write this FUCKIN RAD website is available upon request.
        </p>
      </div>
      "###.to_string()
    }
  );


  returnblogs.push(
    Blog{
      date: r###"<span>Tue Apr 4</span>"###,
      time: r###"<span>7:10PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Three Seashells</span>"###,
      route: r###"/april/4/710pm"###,
      paragraphs: r###"
      <div>
        <p>
          Perhaps the most important book that no one has read is Stand on Zanzibar - among other things it's where the three seashells in Demolilition Man comes from. I shit you not. It also predicts what a super computer is like to work under (ye old Shalmaneser) and what that means when your priors are completely and totally overtaken by new calculations. It predicted the iPhone (Ok, and In Watermellon Sugar). I read until I figured all the characters would die at the end and then dropped it. Most of the characters do a whole lot of complaining and agonizing over their collective trauma. Bitch monkeys in other words. Also, the obligatory super gay subplot - I mean it was the 70s what do you want.
        </p>
        <p>
          So when I saw that there was a BLM art section at the De Young Museum I was sort of - yeah alright. I figure, sure, let's all do this again. It's not that the art is *bad* per se. It's just dull. Yeah, black people have it rough, tell me another one. So's everybody. Last I checked 99.999% of the human population is pure asshole, so if you find someone who isn't an asshole no matter the color you should rejoice. Should we have a woe is me Ukrainian gallery, or a Jew gallery, or a Russian gallery? What was particularly amusing was the 20 foot tall statue of a black man on top of a horse because they're *still* mad about the Confederate statues in Richmond. Good God, do we still care? For what it's worth Salvador Dali's melting clocks is 18 in by 6 in and is bordered in drift wood. I saw it at the Moma in NY one time.
        </p>
        <p>
          The gallery was a bunch of black people in repose on flowers in a sort of Romeo and Juliet psychedelic style. It reminds me of the School of Fantastic Realism which is rather funny - I doubt that the artists would like to be compared unfavorably to Austrian painters. Course, these were all self portraits and the Austrians were capable of painting something other than themselves. Also, they weren't REALLY BIG. Although if that's a matter of quality, these paintings certainly had it.
        </p>
        <p>
          Here's the picture that made my heart sing today - because I understood it for the first time - 
        </p>
        <img loading="lazy" src="static/apr4/raspberries.jpg" class="imagetag"/>
        <p>
          Today was the day my EBT benefits came in and I could eat fresh fruit and food after a month of eating meals on wheels and handout food. What this is is some guy painting raspberries because the winter is coming and it'll be 6 months until he can taste them again. Or maybe he can only get ahold of raspberries for so long because they are an exotic food. I don't care if this is a true story - for me in some desolate plain of windblasted fields there is an artist painting food so he can remember what it looks like. So his eyes might taste it again when he becomes so hungry that's all he has. That to me is art. This BLM stuff is just - it's a battle cry from the victimhood brigade. It's become unfashionable to saddle up your hordes and go thundering into enemy territory so we must always and forever declare our wars ones of defense.
        </p>
        <p>
          Come hear the tears of our battle music for this is how you paint a scream. I've heard that one before kid, skip it. Your balls just drop or what?
        </p>
        <p>
          In 2066 the star crossed lovers are the critics, always chasing their novelist ghost. And in SoZ the last of the mentats are those 'synthesists' that can aggregate disparate information and come up with the nonlinear approximation of truth, the whirling chaotic conspiracy that resolves itself into an stochastically random reality. These are warnings and parables of what not to be, but so much for false epiphanies. We are ALL synthesists - let me show you where I see things headed - 
        </p>
        <p>
          Check out - <a href="https://www.economist.com/international/2023/04/03/was-your-degree-really-worth-it">Was Your Degree Really Worth It</a> by the bastion of free laise-faire liberalist the Economist. 
        </p>
        <img loading="lazy" src="static/apr4/economist.png" class="imagetag"/>
        <p>
          Here's the take away from the article. People have found that the value of a degree in the humanities has dropped and we can now use MACHINE LEARNING to determine this is true. Hence everyone is going into economics and mathematics in order to earn the big bucks. Hate to tell you guys, I majored in mathematics and I'm currently eating cat food, so that's no guarantee of anything. That's for starters. But the bigger problem is that engineering tells you how to optimize the course of the path that society finds itself in. You want to build FASTER computers? You need to be an engineer. You want to find a way to sell your next AI startup you need to be an MBA. Maybe, if degrees are a signalling device for the worth of the human spirit.
        </p>
        <p>
          But I bet that many of you engineers and mathematicians didn't know about the connection between a novel you've never heard of and a movie you've probably seen. You don't know how our current culture is currently wired and programmed to be set on the course it is. And so we are seeing, again, is what at first approximation seems like a good thing. We have a computer! And it's telling us how to make money! Unfortunately, what this means is that we're optimizing for a corner solution where we make it BETTER FASTER HARDER STRONGER for all of our children. And the people who go into the arts and humanities are those who have nothing to lose and those who have all the money in the world. We get Black Lives wallpaper of death and flowers and Elon Musk tweeting from on high.
        </p>
        <p>
          I mean fuck it, we could go all This Side of Paradise on HAAWWWVVAADDD until we have a generation of in-bred Fitzgeralds telling us how to do our tax returns and the ghost of Hemingway attempting to burn the motherfucker to the ground. Why not?
        </p>
        <p>
          It's already obvious how AGI will be created and you just haven't read the right book yet. I hate to say it's all connected man, but it really is.
        </p>
      </div>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Mon Apr 3</span>"###,
      time: r###"<span>2:33PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>All Your Map</span>"###,
      route: r###"/april/3/233pm"###,
      paragraphs: r###"
      <div>
        <p>
          I've added a map with a couple of pictures from around the city. I plan on adding increased functionality over time (such as more information when you click on a dot, or perhaps a link to the blog post the image was first posted in). For the moment, all it is is red dots on the map you can click to see pictures by location. Progress!
        </p>
      </div> 
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sun Apr 2</span>"###,
      time: r###"<span>11:28PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>They'll Creep Up On Ya</span>"###,
      route: r###"/april/2/1128pm"###,
      paragraphs: r###"
      <div>
        <p>
          Now then. I was about to sit down and write a few words before I was rudely interupted by the psychotic homeless that smoked marijuana and drugged me again. They do this because they have nothing about themselves that is special and so they feel that they need to drag others down to their level by drugging them into submission. It's a form of torture - as if I'd be friends with torturers. Seriously? That's what they think would happen. The fuck kind of crack head logic is that. I know they're doing it on purpose, because they can go down to the alley and smoke drugs and they refuse to do so.
        </p>
        <p>
          They'll creep up on you too. I went to the library just to sit outside for 30 minutes before it opened and then they all started sitting closer and closer to me, smoking cigarettes and going AHEM AHEM when I was reading my book. Some derelict passed out at a bus stop started smoking a joint and then some fuckwit on a skateboard ran by yelling JESUS and then started schizo babbling on meth. It's like - yes, I notice they are there. I know they read this website for example. They're not stupid - they can find out my name and then creep on me on social media. And I've noticed how they respond to my posts.
        </p>
        <p>
          But you have to ask yourself - would you ever be friends with someone that drugged you? The fuck? I have - and this is absolutely true - TWICE invited homeless people into my home when they couldn't find a job or find a place. Once the guy found a place, and the other time the guy did not. They both annoyed the piss out of me, but I did it because I was bored and it was the right thing to do. I've given $50,000 to a charity for teaching kids to read in Detroit once - because I was depressed that rich assholes weren't willing to give me a job doing SOMETHING and so I figured I could at least make the world a better place by helping out those less fortunate than I.
        </p>
        <p>
          And here I am, in a homeless shelter, after being told to fuck off and die by my friends and family, being drugged every night by people that aren't willing to work on making something of themselves. It's fucking infuriating. I asked my sister if I could put up a tent in her backyard until I could find a job and my brother in-law gave me $300 to get lost. What a fucking putz.
        </p>
        <p>
          And this is not to say that the rich assholes in this two bit country haven't sown the whole thing up good and tight. I don't know if there's even a way to get a job without either knowing someone, being a kiss ass, or somehow be willing to steal. I did some construction work up in Seattle for a bit and all the electricians were high (while working on electric - fucking stupid man) and all the Mexicans were dealing coke. They didn't like me so they arranged that I work in an area with exposed nails until I tripped and shoved one through my palm. I drove my ass to the hospital and was back on the job the same day because I was sleeping in my car. But if someone doesn't like you badly enough they'll just grind you down until you quit in frustration, just to make it look like it's your fault.
        </p>
        <p>
          So here I am and the only things I have planned for tomorrow are eating from a food line (where I'll be drugged), reading at the library (which I was drugged outside of today), programming on my computer (which when I have my computer out the homeless psychopaths like to babble like the Sand People from Star Wars - while doing drugs), and oh I don't know. Jesus Fucking Christ the homeless in California are fucking vermin.
        </p>
        <p>
          I'm the kind of guy that says the loud part soft and the soft part loud. If someone's a prick - I'll say 'HEY LOOK AT THAT GUY - LOOK AT HOW MUCH OF A PRICK HE IS!' So, I don't tend to make m(any) friends. But I don't see many people who've done so little to harm others and done so much to be good to others, just on principle. Because if you need to believe in God to be a kind person you don't get it - at least for 90% of the people I see who ass their way to Church every week. Not the whole, I'm going to devote my life to being a goody two shoes, but just, I'm going to be a generally stand up guy and not steal, assault, commit adultery or go out of my way to be a prick - and occasionally be good to a couple people. And I'm being royally ass fucked by vultures that are making me high.
        </p>
        <p>
          So what I'm going to do tomorrow is take a tourist map of the city of San Francisco and start making an interactive map of where all the homeless encampments are. Because they won't stop smoking drugs around me, so now it's my problem. Which makes it everyone's problem. Once people know where the encampments are, we can start cleaning them up and putting these people in shelters, rehab, or prison.
        </p>
        <p>
          I DON'T LIKE BEING DRUGGED.
        </p>
      </div> 
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sun Apr 2</span>"###,
      time: r###"<span>11:03PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Burning Rope</span>"###,
      route: r###"/april/2/1103pm"###,
      paragraphs: r###"
      <div>
        <p>
          They're burning marijuana again at Next Door Shelter (1001 Polk Street, San Francisco California). I am being drugged against my will by psychotic homeless that are attempting to force me into having a psychiatric episode. Next Door Homeless Shelter needs to be shut down by the health department and all of the people here put in rehab or prison. Do not give money to the homeless - they will use it on drugs and then use those drugs to harm other people. Give money to the police - the homeless in the United States are not stable or self respecting enough to not harm others.
        </p>
      </div>     
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sun Apr 2</span>"###,
      time: r###"<span>5:01PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Burning Rope</span>"###,
      route: r###"/april/2/501pm"###,
      paragraphs: r###"
      <p>
        So, the markets open - as they often do - on Monday morning. There's been a great deal of consternation over the current state of the banking industry - as if this current crisis were any sort of surprise. It's always when the *stock market* turns down that everyone starts paying attention to how shit the economy is. Gee I wonder why that is. Take a look at this - 
      </p>
      <img loading="lazy" src="static/apr2/deposits.png" class="imagetag"/>
      <img loading="lazy" src="static/apr2/earnings.png" class="imagetag"/>
      <img loading="lazy" src="static/apr2/houses.png" class="imagetag"/>
      <img loading="lazy" src="static/apr2/rent.png" class="imagetag"/>
      <p>
        Once upon a time I was copy bitch in the Emerging Markets section of the Federal Reserve which makes me eminently unqualified to discuss the state of the national economy, but at this point even a coke monkey with down syndrome could see that excrement has well and truly intersected the rotary ventilator. Just ask my bunk mates over at the SF crackhouse I'm an inmate of. So. You'll notice that all of the charts are trending upward. But between 2000 and today (let's say over roughly 20 years) -  
      </p>
      <ul>
        <li>
          US deposits went up 250%
        </li>
        <li>
          Salary by people with a bachelor's degree or higher went up 60%
        </li>
        <li>
          Average sale price of homes went up 200% - 250% if we count the last two years.
        </li>
        <li>
          Rent went up 250%
        </li>
      </ul>
      <p>
        What this tells me is that salaries are the outlier. For college educated people. What we have is a bunch of old fucks that have managed to retire into wealth, and you know what old people don't do? Spend. They don't spend, they go to early bird specials and save every penny so they can pay for their healthcare expenses. And it means that the people who've managed to accumulate wealth by owning things can now afford to buy more investment properties and reinforce the cycle. If you own a bunch of properties which are appreciating and don't have to worry about food, you can afford to buy more properties and not say, spend all your money on <a href="https://www.npr.org/2022/07/02/1109105779/monthly-car-payments-record-700">car payments</a> and such. The upper class only have to compete with the salaried class - and they can outbid them on everything from electronics to car payments to ye old no fat whip cinnamon fucking lattes. And this is over 20 years! Suppose you were to knock up your girl today and have a kid. Well you're completely fucked then, because there's no way this shit show can continue. If you have a kid under 10 you're still screwed because the economy is not going to be this good in ten years. Not by a long shot.
      </p>
      <p>
        Why? Because a stable healthy economy has 300 million consumers buying cheeseburgers and flat screen tvs and fuck I don't know - the fad of the minute. Having an increasingly shrinking wealthier class of people plays hell with the velocity of money - Richy Rich can only eat so many McRibs before he explodes.
      </p>
      <p>
        As far as I can tell this rot is worldwide - fuck look at <a href="https://www.bloomberg.com/news/features/2023-03-27/swedish-housing-market-crash-exposes-economic-divisions#xj4y7vzkg">Sweden</a>.
      </p>
      <p>
        Something to consider.
      </p>
      <p>
        Course, I'm unemployed and homeless, so what do I know?
      </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sun Apr 2</span>"###,
      time: r###"<span>1:11PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Modal Activate!</span>"###,
      route: r###"/april/2/111pm"###,
      paragraphs: r###"
      <div>
        <p>
          I've added a modal that tells you how to pay me for my writing. In essence I've just told you to send me money to my paypal account and then write what you want me to talk about. I'm not adding in an automated paypal dingus because I don't want to screw it up. I've also added a skull. Personally I think it looks fuckin sick but what do I know. Write me!
        </p>
      </div>     
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sat Apr 1</span>"###,
      time: r###"<span>1:05PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>A QUIET PEACEFUL DAY</span>"###,
      route: r###"/april/1/105pm"###,
      paragraphs: r###"
      <div>
      <p>
        A quiet Saturday to read my book. I've managed some graphical updates to the site, but am feeling uninspired. More will come, as it always does.
      </p>
      </div>     
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Fri Mar 31</span>"###,
      time: r###"<span>7:40PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Full Steam Ahead Boys!</span>"###,
      route: r###"/march/31/740pm"###,
      paragraphs: r###"
      <div>
        <p>
          So, Trump is being indicted on Tuesday and that's all the press is talking about. Now, I have no dog in this fight. Do you? Do you give a shit that an ex-president fucked a whore at some point? Or something - I have no idea. I never paid attention to this controversy in the first place. But you have to ask yourself  - 'Why Now?' Remember when we were just talkine about a banking crisis that was going to sink all collective deposits worldwide and then all of a sudden this is all the newspapers are talking about? I'm not saying this is a conspiracy, but I've never seen a more convenient media shit show since Desert Fox. Sleep tight - don't worry about where your money is. I have no money so I don't have anything to worry about. No kids or a mortgage either. CNN you gotta love 'em.
        </p>
        <img loading="lazy" src="static/mar31/fuckwits.png" class="imagetag"/>
        <p>
          This piece I'm entitling 'fuckwits - just fucking fuckwits'. Life as performance art.
        </p>
        <p>
          Here's the plan. I'll become so hated and reviled all over the internet that I'll be banned everywhere except on this server and then become so absurdly popular that people will pay me millions of dollars to read my writing and this will be the only place on the internet where they'll be able to find it. The plan is coming along nicely. Full steam ahead boys!
        </p>
      </div>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Fri Mar 31</span>"###,
      time: r###"<span>9:15PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Cascading Stylized Cadillacs</span>"###,
      route: r###"/march/31/915pm"###,
      paragraphs: r###"
      <div>
        <p>
          We are building a <a href="https://www.youtube.com/watch?v=e2CliA8PuRM&ab_channel=BronxRadioLab">religion</a> and the website is coming along. While there are few truisms in life, one of the few things that I've learned is that you'll never have as cool a website <a href="https://www.lingscars.com/">as this asshole</a>. I've added a links section and an about section and I'm going to start pushing this github page for all she's worth.
        </p>
        <p>
          Meanwhile, this <a href="https://news.ycombinator.com">news aggregation website</a> has royally pissed me off. You know what these cunts are doing? They're shadow banning users and not telling them that they're doing it. Who the fuck does that? What is this, the Mickey Mouse show?
        </p>
        <img loading="lazy" src="static/mar31/pic1.jpg" class="imagetag"/>
        <p>
          The reason that they're doing this is fairly obvious. They have no other way of keeping out the trolls. The website is free, and so the only way to keep people from creating another account and spamming the board with bullshit is to ban them and not tell them they're banned. So, they can't monkey code a solution so they act like little bitches. I mean, fair play, their website. But aren't they supposed to be good at this? The fuck? Baffles the mind it does.
        </p>
        <img loading="lazy" src="static/mar31/code1.png" class="imagetag"/>
        <p>
          Anyway, it also means that hackerNews website is filled with spammy trash. LIKE THIS POLITICAL PARTY. WHITE PEOPLE AI DRIVES LIKE THIS AND BLACK PEOPLE AI DRIVES LIKE THIS. Smegma. Total smegma. So here's 25 lines of code that uses chatGPT to show you the most relevant articles based on their titles, so you don't have to bother reading the site. We are the music makers and we are automating the automaters. I'll leave it as an exercise for the reader to add user input prompts. Integrate it into your site! Tell your friends!
        </p>
        <p>
          I like the concept of automating the automaters. It works in so many ways. Take <a href="https://ohadravid.github.io/posts/2023-03-rusty-python/">this guy</a> for example. What he's doing is he's taking a bunch of python code, looking for the places where that code is inefficient and then replacing the inefficient bits with Rust. Because Rust is a low level language it makes the code faster. What they do is that they look at something called <a href="https://www.polarsignals.com/blog/posts/2023/03/28/how-to-read-icicle-and-flame-graphs/">flame charts</a>. This is just a fancy visualization that will look at the dependencies of what code calls what code calls what code...to find the bit in the nth nested dependency that is slowing down the program. It looks like this - 
        </p>

        <img loading="lazy" src="static/mar31/flame.png" class="imagetag"/>

        <p>
          But think about this for a minute. This is a two step process. First, you look at a graph, then you find the code that's inefficient and replace it with more efficient code in another language. Can't LLM models already do that? This looks like an entire cottage industry that are getting paid the big bucks to do what someone could program once and be done with it. Also they're mapping polygons to vectors. In my heart they're curing cancer, but in reality it's probably another video game where you can <a href="https://www.gamespot.com/articles/saints-row-iv-developers-respond-to-dildo-weapon-criticisms/1100-6412418/">beat someone to death with a dildo"}</a>{". To each their own I suppose.
        </p>
        <p>
          I hate to say it, but the one thing that an LLM model can't do is live authentically. It can't create the alpha of lived experience. I've written about this before - I won't post the link because it's a long and boring article that no one who've I've ever shown has bothered to read or could contribute. What LLM models can do is they can create other people's work and they can create like works. You want a picture that's 'Man Getting Hit In The Balls (but like Picasso)?' Bam! Done! Unfortunately, soon the internet will be filled (if it isn't already) with Picasso Balls and LLM models will start eating their own shit. That is they won't have any more training data to train on, because they've fax machine copied the internet to death. Unless the AI companies start downloading driverless car traffic sensor data - I wouldn't past the dogfuckers. Point being, the people who will rule the future will be the experientialists, those who can create new alpha and know how to live. Unfortunately for all of us poor bastards that see the guys that had the cushy button pushing jobs and thought - damn, I wish I had that. Instead we're all going to have to do those jobs that machines can't (the shit hard jobs like security guard, grocery store worker, garbageman) until those jobs are automate away. The people who will make money will be a shrinking minority of poets and artists that can contribute the alpha to keep the future going. I mean, where the fuck is the money going to come from?              
        </p>
        <p>
          <a href="https://www.vanityfair.com/news/2017/06/neal-stephenson-metaverse-snow-crash-silicon-valley-virtual-reality">In the future we are all gargoyles</a>
        </p>
      </div>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Thur Mar 30</span>"###,
      time: r###"<span>2:27PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>X-TRA!</span>"###,
      route: r###"/march/30/227pm"###,
      paragraphs: r###"
      <p>
        THIS JUST IN - GOOGLE BING CHAT THINKS GAY PEOPLE ARE OBNOXIOUS*. *either that or rainbows.
      </p>
      <img loading="lazy" src="static/mar30/pic11.png" class="imagetag"/>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Thur Mar 30</span>"###,
      time: r###"<span>11:20AM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Dog Fuckers At The Dildo Bazaar</span>"###,
      route: r###"/march/30/1120am"###,
      paragraphs: r###"
      <p>
        So the dog fuckers in the shelter started speaking bullshit glossolalia about drugs and bullshit for an hour and then tried to light up meth. I told them to go fuck themselves and then they said 'OH MASSAH ME SO SAWWEE. I APPRECIATE YOU.' Shitbricks. So I decided to go around the ghetto this morning taking pictures of all the crazy shit that pisses me off and no one is fixing. There's a failure of the imagination in a writer who makes himself the story, but I don't care. So I'm not imaginative. I'm not that <a href="https://www.wired.com/story/brandon-sanderson-is-your-god/">dipshit with a sword</a>. I just have a keyboard a camera and a pair of brass balls. Let me show you something that will make you angry. Something that might make someone give a shit for 5 minutes.
      </p>
      <img loading="lazy" src="static/mar30/pic1.jpg" class="imagetag"/>
      <p>
        I title this piece <a href="https://www.youtube.com/watch?v=5VtDM5jicRQ">today was a good day"</a>.
      </p>
      <img loading="lazy" src="static/mar30/pic2.png" class="imagetag"/>
      <p>
        Yeah - you see that tent in front of the 'Hot Zone' dildo bazaar? That fuckwit likes to smoke meth when people walk by - and because the owners won't clean the sidewalk nothing is done about it. The cops can't do anything about moving this person from the premises. <a href="https://sfgov.org/mod/sites/default/files/FileCenter/Documents/2176-Sidewalk%20Landscape%20with%20SIRP.pdf"> According to Article 15, Section 706 of
        the Public Works Code</a> it's the responsibility of the owner of a premises to clean their own sidewalk. So that means that people can't walk on this side of the street or this shitstain will smoke meth and dose them. In a sane world, this person needs to be in a federal penitentiary for the rest of their natural lives for hundreds of cases of assault with a deadly weapon. But yeah, you're right CNN - it's all the fault of <a href="https://www.youtube.com/watch?v=9nWAic0lHVI&embeds_euri=https%3A%2F%2Fwww.bing.com%2F&embeds_origin=https%3A%2F%2Fwww.bing.com&feature=emb_logo&ab_channel=CNN">this guy</a>. I'd call you fuckwits, but 'journalist' is a dirtier word around here.
      </p>
      <img loading="lazy" src="static/mar30/pic3.png" class="imagetag"/>
      <p>
        People dealing crack in front of a market where the owner has either been dosed into submission, or is getting a kickback because the dealers slip him money. Not safe to walk past here.
      </p>
      <img loading="lazy" src="static/mar30/pic4.png" class="imagetag"/>
      <p>
        This is what it looks like when the cartels decide to take over a sidewalk.
      </p>
      <img loading="lazy" src="static/mar30/pic5.png" class="imagetag"/>
      <p>
        Dealers on the corner.
      </p>
      <img loading="lazy" src="static/mar30/pic6.png" class="imagetag"/>
      <p>
        Well shit on me! I didn't know there was an a capella cover of <a href="https://www.youtube.com/watch?v=3L4YrGaR8E4&ab_channel=RATMVEVO">Bulls on Parade</a>. Shit, give me 800 million dollars and I can sit on my ass all day doing nothing too!
      </p>
      <img loading="lazy" src="static/mar30/pic7.png" class="imagetag"/>
      <p>
        <a href="https://wilwheaton.net/2023/03/the-library-is-a-safe-place/">That guy from Star Trek"}</a>{" tells me that the library is a safe place. That's why there's suicide netting around the balconies. Don't worry - they're replacing it with plexiglass soon, so it'll be more aesthetically pleasing. In San Francisco, the only thing that's illegal is killing yourself in a way that's too sudden and messy. Too gauche. Too outré. We prefer a long slow slide into oblivion around here.
      </p>
      <img loading="lazy" src="static/mar30/pic8.png" class="imagetag"/>
      <p>
        They put up speakers in the capital section of the Tenderloin to play shit music to drive the homeless away from the area. Because policing is harder than putting up speakers that will simply drive someone mad who wants to lie down to rest. Good fucking job. Or maybe the people here like ABBA and Ace of Base on repeat. Who fucking knows.
      </p>
      <img loading="lazy" src="static/mar30/pic9.png" class="imagetag"/>
      <p>
        The landlords near market street with the classical looking buildings can afford to pay some asshole to clean the sidewalks.
      </p>
      <img loading="lazy" src="static/mar30/pic10.png" class="imagetag"/>
      <p>
        OH LOOK IT'S A TROLLEY. We used to ride those to work.
      </p>
      <p>
        This is Peter Teller Weyand telling the lot of you that you have no fucking idea what you're doing.
      </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Wed Mar 29</span>"###,
      time: r###"<span>7:03PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Do My Ears Deceive Me</span>"###,
      route: r###"/march/29/703pm"###,
      paragraphs: r###"
      <p>
        So the crazy people in the shelter are attempting to force me into religious conversion. What this person will do is they'll talk to themselves repeatedly while they play music and flood the shelter with drugs. I started recording after this dogfucker just repeated to himself 'you can be reborn. REBORN!'
      </p>
        I don't want to lay down in the gutter and shoot up with heroin so I must be a born again Christian. I don't know either - crack head logic, what do you want.
      <p>
        Then he got mad when he saw that I had my computer out and started muttering about how he had been 'killed a thousand times' and some other crazy shit. This is what forced religious conversion looks like, by drugging someone repeatedly over and over again. This is at Next Door Shelter, 1001 Polk Street. I am being drugged against my will by people that want me to convert to their crazy bullshit religion.
      </p>
      <p>
        Now with audio!
      </p>
      <audio controls=true>
        <source src="static/crazypeople2.m4a" type="audio/mpeg" />
        "Your browser does not support the audio element."
      </audio>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Wed Mar 29</span>"###,
      time: r###"<span>6:31PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>I Warm Comfortable Horse Fuck Against The Wall</span>"###,
      route: r###"/march/29/631pm"###,
      paragraphs: r###"
      <p>
        Speaking of horse fuckers - just remember <a href="https://www.youtube.com/watch?v=7Pq-S557XQU&ab_channel=CGPGrey">Humans Need Not Apply</a> was written in 2015. It's not like we haven't seen the AI revolution writing on the wall for eight years and no one did a damn thing to make it so people could find stable long term employment that would pay the rent, let you put your kids through school, let alone have enough to eat or not be scared of your retirement plans being 'busking in the street' or 'winning the lottery'. You're welcome.
      </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Wed Mar 29</span>"###,
      time: r###"<span>5:39PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Your Pretty Is Pretty Ugly</span>"###,
      route: r###"/march/29/539pm"###,
      paragraphs: r###"
      <p>
        I've added a picture that I put into Adobe Firefly - I put in the prompt 'The sky over the port was the color of a dead television'. That prompt has been put in so many times that it only came back with this kind of thing. Proving that once again, people have no fucking imagination although it is 'pretty'. If Case could see us now he'd cut his own dick off with a rust pair of pliers. I'll continue to add more to this website and call everyone on the planet a horsefucker.
      </p>
      "###.to_string()
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Wed Mar 29</span>"###,
      time: r###"<span>5:20PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>TESTIFY go fuck yourself</span>"###,
      route: r###"/march/29/520pm"###,
      paragraphs: r###"
      <p>
        I have no money and I'm a homeless bum. I fucking hate the homeless, HOLY FUCKING SHIT DO I HATE THOSE FUCKERS. I'm going to spend the rest of my time developing this website. It's written in yew.rs, which means that I can be horribly inefficient. The downside is that I don't have the money to host a backend so I can't do much on the front. FUCK. In any case I'm going to make this the most rad kickass site I can given that I can't store anything in a database. Fuck you.
      </p>
      "###.to_string()
    }
  );

  returnblogs
}