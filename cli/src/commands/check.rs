use super::OutputFormat;
use acari_lib::{AcariError, Account, Client, User};
use prettytable::{format, table};
use serde_json::json;

pub fn check(client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
  let account = client.get_account()?;
  let user = client.get_myself()?;

  match output_format {
    OutputFormat::Pretty => print_pretty(account, user),
    OutputFormat::Json => print_json(account, user)?,
    OutputFormat::Flat => print_flat(account, user),
  }

  Ok(())
}

fn print_pretty(account: Account, user: User) {
  let mut account_table = table!(
    ["Id", account.id],
    ["Name", account.name],
    ["Title", account.title],
    ["Currency", account.currency],
    ["Created at", account.created_at.to_string()]
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
    ["Created at", user.created_at.to_string()]
  );

  println!();
  println!("User");
  user_table.set_format(*format::consts::FORMAT_CLEAN);
  user_table.printstd();
}

fn print_json(account: Account, user: User) -> Result<(), AcariError> {
  println!(
    "{}",
    serde_json::to_string_pretty(&json!({
      "account": account,
      "user": user,
    }))?
  );

  Ok(())
}

fn print_flat(account: Account, user: User) {
  println!("{}", account.name);
  println!("{}", user.name);
}
