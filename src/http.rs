const HOST: &str = "https://adventofcode.com";

fn get_input_url(year: u16, day: u8) -> String {
  format!("{}/{}/day/{}/input", HOST, year, day)
}

pub fn fetch_input(auth_token: String, year: u16, day: u8) -> super::Result<String> {
  let client = reqwest::blocking::Client::new();
  let res = client
    .get(get_input_url(year, day))
    .header(reqwest::header::COOKIE, format!("session={}", auth_token))
    .send()?;

  let text = res.text()?;
  Ok(text)
}
