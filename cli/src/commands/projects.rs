use super::OutputFormat;
use crate::error::AppError;
use acari_lib::{Client, Project};
use itertools::Itertools;
use prettytable::{cell, format, row, Table};

pub fn all_projects(client: &dyn Client, output_format: OutputFormat) -> Result<(), AppError> {
  let mut projects = client.get_projects()?;

  projects.sort_by(|p1, p2| p1.customer_name.cmp(&p2.customer_name));

  let grouped: Vec<(&str, Vec<&Project>)> = projects
    .iter()
    .group_by(|p| p.customer_name.as_str())
    .into_iter()
    .map(|(customer_name, group)| (customer_name, group.collect()))
    .collect();

  match output_format {
    OutputFormat::Pretty => print_pretty(grouped),
    OutputFormat::Json => print_json(projects)?,
    OutputFormat::Flat => print_flat(grouped),
  }

  Ok(())
}

fn print_pretty(projects: Vec<(&str, Vec<&Project>)>) {
  let mut projects_table = Table::new();
  projects_table.set_titles(row!["Customer", "Project"]);
  projects_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

  for (customer_name, group) in projects {
    projects_table.add_row(row![customer_name, &group.iter().map(|p| p.name.as_str()).join("\n")]);
  }
  projects_table.printstd();
}

fn print_json(projects: Vec<Project>) -> Result<(), AppError> {
  println!("{}", serde_json::to_string_pretty(&projects)?);

  Ok(())
}

fn print_flat(projects: Vec<(&str, Vec<&Project>)>) {
  for (customer_name, group) in projects {
    for project in group {
      println!("{}/{}", customer_name, project.name);
    }
  }
}
