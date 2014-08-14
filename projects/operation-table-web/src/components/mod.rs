use crate::config::use_operation_table;
use dioxus::prelude::*;
use dioxus_katex::use_katex_display;
use operation_table::OperationTable;

pub fn TableEditor(cx: Scope) -> Element {
    let table = use_operation_table(cx);
    let table_tex = table.get_tex();
    let show_base = table.show_base_toggle();
    // let skip_one = table.skip_one_toggle();
    let base = table.base_slider();
    // katex render
    let katex = use_katex_display(&cx);
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
        // div {
        //     class: "form-control",
        //     skip_one
        // }
        div {
            class: "form-control",
            base
        }
    ))
}
