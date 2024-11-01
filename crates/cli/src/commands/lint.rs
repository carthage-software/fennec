use clap::Parser;

use fennec_interner::ThreadedInterner;
use fennec_reporting::reporter::Reporter;
use fennec_service::config::Configuration;
use fennec_service::linter::LintService;
use fennec_service::source::SourceService;

use crate::utils::bail;

#[derive(Parser, Debug)]
#[command(
    name = "lint",
    about = "Lint the project according to the `fennec.toml` configuration or default settings",
    long_about = r#"
Lint the project according to the `fennec.toml` configuration or default settings.

This command analyzes the project's source code and highlights issues based on the defined linting rules.

If `fennec.toml` is not found, the default configuration is used. The command outputs the issues found in the project."
    "#
)]
pub struct LintCommand {
    #[arg(long, short, help = "Only show fixable issues", default_value_t = false)]
    pub only_fixable: bool,
}

pub async fn execute(command: LintCommand, configuration: Configuration) -> i32 {
    let interner = ThreadedInterner::new();

    let source_service = SourceService::new(interner.clone(), configuration.source);
    let source_manager = source_service.load().await.unwrap_or_else(bail);

    let lint_service = LintService::new(configuration.linter, interner.clone(), source_manager.clone());
    let lint_result = lint_service.run().await.unwrap_or_else(bail);
    let lint_result_has_errors = lint_result.has_errors();

    if command.only_fixable {
        Reporter::new(source_manager).report_all(lint_result.only_fixable()).await;
    } else {
        Reporter::new(source_manager).report_all(lint_result.into_iter()).await;
    }

    if lint_result_has_errors {
        1
    } else {
        0
    }
}
