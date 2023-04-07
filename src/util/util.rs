
use super::super::pages::map::ImageCoords;

pub fn imagecoordshandler() -> Vec<ImageCoords>{
  let mut imagecoords = vec![];
  imagecoords.push(
    ImageCoords{
      month: "March 30th".to_string(), 
      image: "static/mar30/pic1.jpg".to_string(),
      notes: "Graffiti above a burnt out car - NOICE ONE MATE".to_string(),
      position: (810,1450)
    }
  );
  imagecoords.push(
    ImageCoords{
      month: "March 30th".to_string(), 
      image: "static/mar30/pic2.png".to_string(),
      notes: "Don't walk past this tent in front of the dildo bazaar if you don't want to be dosed with meth".to_string(),
      position: (820,1452)
    }
  );
  imagecoords.push(
    ImageCoords{
      month: "March 30th".to_string(), 
      image: "static/mar30/pic3.png".to_string(),
      notes: "Gang bangers dealing and taking up the whole sidewalk so it's not safe to walk past...the fuck".to_string(),
      position: (833,1463)
    }
  );
  imagecoords.push(
    ImageCoords{
      month: "March 30th".to_string(), 
      image: "static/mar30/pic4.png".to_string(),
      notes: "Looking up Hyde street - entire sidewalk taken up by meth addicts and homeless".to_string(),
      position: (833,1500)
    }
  );
  imagecoords.push(
    ImageCoords{
      month: "March 30th".to_string(), 
      image: "static/mar30/pic5.png".to_string(),
      notes: "Eddy and Tailor - dealers on the corner in front of tents, sidewalk not safe to walk down".to_string(),
      position: (815,1570)
    }
  );
  imagecoords.push(
    ImageCoords{
      month: "March 5th".to_string(), 
      image: "static/apr5/outlet.jpg".to_string(),
      notes: "The ferry building has one of the few publicly available power outlets in all of SF. If they didn't need to power wash the floors it wouldn't exist.".to_string(),
      position: (590,1845)
    }
  );
  imagecoords.push(
    ImageCoords{
      month: "March 5th".to_string(), 
      image: "static/apr5/bridge.jpg".to_string(),
      notes: "Picture of the Golden Gate Bridge during early evening from the Ferry Building".to_string(),
      position: (600,1870)
    }
  );
  imagecoords
}