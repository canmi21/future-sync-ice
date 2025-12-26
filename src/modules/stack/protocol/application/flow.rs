use super::container::Container;
use crate::modules::plugins::model::ProcessingStep;
use crate::modules::plugins::model::TerminatorResult;

pub(crate) async fn execute_l7(
    step: &ProcessingStep,
    container: &mut Container,
    parent_path: String,
) -> anyhow::Result<TerminatorResult> {
    loop { }
}
