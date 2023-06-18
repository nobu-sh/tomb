use dotenv::dotenv;
use std::{
  env,
  fmt::Debug,
  str::FromStr,
};

#[derive(Clone)]
pub struct Config {
  pub port: u16,
  pub temp: String,
}

impl Config {
  pub fn from_env() -> Self {
    dotenv().ok();
    Self {
      port: get_env("PORT"),
      temp: get_env_or("TEMP", "default value".to_string()),
    }
  }
}

fn get_env_or<T>(var: &str, default: T) -> T
where
  T: FromStr,
  <T as FromStr>::Err: Debug,
{
  match env::var(var) {
    Ok(v) => v.parse::<T>().expect(&format!(
      "Unable to parse {} as {}",
      var,
      std::any::type_name::<T>()
    )),
    Err(_) => default,
  }
}


fn get_env<T>(var: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    env::var(var)
        .expect(&format!("Missing environment variable {}", var))
        .parse::<T>()
        .expect(&format!(
            "Unable to parse {} as {}",
            var,
            std::any::type_name::<T>()
        ))
}
