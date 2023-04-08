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
      bookid: "1pw4eLe2ZDXYIgFXJYbXz3buEKnMWPaK0".to_string(),
      chartdesc: "20th Century Lit 1900-1924".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1xcncO0T0K2C4ZCvpoFdtuFdtC-aWV1rV".to_string(),
      chartdesc: "20th Century Lit 1925-1949".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "133E5AqAfTf4dbU8_GAVwwNEyed2lBKVa".to_string(),
      chartdesc: "20th Century Lit 1950-1974".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "171VTiESQm_zmv9PYF75m_Q20H7n-vFIU".to_string(),
      chartdesc: "20th Century Lit 1976-1999".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1_hDDshqVglR-PYcKs8YQpX7DYv9L33hO".to_string(),
      chartdesc: "21st Century Lit".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1CvhxjsRoQsNDahZ-unfWsOtHNQYUpUGN".to_string(),
      chartdesc: "21st Century".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Wf5t4geJ4R0p-wJlrX-WWjHx2UCpgGY6".to_string(),
      chartdesc: "55 Books Under 200 Pages".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "180ZhmyM3BFFHVspDLvQLRurJZF4-uJzm".to_string(),
      chartdesc: "100 Novellas".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1P0ZXsd3MUy_8lzwV_ZW-8bd6YSre5fnf".to_string(),
      chartdesc: "Absurdism".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1-54j0ilXTkMQvs_LHc2ydlRBlt-o1ZHC".to_string(),
      chartdesc: "American Literature, A Visual Journey".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1FmAsVJ9uPCTYYjxkLBa43BGVEbmggdFF".to_string(),
      chartdesc: "Aphoristic".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1le4v-nHy7vlTffwknHNJg-R_YxkorazH".to_string(),
      chartdesc: "Author Accessibility Depth Chart".to_string(),
      kind: "Literature".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1Tbag4TCzT_aEe5vrcVCaM3L-p8ILiGWr".to_string(),
      chartdesc: "Before You Read Ulysses".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Lwo4yloA1VPJBtAp_hCREA_cCcCxVd7H".to_string(),
      chartdesc: "Beginner's Guide".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Vqchy3EEGeeluUJ5PDlbk2jYToyyYqzk".to_string(),
      chartdesc: "Beowulf".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1qJdKFVxYqBU8s9l7M80zjQmYYmZ4XPdV".to_string(),
      chartdesc: "Blooming, Loving, Life-Affirming".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1vEcFhihZfO_irsfAY73A07u9rbxVUfvv".to_string(),
      chartdesc: "Book Cover Colors".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1-oMVAUsEQG8cYwT9WbQOtxrcUm5Ui3Mi".to_string(),
      chartdesc: "Books About Books".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1FTHmtbnqdnRPfoNuIoz3cAGmnxQk_Ona".to_string(),
      chartdesc: "Books Which Make You Happier".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1JWbwKLilja2t3c21u_dM_GzVRpbULVAn".to_string(),
      chartdesc: "Books, Plays & Epic Poems".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1qeWisqOCW6XIg9Ebms2vEyAqbPwAodlo".to_string(),
      chartdesc: "Canon 2".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1jJVtqUAV0t9XXFdysHAIobZby0r6qwli".to_string(),
      chartdesc: "Canon".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1mxpQIMB_DVN7oCRH7KDgQh19UMRnm9EL".to_string(),
      chartdesc: "Classic Fiction & Modern Classics".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1cj7aYyCGJb4flfQCP1dZOguyiZM7wMlV".to_string(),
      chartdesc: "Dante Translations".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "10B_Q7F8jFmonrAbTuSmQ1lkMAu6HMMG2".to_string(),
      chartdesc: "Depressing".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1wxcCZZ0fz5xnYpQEtnpKKjVaVisCENi3".to_string(),
      chartdesc: "Doorstoppers".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "19JC0xYa1yVQq_UUvO3YLlxJ_oLWGgz5D".to_string(),
      chartdesc: "Dostoyevsky Translations Comparisons".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1cuUKlhTic1II4U5o6XLPchVlkZwR3BM7".to_string(),
      chartdesc: "Downers".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1V4BIjd-qPN7qsS5KSNKqkN9NPaXP22wE".to_string(),
      chartdesc: "English Poetry Guide".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1nkBFu3BjGtF7c5KS-mtyDm6c_paTU578".to_string(),
      chartdesc: "Entry Lit".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1GcUfyKRgWeDKC37_LDu1BxXWeN7iYPcf".to_string(),
      chartdesc: "Epic Poetry".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1UYihY7tEeDDZY5ZHrWtGitbWcXyIT8TL".to_string(),
      chartdesc: "Erotica".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1OJ79kFNm25-gpPGdL9W7f6bMMvKX5mUr".to_string(),
      chartdesc: "Essays".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1kWLiuCbCsgpWDgkEHDaHh2xW1hDBaWxX".to_string(),
      chartdesc: "Female Writers".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "18TU21xdn2Lmp6eUhy0STaiOIVdvClYAX".to_string(),
      chartdesc: "Flash Fiction & Short Stories".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1E_B7rN_-ev6v6j4Kfho5chEmVg8d5g03".to_string(),
      chartdesc: "Foundational Texts For Understanding Western Literature".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1uVLpZlKaSBrpEycAJwLd7xgU4u0xCaA8".to_string(),
      chartdesc: "Good Contemporary Lit 2014-2016".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1AoaJGu_SZeBaobRJgRmFpzIk75n5ygUe".to_string(),
      chartdesc: "Gothic".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1q_Eh7Y4t2TBJQb1ioM_B3pwOqTLVvXd-".to_string(),
      chartdesc: "Guide To Western Literature".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1v2ztlxr-q4vvd8sVVaprw4ul1BndqfGZ".to_string(),
      chartdesc: "Hard Books".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1xrqyTDiKzlukNORlJbOQ3qi5SVbNJEjQ".to_string(),
      chartdesc: "High Romance".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1t9mKZqRrDIEiQDrcvXlG1ehwya_iVkTY".to_string(),
      chartdesc: "History As Literature".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1jp0685HHMD_NpIb1sAGqLtnVkE90kpGE".to_string(),
      chartdesc: "Homo-Core".to_string(),
      kind: "Literature".to_string()
    }
  );
  

  bookidsvec.push(
    BookIDS{
      bookid: "1Rc7iP_FkVJwSjARU2T9-hBytE7kMQTAi".to_string(),
      chartdesc: "Homosexuality".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1ZQd689TULjb82sQk1bHX6PyNs0vBfUsR".to_string(),
      chartdesc: "Honourable Mentions".to_string(),
      kind: "Literature".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "19EbHT3b7uk24Z2BFf8KHQ4BspJa9wOI4".to_string(),
      chartdesc: "Humor".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1njcY5C_PZtZnhInDg-xHc2_l2T7r_HD4".to_string(),
      chartdesc: "Japanese Women Diaries & Memoirs".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1kO8IDTkp32xesJQOLrd6RMm3FsBDVlNV".to_string(),
      chartdesc: "lit Canon".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1dENfjtzCOV_81qA-WkoB-pirtAU99JKP".to_string(),
      chartdesc: "Lit Core Essentials".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1mA9SGFVEYjfiJIntRRdkwCaIcXeoSslK".to_string(),
      chartdesc: "Lit Core Exit Level v1.6".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1kb0KqrsiV58aZpkwQR6eJwYd8aMhpvyp".to_string(),
      chartdesc: "Lucifer".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1w2DKEGXjaLZfiu01-6RF3Jmw-hICT1rL".to_string(),
      chartdesc: "Maximalism".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1ivvo6EAZbJRlnVCgKHRQ-c4GWoDOSVgY".to_string(),
      chartdesc: "Medieval".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1647qs8t86jNYunZRVNGr2eVeTlZ4m8RI".to_string(),
      chartdesc: "Modernist Literature, Lit Guide To".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1232H8G6jOobV6ddodHJ_VeLgnl84nQFt".to_string(),
      chartdesc: "Must Read".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1c1v4cdXrL2W0hsIAYCJoDuQt3NF42MeE".to_string(),
      chartdesc: "Must Read 21st Century Books Revised Edition".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1nZgA5O0MGwnHLo3F_dWlVpQEcaMd9G7Z".to_string(),
      chartdesc: "New York Review of Books".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1b2CZCwKO1qjq_GQ1OE6no4sni89BSZ0W".to_string(),
      chartdesc: "Older Women Younger Men v4".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1XC7aWeGNT_g842b__EGJbd4lvVTTPTZy".to_string(),
      chartdesc: "Patrician Core".to_string(),
      kind: "Literature".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1Gn92eajXvkNnB8GBYQwIhb72ruZ_s_Ua".to_string(),
      chartdesc: "Pedophile & Hebephile Reading Guide".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1B1fjXdLV2u9N2LeYbPz14fZzphPA5a-D".to_string(),
      chartdesc: "Picaresques".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1sv_zDCB6w5g5tSJxBVJJ26av6KMeSa4s".to_string(),
      chartdesc: "Poems, The Epic".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1tYhbdeIOmWQD6BHM2_4JeIB8u8eDlz5R".to_string(),
      chartdesc: "Poetry Introduction".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1uKep77KyqxKNfMSLyPDC3k1IMtZPvqcK".to_string(),
      chartdesc: "Poetry Tier".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Kg-fpOBENvHJPNo-05JXJsdAXJKpjbgG".to_string(),
      chartdesc: "Poetry".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1YiXgqvTzYFUcYg7iFqbtMJjZWDdEywj4".to_string(),
      chartdesc: "Postmodern, Recommended".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1gAjBwak0yijv52efIxetF4A6BDMAvftH".to_string(),
      chartdesc: "Postmodernism".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1DmzBLCle_PoyXVuAiVC5Mz8PcguA8fUK".to_string(),
      chartdesc: "Pre-20th Century and Modern Classics".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1ASmc4K-APXnkcjqwkLEUIikn3QKhHhLv".to_string(),
      chartdesc: "Recommended Novellas".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1oSZYN6ITMwUCdyPFJAnxPBmFZaB2_t2r".to_string(),
      chartdesc: "Recommended Short Stories".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1jisCtm4Vv7YxLhtiywFi7gAuRBnGn2WQ".to_string(),
      chartdesc: "Scaruffi's Best Novels of the 21st Century".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1cdts_qXy7dmTwiI8fnuyIuDQ0rVOb6e0".to_string(),
      chartdesc: "Short Story Collections 19th Century And Earlier".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Gyzwfxk2Y_qN8ZvwYjxolcJs1HqHIpVc".to_string(),
      chartdesc: "Short Story Collections 1900 - 1940".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "11OtAC1nmok5hOkzQD6m1i_lE2Ph7mSSj".to_string(),
      chartdesc: "Starter Kit 2".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1sgg-EBDDmq6qVb2Xm14Fp_Yogzq6fF4b".to_string(),
      chartdesc: "Starter Kit 3".to_string(),
      kind: "Literature".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1F-ffUzLrOqhuCzxhBtYP_eX9DVFGjTGN".to_string(),
      chartdesc: "Starter Kit Second Edition".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1imVIzrGzWcDbH1cu2KTnJzczjvzi1CHj".to_string(),
      chartdesc: "Starter Kit".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1RatalbA_yallDCLu2890Y3uTVurCb-HR".to_string(),
      chartdesc: "Surreal Exploration".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1iiDZ3oSVGh7m3WlN90BVnOAzy-ZsLDqo".to_string(),
      chartdesc: "Surreal Literature".to_string(),
      kind: "Literature".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "10UPWttmla09piz8uuAG-1_wF0YO-C3j5".to_string(),
      chartdesc: "Theater Plays, Top 100".to_string(),
      kind: "Literature".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1wNAN6gi5azBWnSN5n0Om9pWQC93Kx8o4".to_string(),
      chartdesc: "Theatre & Drama, Part 1".to_string(),
      kind: "Literature".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1kOwbi4Fid2EjVcmjy-3GC9Bg1LPVo4sY".to_string(),
      chartdesc: "Theatre & Drama, Part 2".to_string(),
      kind: "Literature".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1yvhoD0cpdE-ScIcPVSglu1Ezy95Xskf4".to_string(),
      chartdesc: "Top 10 Fiction".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1u8alG1wno6E3hW-Fy_ma3htqSmkvndtH".to_string(),
      chartdesc: "Top 20 Plays".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1rXDWbhiSTnpBRhSaLu2Hzs1AW9KQfku7".to_string(),
      chartdesc: "Top 30 Poems".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1IOXmuA_7AUPECj7JzkPux2W_Dd5bYq9g".to_string(),
      chartdesc: "Top 40 Short Stories".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1i-bp-imichdkd209l7GtUbpUI9N81qN_".to_string(),
      chartdesc: "Top 50 Non Fiction Books".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1YbqHVDmaTXN021_180p_FHGwY0WQz-Vs".to_string(),
      chartdesc: "Translations, A Basic Guide".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1NDrA1ZrmtIDo6De4fYuPzX4JjrxeVCnG".to_string(),
      chartdesc: "Travel".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1lqUfUZ1FI9cvzfv0agEmKk54QcDEni4w".to_string(),
      chartdesc: "Tryhard Namedrops".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1iATQnMpWxP3gqlYTyNSMVf62lMueHSpP".to_string(),
      chartdesc: "Tuberculosis Sanitarium".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1vn6OkKGZTB6Dhd7vqCi0vDkp5A_onDqB".to_string(),
      chartdesc: "Ulysses2".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1LD0pCK6Ob48LPmY7MytqwBjE0HP2d4eP".to_string(),
      chartdesc: "Verse".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1ry9lxtuwgh3l8Z9_C-8rih0dITgr0d-R".to_string(),
      chartdesc: "War Literature".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1eVu7kEFB2Sz-nwmkfGGpUS-p4-sTpnzZ".to_string(),
      chartdesc: "What To Read Tier List".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Glkdc8l8zPon04gB3hhOPOlN9TYvxKhI".to_string(),
      chartdesc: "Young Artist".to_string(),
      kind: "Literature".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1Glkdc8l8zPon04gB3hhOPOlN9TYvxKhI".to_string(),
      chartdesc: "Young Artist".to_string(),
      kind: "Literature".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1qAHo95MJlQIVgR9c5ZUZCq3i-THZ-fQv".to_string(),
      chartdesc: "12".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1a3UDdkLn5LKCt2ph81H7xVxfkuDALjij".to_string(),
      chartdesc: "Analytic Metaphysics, Intro to".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "16jozJKXCQoRDW1DELnj2o2dmU1sB1voZ".to_string(),
      chartdesc: "Analytic Philosophy & Philosophy of Mind".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1rkK85Nx2_sqsi5Gs0Qvjw-4jqgbdSBOU".to_string(),
      chartdesc: "Analytic Philosophy 2-1".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1u04eIq8wXa7VmpO0VVNmTp4RESFrSGYw".to_string(),
      chartdesc: "Analytic Philosophy 2-2".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "11TeiTl8IJ1zZ6itm04JKvM9uZ47M7HB4".to_string(),
      chartdesc: "Analytic Philosophy 2-3".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "12qrYL0PuGaZlsVlkjaeMz-OyufJtdfai".to_string(),
      chartdesc: "Analytic Philosophy 2-4".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1n5azPC9wOZ7HP0JB3dhjkU4wCoC9Vpyn".to_string(),
      chartdesc: "Analytic Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1TH82I7B8lKwhf06I0LkGjI5NHg1ApgBV".to_string(),
      chartdesc: "Ancient Western Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1fM658_ZZQUQUde6HFANkgR4ZcKDSGVja".to_string(),
      chartdesc: "Baudrillard, Jean".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "15yeoCbRykHTsLm5k9A7oMOi_HARJtgaC".to_string(),
      chartdesc: "Christian, Catholic, & Medieval Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "18Xt0F6JpCOybJALpUBmZAsqkf4gy5wwo".to_string(),
      chartdesc: "Early Existentialism & Modern Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1jvmRhkD8lU0jVdh7KQXbisSqDZmJuHDY".to_string(),
      chartdesc: "Eastern Mystic".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1PjzEWeAv4c_2YuWhsBH8Hbkrkz39DnZp".to_string(),
      chartdesc: "Ecophilosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1EXRPWSzzdEFSfxIWZOcybdRnz4m_zDpi".to_string(),
      chartdesc: "Egoism Reading List".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1MSRPrOb35FtQeIunxgXvgu2deB2qgMDN".to_string(),
      chartdesc: "Existential Crisis".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1HDbFCRO2K72yx64icpTIchDxhY2uBXKT".to_string(),
      chartdesc: "Existentialism Course".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "18gXIqHNxRKRWioCIbMQF7ImXu4VpyrMM".to_string(),
      chartdesc: "Feminist & Queer Theory".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1FT2h-QFCMQEzglVihLfW-opkpdKtkrjQ".to_string(),
      chartdesc: "German Idealism".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Ruu3TIGOz0bgzYJlNReG9m6OgqPm3lp-".to_string(),
      chartdesc: "Get Your Shit Straight Starter Pack".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "12Py5oBm5HxIX3YfT0YfHraPtmDVfx2vg".to_string(),
      chartdesc: "History of Western Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "135fyCPZvEGn84JIyqQ3gWcoQG7wUAHB9".to_string(),
      chartdesc: "How To Get Into Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1ed-0iD-sw_SDQP8acCzjDhcnqtaH7M3p".to_string(),
      chartdesc: "Kant, Understanding Without Secondary".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1us7BGCEIduNCr2YAskmcPdO4iTcsRP4J".to_string(),
      chartdesc: "Modern Philosophy 2".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1bSDA4G7Bb--2eqITWjgeWWAAyfK6oQYn".to_string(),
      chartdesc: "Modern Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1I-c-mJHcwsq7G0Hze--P003QX0R-4MKI".to_string(),
      chartdesc: "Nietzschean".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1yGkZS-1rDZSYkqCBPm05h5R_-2zjWQhf".to_string(),
      chartdesc: "Overview".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "13_PEnmuNsVQKFaI4GKFDflTw2YrmU8k1".to_string(),
      chartdesc: "Phataphysics".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1bKYMEbp9OR3PNYJpeENE5IKwAHdyCwCJ".to_string(),
      chartdesc: "Philosophy Youtuber Intro Guide".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Ino-fktsQ27tftI7XQ5euSN2yrpqskOG".to_string(),
      chartdesc: "Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1qFgAKkHbrLKIkboHOHA96pIBxufyLUTF".to_string(),
      chartdesc: "Philosophy Stack".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1wHLxP8TehWraBEiH390gxcRwXsFtITVK".to_string(),
      chartdesc: "Political Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "13CgJMtw1_YKIOBBKkEkiEfT1RKg9zTW-".to_string(),
      chartdesc: "Political Theory".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1p0NbU9dphkZWQtifHkshd7A78zB5cBkd".to_string(),
      chartdesc: "Postmodernism".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "151W6eHHDihEnd9EKTjB7MPhUhPHsemWm".to_string(),
      chartdesc: "Pragmatism".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1QD0CJ1PZICOugHxFa9SVeGIUtNyh89rS".to_string(),
      chartdesc: "So You Want To Get Into Philosophy".to_string(),
      kind: "Philosophy".to_string()
    }
  );
  
  bookidsvec.push(
    BookIDS{
      bookid: "1HfDj4rFz4f2HooAL7kjo1HbvHX4MgCKx".to_string(),
      chartdesc: "Stages".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1zgpf7men91BbxXWq1qmGStwJOC8DiPCR".to_string(),
      chartdesc: "Stoicism Introduction".to_string(),
      kind: "Philosophy".to_string()
    }
  );
  
  bookidsvec.push(
    BookIDS{
      bookid: "12EVfpf4w6MM9d9qR5NDln9f7YQ9fNjau".to_string(),
      chartdesc: "Tetralogies".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Zir-j8fbb0pmqaFmjuMqNX1guug9y5Wc".to_string(),
      chartdesc: "The Scientific Revolution".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Z75RiX0J5GGmpm_BmAd26-IhurJBndcb".to_string(),
      chartdesc: "Utopianism".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "13fQjYAvwQfFccthtn7E7BQGurGuqxkN7".to_string(),
      chartdesc: "Various".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "11FXw_gIlqwseTnrtFdXZ8hzUu56UavrO".to_string(),
      chartdesc: "Wittgenstein v2.3".to_string(),
      kind: "Philosophy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Bf6LrZjcyvF_jzTeOV_AX15gcYFszjHm".to_string(),
      chartdesc: "Zubiri".to_string(),
      kind: "Philosophy".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1TkNjSTbbA3bBLs3x9_4F2rs4YLzPDqsl".to_string(),
      chartdesc: "A Redpilling Guide To the Jewish Question".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1JfW5oWY5PWd7MvXRTMrc0UNKVFtbuusp".to_string(),
      chartdesc: "Aus Pol List".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1TAgGB0LZROxTRIy0v6ci9ZwkIy3k2g_C".to_string(),
      chartdesc: "Bronze Age Pervert Reading List".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1jJHLzIQ_UA4TuMLUYIjh2OofBaiPlmEJ".to_string(),
      chartdesc: "Canadian History".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1dmhdSaH3___gxylpwp-NG-eYUKNjJqob".to_string(),
      chartdesc: "Communism & Jews".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "18UxCf6Qrnl3EQKv-5xjxXjx0VKr6L7rw".to_string(),
      chartdesc: "Conservatism American Campus".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "12L2aYOimz8z1mRAYTsguqO6iY0LccgLe".to_string(),
      chartdesc: "Cultural Marxism".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1eNT9hCxJRUIRqj0kn1Zl-RjK9lC1Ima8".to_string(),
      chartdesc: "Evola".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1V9lKGYwa0JkiMyry-3YwJPpq5BSD39xf".to_string(),
      chartdesc: "G Pol X".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1HYHIZCoJbyclGdLJa_OUhDHu0ORtxJ89".to_string(),
      chartdesc: "Hitler".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "19zqmzNLbWAVQ5bCaQjHdsShgOY8EOefI".to_string(),
      chartdesc: "If Only You Knew How Bad Things Really Are".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1hXxTIV4A5QStfWeYyFPxBDbgf5Cp8RqS".to_string(),
      chartdesc: "Jewish Criticism of Jews".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1qX8cGZIWPRovVOJU10rMYwfUsU9VDTA8".to_string(),
      chartdesc: "Jewish Influence In America".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "11ZfABM67gz7YYhmU7ZK6kBnsr4HqJBJ_".to_string(),
      chartdesc: "Jewish Question List".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1OwLGZd4NOEvpEXuZf4RheOQh90PNr8Kr".to_string(),
      chartdesc: "Jewish Question".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1cJnLM6Xocaq4ya_GC3aMNvXuc8AyHrnD".to_string(),
      chartdesc: "Jones, Michael".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "17tFlZ6GfsmrUSqMEReEt5U9YGvmy4WrA".to_string(),
      chartdesc: "Judaism Critiques".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1yzsgxN7nCKQw4v6nGtCD5LW32BzS2W_f".to_string(),
      chartdesc: "Magazines Worth Reading".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1RP0AqboSSv9yc0Z3ktXHWJvYmyfZdSas".to_string(),
      chartdesc: "Nazism, Mystical".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1X_LJytbNh6mX5tCSRLS4OnWRvtdMMrAg".to_string(),
      chartdesc: "Pol books #15".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "10A60vtv1k8dknR2IHdUbVGOPPCb2jxWL".to_string(),
      chartdesc: "Pol books #16".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1FQibFE69LVqROVRHu9mv1-z5rSgDUQ5N".to_string(),
      chartdesc: "Pol books #17".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Mz4MgAk76fC4AMbiQlZFhd8A8W2j-qbf".to_string(),
      chartdesc: "Pol books #18".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1rHhjlDHCOXka09wLKlvaPXyqWRCuuYvM".to_string(),
      chartdesc: "Pol books 1 - sep 2015".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1-onC8XVmNka5GkHY8dVhnlE_Z4wqJbb7".to_string(),
      chartdesc: "Pol books 2 - oct 2015".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1OvakjeVUaZLpr5OUTIPIx-p4v_CMBv4N".to_string(),
      chartdesc: "Pol books 3 - nov 2015".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "10bcJe0b3Z6nSlR6XlsMBRWTg8VTlSDx2".to_string(),
      chartdesc: "Pol books 4 - dec 2015".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "17naLdecSWB308TX67dY-0WYAqHdIxeKb".to_string(),
      chartdesc: "Pol books 5 - jan 2016".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1-bjI-lcIjFuxzWrvqD3db4py8g_SKCSc".to_string(),
      chartdesc: "Pol books 6".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Z7ST-sUSvmeNTgNukeQX9wloa1lmLRN1".to_string(),
      chartdesc: "Pol books 7".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1NN-qtGKMeIPugQgTmLq1R5ToVOP6pHgk".to_string(),
      chartdesc: "Pol books 8".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "10Eb_kt0gcUhI-ouGw2n2_WoE0jO3S9JS".to_string(),
      chartdesc: "Pol books 9".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1S8mxdLYkJfzi1Jtt3azFewNK2CQ00FHF".to_string(),
      chartdesc: "Pol books 10".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "15oSus2zK3FGoj6dfuZyztvVyfmJ0vqqs".to_string(),
      chartdesc: "Pol books 11".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1qkq7Yrzgv5AEqygTknYinUg8pJ3kdcNH".to_string(),
      chartdesc: "Pol books 12".to_string(),
      kind: "Politics".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1X-o1DW6CFEkX0ZL9c6gEawGoG5gfeAJJ".to_string(),
      chartdesc: "Pol books 13".to_string(),
      kind: "Politics".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1CNs2naSX7fYklqj42LHMFqgTuWHKcVqD".to_string(),
      chartdesc: "Pol books 14".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1idKzeGkqcw1722xdFs-EmedLd7fkMEja".to_string(),
      chartdesc: "Pol Essentials".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1IYHPv9mqEABKtpbdqTcNtQIPMF1FlAKI".to_string(),
      chartdesc: "Pol Journey Part 1".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1BYndcdTNy3uHkN2tYsoEBKSqajOU6ALS".to_string(),
      chartdesc: "Pol Journey Part 2".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1JQmyznTtINtQjFz6UGMRjB1d2Mlm77fh".to_string(),
      chartdesc: "Pol Lit Magazine Issue #3".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "18mKwQqVkmfrhJ8bB1nEH2c3ZaPO8zSW8".to_string(),
      chartdesc: "Pol Reading List 2014".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1c9D9S1AgmG5nbCYRHgtf0XuRyhhjg6Hg".to_string(),
      chartdesc: "Pol Recommend Reading".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1QsGcPdFnsrpSm806TEiaPCc0xiM4-15D".to_string(),
      chartdesc: "Pol X".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1FLr4QMYMxtNfjzXcwU2XVeauI7d4TZEW".to_string(),
      chartdesc: "Pol, Brit".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1qbi6GTFqeZF3UswBLOZ8zsgsbwZCWSKi".to_string(),
      chartdesc: "Race & Genetics".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1qYdg7SgzCDl7SzjyhvH5DCXXXLr-lQEW".to_string(),
      chartdesc: "Real Time Watch Lists 2".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1JJqyE4TYposEI-RdWW_PeR67qrz490TE".to_string(),
      chartdesc: "Real Time Watch Lists".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1dlodW8IXxhPZ79a8imy7y-P40OWiRkl0".to_string(),
      chartdesc: "Revolutionary".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1hEA7W7ehxg3yivJoiudNEeIiD_KequdG".to_string(),
      chartdesc: "Socialism, Right Wing".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1MmsxRaDrYkilPpPaGKo5QtS3hJ3FZpNr".to_string(),
      chartdesc: "Tradition, Introduction to".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1SueDciLXz09OZMSFKrbuQ474oFpiecPg".to_string(),
      chartdesc: "Whiteness Studies".to_string(),
      kind: "Politics".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Scc0cxTyUUSF4aOlMMB1bXfOrm0Uayvo".to_string(),
      chartdesc: "Assorted".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "17vTLK4mj7bOSpulQdm-pBm2EVwBxMTL7".to_string(),
      chartdesc: "Atheism, Fedora Core".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1S__qlUVT8_1lmDvRifFjsXJsaR46lDii".to_string(),
      chartdesc: "Atheism, Refuting".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1FOTPT9g276Sycnib4np45y649CyluSMK".to_string(),
      chartdesc: "Atheism".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1hH3h_IJNUSDsNbFW0I_8V3gD3T4D5bD_".to_string(),
      chartdesc: "Bible, One Month Reading Plan".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1k_JzcNZc9l3uZi4xFYmoNrmft40MTkdw".to_string(),
      chartdesc: "Bible, Traditional".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1xXSi1QoOEO_Zy2a0FfDqRRGzyDAjOEi_".to_string(),
      chartdesc: "Bible, Translations Guide".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1a7xJnmH31m7Gm968ysDnLCc2s60eewUt".to_string(),
      chartdesc: "Buddhism, Early & Theravada".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1q5yCBwvNLOumdrx9BeXYi4ijAIkhm4di".to_string(),
      chartdesc: "Buddhism, Non-Sectarian".to_string(),
      kind: "Religion".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1Y6OznTsAPYmX9anwTJLlDcSMAbFm3bnr".to_string(),
      chartdesc: "Buddhism".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1TLolTdK3-hQGR8P25movect5YFe-MFfS".to_string(),
      chartdesc: "Catholic".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1taDOJVVvdkbEDVZtP0oIHCYYkmzNtmX9".to_string(),
      chartdesc: "Catholic Japanese".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1RfRvNcvFwGxX2YMd_Rd-MYF1kd5NGr9v".to_string(),
      chartdesc: "Catholicism".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "10023Au2xe--8TYnzz6uIYCwqT_eY_bCz".to_string(),
      chartdesc: "Christian History".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Sv3-PjMBylcfprfimhZm9ym9BKpnwHiR".to_string(),
      chartdesc: "Christian Mysticism".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1MzpZmZfSEcroSoV3ArR-twX7h-AkFhrr".to_string(),
      chartdesc: "Christian Poetic Narratives, Second Edition".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1ifULzVgh1fKBAzh8cKRwMMYhuwSesoQ-".to_string(),
      chartdesc: "Christian Recs".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1De-DtnkQTwxJobAAvSggDF7O4ECfZZ-_".to_string(),
      chartdesc: "Christian, Protestant & Reformed".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1x_DtU8KJSGGY0Fqq6Kpc4qH7GkvY1y28".to_string(),
      chartdesc: "Christian, Puritan Starter Guide".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1zTxwjYBc8p0Qj77R6VVejOuLixs8UMhe".to_string(),
      chartdesc: "Christian, Theology of the First Seven Ecumenical Councils".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1qemuyTzQY23imNxuTiZvi4W8JTxFlsBk".to_string(),
      chartdesc: "Christian".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1gfWrL7W_qAY7IjT1xJWaLEouHQ3iT-8h".to_string(),
      chartdesc: "Christianity, Learn About".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1uUKswofPL2SuYeDzpY70XiRNm_4eO0o9".to_string(),
      chartdesc: "Christianity, Orthodox Intro".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1pBRGVuVflzH5-sfsvGHKXeEhYfaHG2vE".to_string(),
      chartdesc: "Christianity, Orthodox".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1RZYHSUKf8GGpul5__996JKVPyMCOzNQj".to_string(),
      chartdesc: "Christianity's Development".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1fypkFhjsLhp5IZ2iQB9ibI8nXdY-i-4_".to_string(),
      chartdesc: "Daoist Texts".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1J76lDfyw2ladsrVmx948FPV5oEsLJGfu".to_string(),
      chartdesc: "Ecotheology".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Jv2GP1f0uNGsSkmUHWcj5eM-hMibUEOp".to_string(),
      chartdesc: "Esotericism, Intro to".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1J6ixPNeouIb3xjMpInTpqJ_PefzWaqIB".to_string(),
      chartdesc: "False Religions".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1xxty4Y7LhRByvoQZrTRI6pCLACkGD_Hy".to_string(),
      chartdesc: "Fringe".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Q5VU_O0jgR60OYAWIP28_onKqPJjd4RJ".to_string(),
      chartdesc: "Hermeticism".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1s2y5pvUQ3Rv5pjKAcrZhLqPOD-LUXgXT".to_string(),
      chartdesc: "Hindu".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1iM6W0BELa3Xhrp8DckqUYgq5QhG8HwMX".to_string(),
      chartdesc: "Islam, Esoteric".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1kpBENTjOKlSx26IVblulx0n03pSwbINx".to_string(),
      chartdesc: "Islam, Quran Translations".to_string(),
      kind: "Religion".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1L7bSjVPJc5TWfE4GQd2CvMv088I19tUH".to_string(),
      chartdesc: "Islam".to_string(),
      kind: "Religion".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1r-bNFZssC6fUef9Y2-j3vB04aoKWIX_3".to_string(),
      chartdesc: "Judaism, Basic Intro To".to_string(),
      kind: "Religion".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "13xdy2ZIYUEr5A2G3X7hVGw0lY5Yrho1K".to_string(),
      chartdesc: "Mystic".to_string(),
      kind: "Religion".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1fSPb7s3X8HWjrlCIdix87rIZubJOOn4Y".to_string(),
      chartdesc: "Occult Chart".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1DBso1jGKqWUt9MKYBJTyhqPOlPvtJlg1".to_string(),
      chartdesc: "Occult Journey".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1wAw5f00R123IPKxmYKaNdNGz5dM29RWU".to_string(),
      chartdesc: "Occult Tiers".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "14nhetvFQYZXIF_06FZsvjMCkV12vt7rj".to_string(),
      chartdesc: "Occultism".to_string(),
      kind: "Religion".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1KgclbbR9p9My39xHqcoe-yf_23dK0ftX".to_string(),
      chartdesc: "One God Or Many Concepts of Divinity In the Ancient World".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1rvTISOUAMBKpKPYAW8TIF-2u8lNpwURP".to_string(),
      chartdesc: "Paganism, Celtic".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "13FHxk2VaACmRsYCIZy2-LLh6lgerC5I6".to_string(),
      chartdesc: "Paganism, Germanic Reading List".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1rk9TLT92C7PXvJ5kxcnlvKh8Decs8xdr".to_string(),
      chartdesc: "Paganism, Germanic".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Xz2QSmgLwvitWnVqs98y3JcUdN-TxS3E".to_string(),
      chartdesc: "Recommended Reading List".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1yvGEGTI_kzwS9emXbTsj0Q7oyOqb4wOR".to_string(),
      chartdesc: "Selected Buddhist Texts".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1OJA0jQ5-POOIOW2aqgYnQjvSVuWtM9jB".to_string(),
      chartdesc: "Seraphim Rose Fr. Reading Guide".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1TgSiVSOmcEXSJ5H-6l-qESS8LHwfaR68".to_string(),
      chartdesc: "The Quran".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "15vhV0hnTqQP6tsXegJFSZRXlwKjgtKiq".to_string(),
      chartdesc: "Which Bible Should I Get".to_string(),
      kind: "Religion".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1EOA-TUPb_UgxT2S4kpM1CJ9Ng3YOp2fK".to_string(),
      chartdesc: "World Religions".to_string(),
      kind: "Religion".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1bt-FE3jqZfVRb-ozPcatOmNhj2q2e2Ae".to_string(),
      chartdesc: "Books To Avoid".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1yI0Q0UCQE0FZK7NUIj8CovG1xZ3-202j".to_string(),
      chartdesc: "Degenerate".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1tRoqxsqxV9zDhW2g_UN3cl-uozbyuLA5".to_string(),
      chartdesc: "Dune Reading".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1jsbk5MOi9WTg6hef-FDacMjtdFpG3XM5".to_string(),
      chartdesc: "Mature Women With Younger Boys".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1wI2m-1N6UHtBBtmu0SnOgtRJxX-Io6aO".to_string(),
      chartdesc: "Military Fantasy First Contact".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1KjGrA1R_cOZsl6JyfKfRljQetIDA1Mjm".to_string(),
      chartdesc: "Reading Brandon".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1lx_6pPcuTfcO2qpS5Rkg64Chl7KFKBoM".to_string(),
      chartdesc: "Recommended Fantasy".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1ImxciNogzy_pgmrxBZrloKelAVCx2XcA".to_string(),
      chartdesc: "Sci-Fi Essentials Meme Recs".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "15Bj3nStC80Sib19e3Ad2GYG4SqZ99iHr".to_string(),
      chartdesc: "SF Essential".to_string(),
      kind: "Scififantasy".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "171CoNFcJMNzss4HKHjJqxH4a_Wxg1KKK".to_string(),
      chartdesc: "SFFG Top 20 2019 Fuck Democracy Edition".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1gs9CdPHwu8ZXaoS3-tgafhXQv_sG1Xfc".to_string(),
      chartdesc: "A Guide To Navigating NPR's Top 100 Science Fiction and Fantasy Books".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1-5DyoOHmyrm7P81edjMXEDn3J91OSDZn".to_string(),
      chartdesc: "About Women, For Women, By Women".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1NzVyuTXGPfz3fxws5FErFy2RYzSqCvpL".to_string(),
      chartdesc: "Arthurian".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1vcmpKdqNm8JN3RJhoFmhlIx4vOtF7s-j".to_string(),
      chartdesc: "Beginner's Guide To Fantasy".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1gNh911iI6EDzV1xv2Ea1dFQPduJQ6zX0".to_string(),
      chartdesc: "Conan The Barbarian".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1sD0C2NPuKB35Aljp8OS-guK3CLxRl0-Z".to_string(),
      chartdesc: "Cosmere Reading Guide".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1I-9XwFjr2rJKJzj1uDFvus2MNmRygBxQ".to_string(),
      chartdesc: "Cosmere Reading Order".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1jgJt-GxyHMKc-MUDAjtB0e7-pCGkKsTj".to_string(),
      chartdesc: "Cyberpunk Pretending".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "12LtCYjkh9aPsNipBpHFS3k-kTQiWaDWA".to_string(),
      chartdesc: "Dick, Philip K.".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1KaB77jP7dep_8qIiOR808cYbsXoDz9nN".to_string(),
      chartdesc: "Dick, Philip K.".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1iWatXeSHlv43AwfWrYjosn8Hiau3Fb8R".to_string(),
      chartdesc: "Dungeon Synth".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Brcv5rYrN0bg7hTvVOmE6TNismnTvf-I".to_string(),
      chartdesc: "Dying Earth Core".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "179ZnSe6nJc4zz48sbfsbb1eiwycrLD8Y".to_string(),
      chartdesc: "Dystopia & Utopia".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1m0L-6uuqPGdzg6QWJrBT_O0L6gx4ewpJ".to_string(),
      chartdesc: "Fantasy and Science Fiction of the Past 20 Years".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1GgDB7oSlJz86gd4IkCxkhSXIXyBMWbX-".to_string(),
      chartdesc: "Fantasy Flowchart".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1hScLkXjmGInPFAhUWaROUg4fSwxX3Sdk".to_string(),
      chartdesc: "Fantasy Flowchart".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1YyCregK_sIN-utwGHvhAnI5EpeCKaNAw".to_string(),
      chartdesc: "Fantasy Genre".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1lN_4ZeGqF3LU-LEw-WOxR2MuVAPrxCX6".to_string(),
      chartdesc: "Fantasy Guide".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1SGcC0IXXEptmaqmeop726v9Ron6ifLFE".to_string(),
      chartdesc: "Gamelit Favorites".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1h8IA8eORdJp_7gYF89uns6BUWYj0qQUk".to_string(),
      chartdesc: "Hidden Gem, Sci-fi".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1wz_ow5_DzFb1Zx6xLQUuVj69de0H0SuP".to_string(),
      chartdesc: "Lesbian Fantasy, Top".to_string(),
      kind: "Scififantasy".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1_GZADyAith1S7kOpngXNyOgD3iJsJNEv".to_string(),
      chartdesc: "Lesbian Genre Fiction".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1GU9c5g6Y8cTRv2J1AUKw2SB9fS7V70Ag".to_string(),
      chartdesc: "Lit Selected Fantasy".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1n5bhtSAmv8aODz9Y_dDXIM5vTyliojO0".to_string(),
      chartdesc: "Lit RPG Chart Final v1.0".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1GPgD1vUnVMcKBtbMCr6W5OR8W3mNSDEs".to_string(),
      chartdesc: "Malazan Reading Order".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1xqWysYnIabuxJdhIDMBg8d8Li1N-fdpe".to_string(),
      chartdesc: "Mark Lawrence's Reading List".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1rGZ7gBipdFm0b15S-ScY39NfQYmqoENm".to_string(),
      chartdesc: "Middle Earth".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "16aLXtg6_dNjta4eq4nW_C4fBe8WX-OMs".to_string(),
      chartdesc: "Mieville, China".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1J4t626jVLLIkemDJRL5kv-P7sAuI84Ag".to_string(),
      chartdesc: "Modern Fantasy & Science Fiction".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1PLJjHOPapEeID8-l0dLhSr0Gd2VZgpUM".to_string(),
      chartdesc: "Neal Asher Polity Draft Full".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1fuadXBhUIvWxOx90ejwLqWJ8FOk6Ucra".to_string(),
      chartdesc: "Neal Asher Polity Full".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1cAMUkkRRP9ww8Dgwxpx_OZWzXO8kvwXl".to_string(),
      chartdesc: "Non-Tolkien Fantasy".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1W7ayhuI0EM1OdqETZugbhVpu1Wxa_k5L".to_string(),
      chartdesc: "Personal sffg ratings by random anon (July 2020)".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1E1knCaEg_py3MRrrh0hiKPgFoqj5Oi6e".to_string(),
      chartdesc: "Pop Fantasy Tiers".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1F3b8bZoCJWph-WTGnc8SCbWk293WjF-5".to_string(),
      chartdesc: "Post-Apocalyptic".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1rdiTrRd852FNFDY8GQjMzcCegiHCFcpF".to_string(),
      chartdesc: "Recommendations Prototype v5".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1R1cdUmGnNbRLMP6DBL5y-eXxo4oz9LC4".to_string(),
      chartdesc: "Russian & Polish SF".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1bEjZ3MTSFRTC8206wNPc1cvo55YpL77K".to_string(),
      chartdesc: "Sci Fi Genres".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1sblRpIREiejn-C0uQjreBsueK3OUg-Qn".to_string(),
      chartdesc: "Sci Fi Guide".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1l-E8296I6fjrihbLwJpqMF3IR2iKDtbg".to_string(),
      chartdesc: "Science Fiction Crash Course".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1vzNZiaQTkkqNc-u_g_qE5BQyZqRolneb".to_string(),
      chartdesc: "Science Fiction Favorites v2".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1wDpvA4eDPzX-_TAovKnrnCAvZQgLU_Rk".to_string(),
      chartdesc: "Science Fiction, The Best v5 or something".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Ii-lXsQY6mMV-cM4Q0Qn9vXL8yCgyUqt".to_string(),
      chartdesc: "SciFilightenment".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1kxC9o0vCcIL78XCyRPlsCmY2eJltgObZ".to_string(),
      chartdesc: "Self Published Book Chart".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1ktd7QG-s8JlL4qfL_TpwHfD3OrIRfyG0".to_string(),
      chartdesc: "Self Published Recommendation Chart".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1u61WBr53qUBbcYdvWglb8pAqrFaTdy2l".to_string(),
      chartdesc: "SF Guide V".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "14q7-RHbMDJoDzbw9A_VoA29pMW6KvbHH".to_string(),
      chartdesc: "SF The Big Three".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1a3kmZUtOstHjdLF6dUI7K5wTLVbvUYM4".to_string(),
      chartdesc: "SF What To Read".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1i-7ffpECMmlz-MB-ECZXI0bZKUyILnvO".to_string(),
      chartdesc: "SFF Vietnam".to_string(),
      kind: "Scififantasy".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "179Fmq99b8u6i1AoaHYKsRFR8wNsTkNm_".to_string(),
      chartdesc: "SFFG Rec Reading".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1AwA8dhQKEEDZm2vS8LkqGv5Cl5paE2kM".to_string(),
      chartdesc: "SFFG Recs Dec 2020".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "174P9ZDfS65yc8c1U9HndgDtrxuwiewYB".to_string(),
      chartdesc: "SFFG Suggestions".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Xu0EkdNtMLjWkB12mMM-t-1vW9nYO-RQ".to_string(),
      chartdesc: "SFFG Top 20 2019".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1LIGFwZmUrlTiJnkQGG5CFrhb2cE7lA9S".to_string(),
      chartdesc: "SFFG Top 25 Books 2020".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1hWuFCr-hA1dxmdOcjY1jYC3vX8s7kAQz".to_string(),
      chartdesc: "SFFG".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1uqjEwHrpDSrZGPAmTQFCHMRbcGgHN0mS".to_string(),
      chartdesc: "Soft SF".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1eJBtMWzB6mDiOdhcUHIazdWWEAuoEv46".to_string(),
      chartdesc: "Star Wars Tiers".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1bSH22CNZMssN6mevdJxm1mHpTqY4AhHg".to_string(),
      chartdesc: "The Best Science Fiction v3".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1rw7mnpHgSIqZ8DiMRDuCWbi5kzjC3Li2".to_string(),
      chartdesc: "The Discworld Reading Order Guide 3.0".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Al-Yg1FZiM-mlNX1NE3zVCv1hKZWGPow".to_string(),
      chartdesc: "Van Gogt, A.E".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1cNLgektvvut2rXRaViEnATlXH2Rc2riP".to_string(),
      chartdesc: "Wall of Text Chart".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1_TjC7w8hAG-JSQAxhUnJKYJWl_b1DqfU".to_string(),
      chartdesc: "Warhammer 40k Recs".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1mNHlk_-qU8dZQRp8NWQIRE5I-Nlw2cZm".to_string(),
      chartdesc: "Warhammer 40k Tiers".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "10fot2lQzJh_10eGBrRSqFONeDKBLWNVq".to_string(),
      chartdesc: "Warhammer Tier List".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1O7ZuOKar62QFjzN_1Yoj1JPTy6kfJIZB".to_string(),
      chartdesc: "web novels".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1K-osEdZYLKWb-SqmajtoNv3jslESAZgq".to_string(),
      chartdesc: "Wolfe, Gene".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "126gAWxhvn84XhX2o6ja9dw3x40BdAcEU".to_string(),
      chartdesc: "Writing Fantasy".to_string(),
      kind: "Scififantasy".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1q8eirU1JH2cL3jUjjpW9Jy4oceWdqUZK".to_string(),
      chartdesc: "100 Horror Authors And Titles".to_string(),
      kind: "Horror".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1USphgUWTZP1m0ecKU62UZFSuszGjW7ut".to_string(),
      chartdesc: "Dark & Disturbing".to_string(),
      kind: "Horror".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Wlwz7M36035vhECFG5oqxh1_da4iDJeW".to_string(),
      chartdesc: "Gothic & Horror Flowchart".to_string(),
      kind: "Horror".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1pjctyGsnEaJKCl-WDPZrlc_5fjM6YqA-".to_string(),
      chartdesc: "Horror Collections".to_string(),
      kind: "Horror".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1bQPv0sEkWL0OuqNOWFWHZreWTIi3RvU4".to_string(),
      chartdesc: "Horror Fiction Types".to_string(),
      kind: "Horror".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1H-qAWMf43g_z5I-l6pre6YOeeUQuLZcW".to_string(),
      chartdesc: "Horror Fiction".to_string(),
      kind: "Horror".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1ZLU6PRY5udNgF4aVKDSLMY8v6ykZMTXD".to_string(),
      chartdesc: "Horror Guide".to_string(),
      kind: "Horror".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1fdfyw3Gk4BGFRS_CY0xPech1yqJ4UH4n".to_string(),
      chartdesc: "Horror Timeline".to_string(),
      kind: "Horror".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1yC1t5T6n1JpirCOErUzpmmjJQsYLapNY".to_string(),
      chartdesc: "Horror, Japanese".to_string(),
      kind: "Horror".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1zQ-KD_k_nvITEZR4KaYqkaR09Zy3AZZy".to_string(),
      chartdesc: "Weird Stories".to_string(),
      kind: "Horror".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "17AQVA_SQi_nAFR52WS-rc2rAnA_OUjWT".to_string(),
      chartdesc: "X Book Recommendations".to_string(),
      kind: "Horror".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1HfdwocDxllAkBCQLFHQhyD5sj1ywIeRy".to_string(),
      chartdesc: "Academic Manga Guides".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1j0RW61cPlJ0ZPeL2iOGM1OmOJbDDS8gt".to_string(),
      chartdesc: "Academic".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1ovTVR8mowOk81koZuQQZdsJd919diQru".to_string(),
      chartdesc: "Architecture".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1RQFG6s5lncJv8GTvhx6LaPvh7fzeu8z-".to_string(),
      chartdesc: "Assorted".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1ZKWhnBEGgyLjR8zWK36zk56REWrIuVb0".to_string(),
      chartdesc: "Biologyv1".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1COYq-U3420D97LHtAPMlrLJozfg2jz5_".to_string(),
      chartdesc: "BizLitIntro".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "12h6DHLaECVBJpYwcv587gzxaime-0MNy".to_string(),
      chartdesc: "Biz".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1OQ85Q9JeaNAoMVkZkNfSbhDTbmXHILHW".to_string(),
      chartdesc: "BizLitWithoutOrder".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1D2LQ9yP5tF0F6Q4_jwmikAzI8zqwW-bQ".to_string(),
      chartdesc: "Business".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1TWQkmlEvr0JGzdjwbuTR1aGXstfRvPGi".to_string(),
      chartdesc: "Computer Science Core".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1TWQkmlEvr0JGzdjwbuTR1aGXstfRvPGi".to_string(),
      chartdesc: "Computer Science Core".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1B7tUtzUILfqrHYhje3FX9fEyDlgXsaZD".to_string(),
      chartdesc: "Computer Science Programming, v0.2".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1_Q1cXWfBrfbjfvKHLqbmJjB0sgudG4iP".to_string(),
      chartdesc: "Computer Science".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1R0dk7JFy1Fe4SIklkq_uRiIR9BeJmDGF".to_string(),
      chartdesc: "Economics".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1aAY5cxeeM7Kz98Pf6y0Cvw29r3C8chlo".to_string(),
      chartdesc: "Film Theory".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1JZVddQ7559YCxE6sF_fT0WrySgl4OLjW".to_string(),
      chartdesc: "Essential Maths (Spanish)".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1eqPZf0WdD1C8UmostnufNGySooJMvbyW".to_string(),
      chartdesc: "Math by years".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1mVWqVM-K-kNgRtM-xCPl6cn6jopl2dz2".to_string(),
      chartdesc: "Math Logic".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1re-XpWa4TEW7qAPGNDfihwMO8_LhuNs5".to_string(),
      chartdesc: "Math Tiers".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "14KD_MZ-vRuUTp-DJJd-bQFy9-OIoLAIx".to_string(),
      chartdesc: "Math Unemployed Guide".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1LvmsDGNGmSypQooy2Gpc1dJOfjmA0RHG".to_string(),
      chartdesc: "Math Unreadable Edition".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1atq8PjT_hxNeqBs8CmvB7pukweMj4nAY".to_string(),
      chartdesc: "Math".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1oQcPPsy6ffg5IWeveRCXiuWKzdXUVdL1".to_string(),
      chartdesc: "Mathematician, Becoming One".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1cV2JGdNQSeREKm2aJB_g3Y4kGlfhSdIm".to_string(),
      chartdesc: "Mathematics Study Guide 2".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1-_st5ry48nQrHbkFIpm4VY20AjG03gii".to_string(),
      chartdesc: "Mathematics Study Guide 3".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1f-77NGoswH1sH3XKx7_LzPK0I8kaB1-a".to_string(),
      chartdesc: "Mathematics Study Guide v1.0".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1zRPxJ9ksiht7SgmlYXX-nVZvHtFSeIX5".to_string(),
      chartdesc: "Mathematics Study Guide v2".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1KVoWupW6qqvXXp2JUdARYtu6PKTwVDzZ".to_string(),
      chartdesc: "Mathematics Trench".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "12P4duCQCqTCrke6t2w5K8lKUj8ZEeanK".to_string(),
      chartdesc: "Mathematics, Basic Course".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1vOHJfila7kHDlcWnLhyNtH5BrqL-PuSn".to_string(),
      chartdesc: "Mathematics, Guide to Starting".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1TRVgjMGRIKbL2PDPIKGg_lnR1hj09w-s".to_string(),
      chartdesc: "Meme".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1X3Ioep2N9Z0M9uH4YgvO3k1CwEB6CXiT".to_string(),
      chartdesc: "Nuclear Physics".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1cYqNAkwuWlAMQGB84m1Fq79QDVEtFLru".to_string(),
      chartdesc: "Programming Books".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1D5SzBFs6Kstx0IOvwzfcJENEySc6mEfk".to_string(),
      chartdesc: "Programming Brains".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1P3YJPUj0NYpriSu78_hcDGAN1y-41uBs".to_string(),
      chartdesc: "Programming".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1HnWizF6hio5Zceehi3Q2uny-btXZ2Ro-".to_string(),
      chartdesc: "Required Reading, Easier Mode".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "10DHB5enIEcdvQbk4YgI8UmeE8sIWupmC".to_string(),
      chartdesc: "Required Reading 2".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "19JnPweV_LW96TJ_2P7c6ZJqGvUlfLNy4".to_string(),
      chartdesc: "Required Reading 3".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1PR6V5HSkQoS94v6Swr0LYV58bknrgLKw".to_string(),
      chartdesc: "Required Reading".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1DJJgAq1u6-65ZdRRfYJWwCiUbq6zGndk".to_string(),
      chartdesc: "Sci".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1FIlmYs3FIvN3fYxRj-TBbBkAaN_hW9O6".to_string(),
      chartdesc: "Science Literature".to_string(),
      kind: "Academic".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1Wx865a-ogh2e8NeZkeMfni6mt4D9Buhv".to_string(),
      chartdesc: "Bronze Age".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1WyxmqAETgD8GUnC7NQhqoylzXXAg4vSo".to_string(),
      chartdesc: "Classical 800BC - 500AD".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1tEZSYs6kg992zaK0UbT5k4Pw2ga0CWfr".to_string(),
      chartdesc: "Grecs, Commence Avec Lex".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1MvRG6gJ9VdBqiETBDWFPOEeDG4M56RjZ".to_string(),
      chartdesc: "Greeks, So You Want To Be A Classics Nerd 2.1".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "11RI40jQd2voIDPubmMhs5sj31hQ-Qgxg".to_string(),
      chartdesc: "Greeks, So You Want To Be A Classics Nerd 2.2".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1zzZmetz29ZiQJKr4uhzEJBMy5ejytgFv".to_string(),
      chartdesc: "Greeks, So You Want To Be A Classics Nerd 2".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1G0fvTaxc8DZF5DdWrA146Kk9JEYwjZ2h".to_string(),
      chartdesc: "Greeks, Start With The 2".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "163Ctuc1NsLSfcK3z9PQf-Da4cevoWUso".to_string(),
      chartdesc: "Greeks, Start With The".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1G_wWRpmHa4tgXY9BdqVK4rYWvbA-JM6K".to_string(),
      chartdesc: "Greeks".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1w1cvtg4UbXjCKM3GjpbfDR5mJFPpQCvx".to_string(),
      chartdesc: "Hellenes, Start With The".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "10CuPi_q9jdw5K2VKwY5pcuYIR9ftEWCk".to_string(),
      chartdesc: "Indo-Europeans".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "15ATQyvz1PyiV-OByUiknloLUUn_BIpQ-".to_string(),
      chartdesc: "Romans, Resume With 2".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1m05EvqO6uUrCbO4MWKl0IadyGlDm6sC0".to_string(),
      chartdesc: "Romans, Resume With".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "19HHzJMmPo0kuzAWQMOLyn-6uNGAfAYuh".to_string(),
      chartdesc: "So You Want To Be A Classics Nerd Part 1".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1-rlDkcJrbDf4PNQVNKho3eNBgLK9wood".to_string(),
      chartdesc: "Sumerians".to_string(),
      kind: "Classical".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1S3cyyMUl-Qq4lBEVpw0lbrsomRSXs9s9".to_string(),
      chartdesc: "Trivium".to_string(),
      kind: "Classical".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1hUfhXhAxtYGwD2X5Grde1naIY7fGsT_R".to_string(),
      chartdesc: "Alexander The Great, A Reading Guide To".to_string(),
      kind: "History".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1a_wEdgdjrPciKKtKjKGGZhu6pFPE4KN3".to_string(),
      chartdesc: "American Presidents".to_string(),
      kind: "History".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1TtQnZvSwWVCEv571hgvMllow6EzP05ZV".to_string(),
      chartdesc: "American Revolution".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "18GKCXi9NNhTRVclDdZWsi-U_hR96TVhu".to_string(),
      chartdesc: "Anglo Imperium Reading Chart".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "18_auka55T1eD3Dx2mNErMNhPitt_cRsZ".to_string(),
      chartdesc: "Arabia".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1NbtBayce1paQSES2P0ohUmR2Gb-k8HoT".to_string(),
      chartdesc: "Bronze & Marble".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1pXygmZQFsRpnIPCE28D8o-txJFN8pkJ3".to_string(),
      chartdesc: "China".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1tsVJ-f1TWEHVyTHmAKdipXaKzH9Rhhdh".to_string(),
      chartdesc: "Civil War Fiction".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "13YgRSdmdUzx-Y_6_x2XqmZLcVqNDPQLE".to_string(),
      chartdesc: "Civil War Nonfiction".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1-0sWbeCzTSPr9Q865vtm7d_DUn5rR92u".to_string(),
      chartdesc: "Civil War, A Very Brief Intro".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1-0sWbeCzTSPr9Q865vtm7d_DUn5rR92u".to_string(),
      chartdesc: "Civil War, A Very Brief Intro".to_string(),
      kind: "History".to_string()
    }
  );
  
  bookidsvec.push(
    BookIDS{
      bookid: "1wX8XRh4AMPtiYh0LIPuoIX6g8DgsAcMl".to_string(),
      chartdesc: "Fascism".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1cYQS2NQFTXQ2FsiKDNhhKJC8Nj8DVYxR".to_string(),
      chartdesc: "German Colonialism".to_string(),
      kind: "History".to_string()
    }
  );
  
  bookidsvec.push(
    BookIDS{
      bookid: "1jbuRRH7rardND7m3i_fSHilskATOV26a".to_string(),
      chartdesc: "German Nazi".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1oyLf9oOQPHdYPlpMjXuzw4qOUbQk4IFr".to_string(),
      chartdesc: "German, Weimar Republic & Nazism".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Mhltc9Rf4W4WDvnewxg7oWuWrESWYXWz".to_string(),
      chartdesc: "his approved literature part 2".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1gJ72yTDCoOi7HUTXuNDXUn_NtyB5mFPr".to_string(),
      chartdesc: "his approved literature part 3".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "172xFHw_RnCoUwjdQYVVPD9L5YzsVRZPD".to_string(),
      chartdesc: "his approved literature part 4".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1xuWJMcHJKhJs8f52cUfGAdj2sc2nkpq5".to_string(),
      chartdesc: "History Books, Haphazard And Unqualified".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1clSyCYmvsbHVFHFIaN5wDTGXCpATytQD".to_string(),
      chartdesc: "History Books".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "13DZ_f2J0FaMh7iu3xCUIlaQCKgEtGnDB".to_string(),
      chartdesc: "History Overview".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1pSJlYucs3auCzdEwu6n_6tXWLldLsF_R".to_string(),
      chartdesc: "history podcasts".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1Z4EzoTHjOQVeClKX0nbUl_EQJllzmMeR".to_string(),
      chartdesc: "History Recs".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1A2YWMibKwioMHQO55rcW4SBwb7Kfs0Ug".to_string(),
      chartdesc: "Italian City-States".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1sGgZmq9YvTcl24-C6xlg59JZeQSxZ5rD".to_string(),
      chartdesc: "Late Antiquity".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1u7vC_UrjL0ZG97cPvq5Z3yECd95KEUJu".to_string(),
      chartdesc: "Mesoamerican History".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1jq2v-gBMJhGZq29lro6xPBFpsKQGFBrN".to_string(),
      chartdesc: "Middle Eastern and Levantine History - Conflict Edition".to_string(),
      kind: "History".to_string()
    }
  );


  bookidsvec.push(
    BookIDS{
      bookid: "1w-fsUEgRazq2ngqsCEVmD_7ilgzO-saR".to_string(),
      chartdesc: "Nazi History, Beginner".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "17PTePQuuOoZuYOOxPaS65CRj5SVb9wb8".to_string(),
      chartdesc: "Nazi Origins".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1UYMfzVyIgyqpOWFyhITBZaL2G-3PAIWe".to_string(),
      chartdesc: "Persia, Sasanian".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1BRvrpWOO-PmtZnJs9Sz4Bw7wmGNTRcTc".to_string(),
      chartdesc: "Prehistory And Ancient History Bibliography".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1QNAYCW04FsTBIIJE5AxqZa4wmSioNdt7".to_string(),
      chartdesc: "Prehistory And Ancient History".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1JxZoNNsxRE8WBaLNqnzgpP3FjIuaIW7N".to_string(),
      chartdesc: "Roll, Political Research".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1LXW1D7fJ9HuNFIapKAaOpFsTSgwOdonT".to_string(),
      chartdesc: "Roman Empire, Eastern".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1ubs5y96UK1d6CjAlQM71bwqByfAsP5L6".to_string(),
      chartdesc: "Roman History for Plebs".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1AG0DIH0Uu04CofQ6St1hXBey_OmE97hU".to_string(),
      chartdesc: "Russian Revolution".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1cQWif6dm-dmi3MBza8mtxMIby2fbuDZe".to_string(),
      chartdesc: "Seleucid Empire, Hellinistic Age Podcast".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1fJigE8pCppl49WxZG4vojRr0mmMw14B-".to_string(),
      chartdesc: "Sino-Japanese War, Second".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1WlZRRHe7nHffl-6E0b7XOa-NfpGEI9yL".to_string(),
      chartdesc: "US History Infograph".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1SkbwgPGxHO47mMh9rzHVGTRrtXcbPu-a".to_string(),
      chartdesc: "Vikings".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1AQGs5NM3EXMBkKEtvZVt06hGcmF8xl6c".to_string(),
      chartdesc: "Western Science".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1JriKsjRwwFrxlHiDGb0iNZtAsjQVTHBn".to_string(),
      chartdesc: "World War I his guide".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1wWAkU7kWkrjtZpUDzUfUWz4HUy5Z_IPr".to_string(),
      chartdesc: "World War I".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "1stubLE6mwvVmAbnlDTjGzc2sX1PF4OYO".to_string(),
      chartdesc: "World War II Mises".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec.push(
    BookIDS{
      bookid: "19hBHfVCeNH-zFAFvbIJButGHQNnIIW_b".to_string(),
      chartdesc: "African Warfare".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1Hm7BWuAL2TBMOaIuDxCtnud5H3-WJ6Xz".to_string(),
      chartdesc: "Albania".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1x6at_WT0nv-jhircKC5EfcV4yO6lBwo1".to_string(),
      chartdesc: "American Civil War Cavalry".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1lDLAdsRf60Hr5atmsxFrCwDHDjvbWLvo".to_string(),
      chartdesc: "American Civil War Innovation".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1Md3Khc6grmeXqu3dXF8oNibm4YREJGx_".to_string(),
      chartdesc: "American Civil War Medicine".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "11XyJQu51hiOF3VZkrOWvNR92TIgwrwcf".to_string(),
      chartdesc: "American Civil War Navies".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1doJ5aFL128FRJC5jct-Lx2eUa8bTUeFn".to_string(),
      chartdesc: "American Civil War, Antietam".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1NQMEdz92s6uM7Q4R9udOjqglS6UYdr2g".to_string(),
      chartdesc: "Arabia".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "168XHoAAfyuG3iH-MA5a9Q0icCan9k5vs".to_string(),
      chartdesc: "Balkans".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "15MVoiPBaAI6e8uJzWPO5Q4TuzUVrT0mX".to_string(),
      chartdesc: "Berbers".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1jwcmazwS65TEUHhdEaghqSEo6OSnrRy2".to_string(),
      chartdesc: "Britain, War of the Roses".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1T5iuuhPFGVZAy_KLHuR2eJkAYAAgxEjp".to_string(),
      chartdesc: "Brittany".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1S0kL1_pNd5SwgDYTs4i8rg5RQlA0wH1F".to_string(),
      chartdesc: "Cambridge Historical Books".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1ULOEoxblg-uARvpJnnDHjIQKinYHBnM0".to_string(),
      chartdesc: "Carthage".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1wZdRkj1fH6ZBTdFg4T0ydLQ79SvOFciF".to_string(),
      chartdesc: "China, Yuan".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1LYfl3S1bpRYytdLR-eOqtgPUhCz2a6_k".to_string(),
      chartdesc: "Egypt, Ptolemaic 1".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1LCmPLbqehpG1uK5YlO3ICkDsVckFkGB_".to_string(),
      chartdesc: "Egypt, Ptolemaic 2".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1cV5HG-nji6F6qxAnGnT6oHWKuFJVJ0HG".to_string(),
      chartdesc: "Etruscans".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1Zy-UDUaCwzwp903k1ygkTHUffS5_FRQ2".to_string(),
      chartdesc: "European Peasantry, Medieval".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1OYXqDEx_k517z-FwwTwo5b8EvyV60A2s".to_string(),
      chartdesc: "Finland".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1JlQuWsLGXLRlhhWp78fNmzU95rRrCIPs".to_string(),
      chartdesc: "France, Early Medieval".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1roJYzXDTE6wCupRvB5OQGs2zP6oPban2".to_string(),
      chartdesc: "Genoa".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1JhdmiB3Y-qIul_9NT9E1wmZpR05SW02I".to_string(),
      chartdesc: "Hanseatic League".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1E-JznC8ByS2pzHiMxynqXu7Ue-toGOee".to_string(),
      chartdesc: "Hellenistic 1".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "11Su6_Wjbq4lIz4cU8yWWZZygvuPCgJh_".to_string(),
      chartdesc: "Hellenistic 2".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1Cl8zy-Ik6Z5_hJepioKQl_TfYwfUFOLl".to_string(),
      chartdesc: "Hugary 1".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1MRyuVoPa4e4lZaHgQJMdW8Fib5Kb4zuL".to_string(),
      chartdesc: "Hugary 2".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1T0U5PXngAMhzwjAKjLkJvoYrvOC994EW".to_string(),
      chartdesc: "Ireland, Early Medieval".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "13Gh9cznOlUU8v1aYopgvQxA4CXY7Cwfc".to_string(),
      chartdesc: "Ireland, Late Medieval".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "10xtHVFiHxpZL1HxAL5QISMpSEVSj6hoI".to_string(),
      chartdesc: "Ireland, Viking Age".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1FI88xcYIWrxsnELntklJgxzu4Vx9B0Vi".to_string(),
      chartdesc: "Israel, Ancient".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1vXtf9vXkcWIoqO-tTHK9RHpg_eRW3bR4".to_string(),
      chartdesc: "Italian Fascism".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1zj-RnR5GR_SYwksZDjCmzfiiJ-GZAw7J".to_string(),
      chartdesc: "Italy Middle Ages".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1sVkJtzHLFQppojBHG8Ungk8futH1DK4y".to_string(),
      chartdesc: "Italy, Milan 1".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "16dzk4SYfqA8_kLdRCm5TkhNY_3IUBnTq".to_string(),
      chartdesc: "Italy, Milan 2".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1NpjPRWTmQa9PUR59nrT_tQjsdAXErd8Z".to_string(),
      chartdesc: "Italy, Norman Kingdom of Sicily".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1umG-I_LFoRVGfpE3QmSzMvsxOyWDlV09".to_string(),
      chartdesc: "Knights Hospitaller".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "16OH5opcTy3soGr3goghtNDrPLFpYxcg1".to_string(),
      chartdesc: "Lenin".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1SYpxS_eOMSMLWU1IJQW_oZqPA1fiB_or".to_string(),
      chartdesc: "Magri".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1GQARrEvv9AXFxlDLyzLpAG8Qp9577vFh".to_string(),
      chartdesc: "Mormon".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1yRUcmpohMVjzr2qzH3vdWr3OitXDYgey".to_string(),
      chartdesc: "Mycenaean".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1-3pkfeNbldvhpiVkBs5rcrtBEu2jgUq7".to_string(),
      chartdesc: "Near East, Ancient".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1F87iOnvjHQs4PKqEP45Zw6QRbRxpwmrZ".to_string(),
      chartdesc: "Pacific Islands".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1l6vQohs8tPEGPFF9Wd2Qt1-7E7KZnt1W".to_string(),
      chartdesc: "Phoenicians".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1kNA1R4fJ_ERDJd-3gSoK1mwG5gWgQht7".to_string(),
      chartdesc: "Poland, Medieval".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1P6urEh3tyFYkV210tuRHeIyLAEOBJLx9".to_string(),
      chartdesc: "Polish-Soviet War".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "15xqC4PJWd18blSh6K13670N6rITc46sR".to_string(),
      chartdesc: "Portugal, Early Modern".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "19SywM62g2Uuen87no8dUMmODKNndSzSZ".to_string(),
      chartdesc: "Portugal, Middle Ages".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1nmzVrKYzM8krnxWVj1uC6yAyFDGOX3Cj".to_string(),
      chartdesc: "Portuguese 2".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1vlOf7MSnq4d1hMZ2L1aA_S-jmJZDpc4O".to_string(),
      chartdesc: "Roman Republic".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "13esERYURL6T9E5rqz93j_CBdrAChtMal".to_string(),
      chartdesc: "Sami".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1AyRIL_2JxpO9EZlhF14xfy3k_BqM3oph".to_string(),
      chartdesc: "Scotland, The First Millenium".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1SjBdBUdTbCI4JfPJ-_MihTJlqYbA1toa".to_string(),
      chartdesc: "Seleucid Empire".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1C6JQYEEtQt3BBQNefN0aHHs0HoFxryPv".to_string(),
      chartdesc: "Siena".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1GiF5hE42ZbzPyHD0TcsK7Jaxo9qH9n4D".to_string(),
      chartdesc: "Spain, Civil War 1".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1OLnK0G1K6L1sEXNdIypjdvuRckdcw7E2".to_string(),
      chartdesc: "Spain, Franco Regime".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1YzK-MTtRSKg4gTulSzoDdARAkl7bl-i1".to_string(),
      chartdesc: "Spain, Muslim 1".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "13U9syVkJiVc2KnQoG1zXeRsA9twQTzoU".to_string(),
      chartdesc: "Spain, Muslim 2".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1goTZjVH_iK_IIC13xEy1SyyG0Z1SZW6I".to_string(),
      chartdesc: "Spain, Visigothic 2".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1s6hgoYhRfS6X6zL2STppV1KLJsFuFN_f".to_string(),
      chartdesc: "Spain".to_string(),
      kind: "History".to_string()
    }
  );
  bookidsvec.push(
    BookIDS{
      bookid: "1XI2w3mDxvedBbWgw3gzK9nie_du_Azyx".to_string(),
      chartdesc: "Venice".to_string(),
      kind: "History".to_string()
    }
  );

  bookidsvec
}