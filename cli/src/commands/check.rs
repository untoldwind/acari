use crate::config::Config;
use crate::error::AppError;
use prettytable::{cell, format, row, table};

pub fn check(config: &Config) -> Result<(), AppError> {
  let client = config.client();
  let account = client.get_account()?;
  let user = client.get_myself()?;

  let mut account_table = table!(
    ["Id", account.id.to_string()],
    ["Name", account.name],
    ["Title", account.title],
    ["Currency", account.currency],
    ["Created at", account.created_at.to_string()],
    ["Updated at", account.updated_at.to_string()]
  );

  println!("Account");
  account_table.set_format(*format::consts::FORMAT_CLEAN);
  account_table.printstd();

  let mut user_table = table!(
    ["Id", user.id.to_string()],
    ["Name", user.name],
    ["Email", user.email],
    ["Role", user.role],
    ["Language", user.language],
    ["Created at", user.created_at.to_string()],
    ["Updated at", user.updated_at.to_string()]
  );

  println!();
  println!("User");
  user_table.set_format(*format::consts::FORMAT_CLEAN);
  user_table.printstd();
  Ok(())
}
