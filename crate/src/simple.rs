#![allow(non_snake_case)]
use crate::generated::css_classes::C;

use comp_state::{topo, use_state};
use seed::dom_types::UpdateEl;
use seed::prelude::*;
use seed::*;
use seed_comp_helpers::on_click;

use super::Msg;

pub fn view() -> Node<Msg> {
    div![
        render_button(),
        render_button(),
        render_button(),
        render_button(),
        render_button(),
        render_button(),
        render_button(),
        render_button(),
        render_button(),
    ]
}

fn render_button() -> Node<Msg> {
    topo::call!({
        let (count, count_access) = use_state(|| 0);
        button![
            class![C.p_4, C.bg_gray_4, C.rounded, C.shadow, C.m_4],
            format!("Pressed {} times", count),
            on_click(move |_| count_access.set(count + 1))
        ]
    })
}
