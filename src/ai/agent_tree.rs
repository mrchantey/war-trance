use crate::*;
use bevy::prelude::*;
use gamai::common_selectors::*;
use gamai::*;
use std::time::Duration;

#[tree_builder]
pub fn AgentTree() -> impl TreeElement {
    tree! {
        <highest_score>
            <seek_target before_parent=seek_enemy_scorer/>
            // <succeed_in_duration/>
        </highest_score>
    }
}

pub fn agent_tree_bundle() -> impl Bundle {
    (
        TreeBundle::root(AgentTree, Running),
        TreeBundle::recursive(AgentTree, SeekTarget::Position(Vec3::ZERO)),
        TreeBundle::recursive(AgentTree, Duration::from_secs(1)),
        TreeBundle::recursive(AgentTree, Score::Fail),
        TreeBundle::recursive(AgentTree, ActionTimer::default()),
    )
}
