use fennec_feedback::create_progress_bar;
use fennec_feedback::remove_progress_bar;
use fennec_feedback::ProgressBarTheme;
use fennec_interner::ThreadedInterner;
use fennec_linter::plugin::best_practices::BestPracticesPlugin;
use fennec_linter::plugin::comment::CommentPlugin;
use fennec_linter::plugin::consistency::ConsistencyPlugin;
use fennec_linter::plugin::naming::NamingPlugin;
use fennec_linter::plugin::redundancy::RedundancyPlugin;
use fennec_linter::plugin::safety::SafetyPlugin;
use fennec_linter::plugin::strictness::StrictnessPlugin;
use fennec_linter::plugin::symfony::SymfonyPlugin;
use fennec_linter::settings::RuleSettings;
use fennec_linter::settings::Settings;
use fennec_linter::Linter;
use fennec_reporting::Issue;
use fennec_reporting::IssueCollection;
use fennec_reporting::Level;
use fennec_semantics::Semantics;
use fennec_source::error::SourceError;
use fennec_source::SourceIdentifier;
use fennec_source::SourceManager;

use crate::linter::config::LinterConfiguration;
use crate::linter::config::LinterLevel;
use crate::linter::result::LintResult;

pub mod config;
pub mod result;

#[derive(Debug)]
pub struct LintService {
    configuration: LinterConfiguration,
    interner: ThreadedInterner,
    source_manager: SourceManager,
}

impl LintService {
    pub fn new(configuration: LinterConfiguration, interner: ThreadedInterner, source_manager: SourceManager) -> Self {
        Self { configuration, interner, source_manager }
    }

    /// Runs the linting process and returns a stream of issues.
    pub async fn run(&self) -> Result<LintResult, SourceError> {
        let source_ids: Vec<_> = self.source_manager.source_ids().collect();

        // Initialize the linter
        let linter = self.initialize_linter();

        // Filter source ids based on linter settings
        let filter_source_ids = self.filter_semantics(&linter, source_ids);

        // Process sources concurrently
        self.process_sources(linter, filter_source_ids).await
    }

    #[inline]
    async fn process_sources(
        &self,
        linter: Linter,
        source_ids: Vec<SourceIdentifier>,
    ) -> Result<LintResult, SourceError> {
        let mut handles = Vec::with_capacity(source_ids.len());

        let source_pb = create_progress_bar(source_ids.len(), "📂  Loading", ProgressBarTheme::Red);
        let semantics_pb = create_progress_bar(source_ids.len(), "🔬  Building", ProgressBarTheme::Blue);
        let lint_pb = create_progress_bar(source_ids.len(), "🧹  Linting", ProgressBarTheme::Cyan);

        for source_id in source_ids {
            handles.push(tokio::spawn({
                let interner = self.interner.clone();
                let manager = self.source_manager.clone();
                let linter = linter.clone();
                let source_pb = source_pb.clone();
                let semantics_pb = semantics_pb.clone();
                let lint_pb = lint_pb.clone();

                async move {
                    // Step 1: load the source
                    let source = manager.load(source_id).await?;
                    source_pb.inc(1);

                    // Step 2: build semantics
                    let semantics = Semantics::build(&interner, source);
                    semantics_pb.inc(1);

                    // Step 3: Collect issues
                    let mut issues = linter.lint(&semantics);
                    issues.extend(semantics.issues);
                    if let Some(error) = &semantics.parse_error {
                        issues.push(Into::<Issue>::into(error));
                    }
                    lint_pb.inc(1);

                    Result::<_, SourceError>::Ok(issues)
                }
            }));
        }

        let mut results = Vec::with_capacity(handles.len());
        for handle in handles {
            results.push(handle.await.expect("failed to collect issues. this should never happen.")?);
        }

        remove_progress_bar(source_pb);
        remove_progress_bar(semantics_pb);
        remove_progress_bar(lint_pb);

        Ok(LintResult::new(IssueCollection::from(results.into_iter().flatten())))
    }

    #[inline]
    fn initialize_linter(&self) -> Linter {
        let mut settings = Settings::new();

        if let Some(level) = self.configuration.level {
            settings = match level {
                LinterLevel::Off => settings.off(),
                LinterLevel::Help => settings.with_level(Level::Help),
                LinterLevel::Note => settings.with_level(Level::Note),
                LinterLevel::Warning => settings.with_level(Level::Warning),
                LinterLevel::Error => settings.with_level(Level::Error),
            };
        }

        if let Some(external) = self.configuration.external {
            settings = settings.with_external(external);
        }

        if let Some(default_plugins) = self.configuration.default_plugins {
            settings = settings.with_default_plugins(default_plugins);
        }

        settings = settings.with_plugins(self.configuration.plugins.clone());

        for rule in &self.configuration.rules {
            let mut rule_settings = match rule.level {
                Some(linter_level) => match linter_level {
                    LinterLevel::Off => RuleSettings::disabled(),
                    LinterLevel::Help => RuleSettings::from_level(Some(Level::Help)),
                    LinterLevel::Note => RuleSettings::from_level(Some(Level::Note)),
                    LinterLevel::Warning => RuleSettings::from_level(Some(Level::Warning)),
                    LinterLevel::Error => RuleSettings::from_level(Some(Level::Error)),
                },
                None => RuleSettings::enabled(),
            };

            if let Some(options) = rule.options.clone() {
                rule_settings = rule_settings.with_options(options.clone());
            }

            settings = settings.with_rule(rule.name.clone(), rule_settings);
        }

        let mut linter = Linter::new(settings, self.interner.clone());

        linter.add_plugin(BestPracticesPlugin);
        linter.add_plugin(CommentPlugin);
        linter.add_plugin(ConsistencyPlugin);
        linter.add_plugin(NamingPlugin);
        linter.add_plugin(RedundancyPlugin);
        linter.add_plugin(SafetyPlugin);
        linter.add_plugin(StrictnessPlugin);
        linter.add_plugin(SymfonyPlugin);

        linter
    }

    #[inline]
    fn filter_semantics(&self, linter: &Linter, source_ids: Vec<SourceIdentifier>) -> Vec<SourceIdentifier> {
        if !linter.settings.external {
            source_ids.into_iter().filter(|s| !s.is_external()).collect()
        } else {
            source_ids
        }
    }
}
