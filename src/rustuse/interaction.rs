use anyhow::Result;
use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};

pub(crate) fn select_one(prompt: &str, items: &[&str]) -> Result<usize> {
    Ok(Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()?)
}

pub(crate) fn select_many(prompt: &str, items: &[&str]) -> Result<Vec<usize>> {
    Ok(MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .interact()?)
}

pub(crate) fn confirm(prompt: &str, default: bool) -> Result<bool> {
    Ok(Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()?)
}

pub(crate) fn input(prompt: &str, default: Option<&str>) -> Result<String> {
    let mut input = Input::<String>::with_theme(&ColorfulTheme::default()).with_prompt(prompt);

    if let Some(default) = default {
        input = input.default(default.to_owned());
    }

    Ok(input.interact_text()?)
}
