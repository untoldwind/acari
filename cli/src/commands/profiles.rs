use crate::config::Config;

pub fn profiles(config: Config) {
  for profile in config.profiles.keys() {
    println!("{}", profile);
  }
}
