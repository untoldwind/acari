use super::OutputFormat;
use acari_lib::{AcariError, Client, Customer};
use prettytable::{cell, format, row, Table};

pub fn customers(client: &dyn Client, output_format: OutputFormat) -> Result<(), AcariError> {
  let mut customers = client.get_customers()?;

  customers.sort_by(|c1, c2| c1.name.cmp(&c2.name));

  match output_format {
    OutputFormat::Pretty => print_pretty(customers),
    OutputFormat::Json => print_json(customers)?,
    OutputFormat::Flat => print_flat(customers),
  }

  Ok(())
}

fn print_pretty(customers: Vec<Customer>) {
  let mut customers_table = Table::new();
  customers_table.set_titles(row!["Customers"]);
  customers_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

  for customer in customers {
    if customer.archived {
      customers_table.add_row(row![FY => customer.name]);
    } else {
      customers_table.add_row(row![customer.name]);
    }
  }
  customers_table.printstd();
}

fn print_json(customers: Vec<Customer>) -> Result<(), AcariError> {
  println!("{}", serde_json::to_string_pretty(&customers)?);

  Ok(())
}

fn print_flat(customers: Vec<Customer>) {
  for customer in customers {
    if !customer.archived {
      println!("{}", customer.name);
    }
  }
}
