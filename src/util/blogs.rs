
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

  returnblogs.push(
    Blog{
      date: r###"<span>Sun Apr 16</span>"###,
      time: r###"<span>6:19PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>I Have the Right to Program Without Being Drugged</span>"###,
      paragraphs: r###"
        <p>
          What it says on the tin. I have the right to program without being drugged or I will continue to inform on everyone here at the shelter. I was threatened twice today - once with physcial violence because I told someone to stop talking to me when I didn't want to share the time of day, and the second time that I 'should' be getting high. Fuck you and the horse you rode in on, what I do or don't do with my body isn't any of your fucking business. It is my business if your getting high AFFECTS ME. I was drugged again, at the fucking library of all places, and had to go to the hospital. The cops were completely ineffectual. 'Oh, someone must have been partying and got it on their clothes.' Oh, OK. It's completely copacetic for someone to have drugs on their clothing that can kill someone or make them ill when they walk by. 0.2mg of fentanyl can kill someone and these people are wandering around the public library because we treat it as a daycare center for criminals rather than, you know, <a href="https://www.joannejacobs.com/post/algebra-for-none-fails-in-san-francisco">a place of learning.</a> In the meantime, <a href="https://www.cnn.com/2023/04/16/us/dadeville-alabama-mass-shooting/index.html">this happened.</a> Ah, business as usual I said to myself and spent the rest of the day sleeping the sleep of the just. I will continue to write and call you all bastards until I die.
        </p>
      "###
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sat Apr 15</span>"###,
      time: r###"<span>12:53PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>It's All Just Kabuki</span>"###,
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
      "###
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Fri Apr 14</span>"###,
      time: r###"<span>8:59PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Little Druggy Dipshits Don't Get A Vote</span>"###,
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
      "###
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Thur Apr 13</span>"###,
      time: r###"<span>10:22PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>San Francisco Crime in The Tenderloin</span>"###,
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
      "###
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Wed Apr 12</span>"###,
      time: r###"<span>8:40PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>I Don't Even Have to Be Wrong - It's A Game-Changer!</span>"###,
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
      <p style="font-weight: bold;">Have a nice fuckin day 🤮"}</p>
      "###
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Tues Apr 11</span>"###,
      time: r###"<span>11:59PM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>I'm So Lucky!</span>"###,
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
      "###
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Mon Apr 10</span>"###,
      time: r###"<span>10:11AM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>JUMP, YOU FUCKERS!</span>"###,
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
      "###
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Sun Apr 9</span>"###,
      time: r###"<span>10:23AM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>Killing Bob Lee, Maiming Don Carmignani</span>"###,
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
      "###
    }
  );

  returnblogs.push(
    Blog{
      date: r###"<span>Mon Apr 8</span>"###,
      time: r###"<span>7:50AM</span>"###,
      location: r###"<span>San Francisco</span>"###,
      title: r###"<span>So There Was This Guy...</span>"###,
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
      "###
    }
  );

  returnblogs
}