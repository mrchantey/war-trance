use bevy::prelude::*;
use gamai::*;
// use gamai::common_actions::*;
use crate::*;
use gamai::common_selectors::*;

#[tree_builder]
pub fn AgentTree() -> impl AiNode {
    tree! {
        <sequence>
            <seek_target before_parent=seek_enemy_scorer/>
        </sequence>
    }
}

pub fn agent_tree_bundle() -> impl Bundle {
    (
        TreeBundle::root(AgentTree, Running),
        TreeBundle::recursive(AgentTree, SeekTarget::Position(Vec3::ZERO)),
        TreeBundle::recursive(AgentTree, Score::Fail),
    )
}
