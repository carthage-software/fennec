use rules::redundant_write_visibility::RedundantWriteVisibilityRule;

use crate::plugin::redundancy::rules::redundant_block::RedundantBlockRule;
use crate::plugin::redundancy::rules::redundant_closing_tag::RedudnantClosingTagRule;
use crate::plugin::redundancy::rules::redundant_continue::RedundantContinueRule;
use crate::plugin::redundancy::rules::redundant_final_method_modifier::RedundantFinalMethodModifierRule;
use crate::plugin::redundancy::rules::redundant_if_statement::RedundantIfStatementRule;
use crate::plugin::redundancy::rules::redundant_label::RedundantLabelRule;
use crate::plugin::redundancy::rules::redundant_method_override::RedundantMethodOverrideRule;
use crate::plugin::redundancy::rules::redundant_noop::RedundantNoopRule;
use crate::plugin::redundancy::rules::redundant_parentheses::RedundantParenthesesRule;
use crate::plugin::redundancy::rules::redundant_string_concat::RedundantStringConcatRule;
use crate::plugin::Plugin;
use crate::rule::Rule;

pub mod rules;

#[derive(Debug)]
pub struct RedundancyPlugin;

impl Plugin for RedundancyPlugin {
    fn get_name(&self) -> &'static str {
        "redundancy"
    }

    fn is_enabled_by_default(&self) -> bool {
        true
    }

    fn get_rules(&self) -> Vec<Box<dyn Rule>> {
        vec![
            Box::new(RedundantParenthesesRule),
            Box::new(RedundantBlockRule),
            Box::new(RedudnantClosingTagRule),
            Box::new(RedundantContinueRule),
            Box::new(RedundantStringConcatRule),
            Box::new(RedundantNoopRule),
            Box::new(RedundantMethodOverrideRule),
            Box::new(RedundantFinalMethodModifierRule),
            Box::new(RedundantLabelRule),
            Box::new(RedundantIfStatementRule),
            Box::new(RedundantWriteVisibilityRule),
        ]
    }
}
