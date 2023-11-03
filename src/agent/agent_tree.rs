use crate::*;
use gamai::common_selectors::*;
use gamai::*;

#[tree_builder]
pub fn AgentTree() -> impl TreeElement {
	tree! {
		<repeat>
			<highest_score apply_deferred>
				<group actions=(ranged_attack,ranged_attack_scorer) apply_deferred/>
				<group actions=(seek_target,seek_enemy_scorer)/>
				// <succeed_in_duration/>
			</highest_score>
		</repeat>
	}
}
