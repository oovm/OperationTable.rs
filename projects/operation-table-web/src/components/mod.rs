use crate::config::use_operation_table;
use dioxus::prelude::*;
use dioxus_katex::use_katex_display;
use operation_table::OperationTable;

pub fn BMSEditor(cx: Scope) -> Element {
    let table = use_operation_table(cx);
    let mut config = OperationTable::default();
    // config.ellipsis = dayan.ellipsis();
    // initial value
    // let place_holder = r#"(0, 0, 0)(1, 1, 1)(2, 1, 0)"#;
    // let current_text = use_state(&cx, || place_holder.to_string());
    let mut table_tex = table.get_tex();
    // bms.set_expand_steps(dayan.expands());
    // let e_bms = bms.expand();
    let show_base = table.show_base_toggle();
    let skip_one = table.skip_one_toggle();
    let base = table.base_slider();
    // let update_bms = dayan.on_alpha_input();
    // katex render
    let katex = use_katex_display(&cx);
    // config.display = false;
    // let bms_inline = config.render(&e_bms);
    let math_inline = katex.compile(&table_tex);
    cx.render(rsx!(
        div {
            class: "flex-1 ml-2 mr-2",
            h3 {
                "Display Math:"
            }
            pre {
                class: "text-sm",
                "{table_tex}"
            }
            math_inline
        }
        div {
            class: "form-control",
            show_base
        }
        div {
            class: "form-control",
            skip_one
        }
        div {
            class: "form-control",
            base
        }
    ))
}
