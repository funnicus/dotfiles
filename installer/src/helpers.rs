use indicatif::ProgressBar;
use inquire::{Confirm, Select};

/// Prompts the user with a spinner and a confirmation message.
/// Returns `true` if the user confirms, `false` otherwise.
///
/// # Arguments
///
/// * `spinner` - The progress bar spinner to display.
/// * `message` - The confirmation message to display.
/// * `default` - The default value to use if the user does not provide an input.
pub fn confirm_with_spinner(
    spinner: &ProgressBar,
    message: &str,
    default: bool,
) -> anyhow::Result<bool> {
    if is_non_interactive() {
        return Ok(default);
    }

    spinner
        .suspend(|| Confirm::new(message).with_default(default).prompt())
        .map_err(|err| anyhow::anyhow!("Prompt failed: {err}"))
}

/// Prompts the user with a spinner and a selection message.
/// Returns the user's selection.
///
/// # Arguments
///
/// * `spinner` - The progress bar spinner to display.
/// * `message` - The selection message to display.
/// * `choices` - The list of choices to display to the user.
pub fn select_with_spinner<T>(
    spinner: &ProgressBar,
    message: &str,
    choices: Vec<T>,
) -> anyhow::Result<T>
where
    T: std::fmt::Display + Clone,
{
    if is_non_interactive() {
        return match choices.first() {
            Some(choice) => Ok(choice.clone()),
            None => anyhow::bail!("No choices available"),
        };
    }

    spinner
        .suspend(|| Select::new(message, choices).prompt())
        .map_err(|err| anyhow::anyhow!("Prompt failed: {err}"))
}

/// Returns `true` if the current process is running in a non-interactive mode.
/// Checks for `CI` and `DRY_RUN` environment variables, and also if stdin is not a terminal.
pub fn is_non_interactive() -> bool {
    std::env::var("CI").is_ok()
        || std::env::var("DRY_RUN").is_ok()
        || !std::io::IsTerminal::is_terminal(&std::io::stdin())
}
