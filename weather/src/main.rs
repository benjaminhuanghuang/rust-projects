use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Input {
  city: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Coord {
  lon: f64,
  lat: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct W {
  coord: Coord,
  weather: Weather,
  base: String,
  mian: Main,
}
impl W {
  async fn get(city: &String) -> Result<Self, ExitFailure> {
    let url = format!(
      "https://api.openweathermap.org/data/2.5/weather?q={}&appid=d5a73e57807cdcb92ca1cd9be45b1045",
      city
    );
    let url = Url::parse(&*url)?;
    let resp: W = reqwest::get(url).await?.json::<W>().await?;

    Ok(resp)
  }
}

#[derive(Deserialize, Serialize, Debug)]
struct Weather {
  details: Details,
}

#[derive(Deserialize, Serialize, Debug)]
struct Details {
  id: i32,
  main: String,
  desription: String,
  icon: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Main {
  temp: f64,
  feel_like: f64,
  temp_min: f64,
  temp_max: f64,
  pressure: i32,
  humidity: i32,
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
  let input = Input::from_args();  //BeiJing
  println!("{}", input.city);

  let resp = W::get(&input.city).await;
  println!("{:?}", resp);


  Ok(())
}
