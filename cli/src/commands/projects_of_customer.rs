use super::OutputFormat;
use acari_lib::{AcariError, Client, Project};
use prettytable::{cell, format, row, Table};

pub fn projects_of_customer(client: &dyn Client, output_format: OutputFormat, customer_name: &str) -> Result<(), AcariError> {
  let mut projects = client.get_projects()?;

  projects.retain(|p| p.customer_name == customer_name);
  projects.sort_by(|p1, p2| p1.customer_name.cmp(&p2.customer_name));

  match output_format {
    OutputFormat::Pretty => print_pretty(projects),
    OutputFormat::Json => print_json(projects)?,
    OutputFormat::Flat => print_flat(projects),
  }

  Ok(())
}

fn print_pretty(projects: Vec<Project>) {
  let mut projects_table = Table::new();
  projects_table.set_titles(row!["Projects"]);
  projects_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

  for project in projects {
    if project.archived {
      projects_table.add_row(row![FY => project.name]);
    } else {
      projects_table.add_row(row![project.name]);
    }
  }
  projects_table.printstd();
}

fn print_json(projects: Vec<Project>) -> Result<(), AcariError> {
  println!("{}", serde_json::to_string_pretty(&projects)?);

  Ok(())
}

fn print_flat(projects: Vec<Project>) {
  for project in projects {
    if !project.archived {
      println!("{}", project.name);
    }
  }
}
