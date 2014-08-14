use dioxus::{core_macro::rsx, events::FormEvent, prelude::*};
use operation_table::OperationTable;
use std::{cell::RefCell, rc::Rc, str::FromStr, sync::Arc};

#[derive(Clone)]
pub struct UseOperationTable {
    table: Rc<RefCell<TableOptions>>,
    updater: Arc<dyn Fn() + 'static>,
}

pub struct TableOptions {
    uppercase: bool,
    config: OperationTable,
}

impl Default for TableOptions {
    fn default() -> Self {
        Self { uppercase: true, config: OperationTable::default() }
    }
}

pub fn use_operation_table(cx: &ScopeState) -> &mut UseOperationTable {
    let options = TableOptions::default();
    let katex = UseOperationTable { table: Rc::new(RefCell::new(options)), updater: cx.schedule_update() };
    cx.use_hook(|| katex)
}

impl UseOperationTable {
    pub fn get_tex(&self) -> String {
        self.table.borrow().config.display().to_string()
    }
    pub fn on_alpha_input(&self) -> impl Fn(FormEvent) {
        let copy = self.clone();
        move |e| {
            let mut v = copy.table.borrow_mut();
            // copy.table.borrow().config.
            todo!()
        }
    }
}

impl UseOperationTable {
    pub fn show_base(&self) -> bool {
        self.table.borrow().config.show_base
    }
    /// Get the current value of the color option.
    pub fn show_base_toggle(&self) -> LazyNodes {
        let click = move |e: FormEvent| {
            let mut v = self.table.borrow_mut();
            match e.value.as_str() {
                "true" => v.config.show_base = true,
                "false" => v.config.show_base = false,
                _ => {}
            }
            self.needs_update();
        };
        let v = self.table.borrow().config.show_base;
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "Show display base"
                }
                input {
                    r#type: "checkbox",
                    class: "toggle",
                    checked: "{v}",
                    oninput: click
                }
            }
        )
    }
    pub fn uppercase(&self) -> bool {
        self.table.borrow().config.show_base
    }
    /// Get the current value of the color option.
    pub fn uppercase_toggle(&self) -> LazyNodes {
        let click = move |e: FormEvent| {
            let mut v = self.table.borrow_mut();
            match e.value.as_str() {
                "true" => v.config.show_base = true,
                "false" => v.config.show_base = false,
                _ => {}
            }
            self.needs_update();
        };
        let v = self.table.borrow().config.show_base;
        rsx!(
            label {
                class: "cursor-pointer label",
                span {
                    class: "label-text",
                    "Show display base"
                }
                input {
                    r#type: "checkbox",
                    class: "toggle",
                    checked: "{v}",
                    oninput: click
                }
            }
        )
    }
    /// Get the current value of the color option.
    pub fn get_base(&self) -> usize {
        self.table.borrow().config.base
    }
    /// Add a toggle button for the ellipsis option.
    pub fn base_slider(&self) -> LazyNodes {
        let click = move |e: FormEvent| {
            match usize::from_str(e.value.as_str()) {
                Ok(v) => {
                    self.table.borrow_mut().config.base = v;
                }
                Err(e) => {
                    log::warn!("range {:?}", e);
                }
            }
            self.needs_update();
        };
        let v = self.table.borrow().config.base;
        rsx!(
            label {
                class: "label flex",
                span {
                    class: "mr-auto label-text",
                    "Base"
                }
                input {
                    class: "range range-sm",
                    style: "width: 200px;",
                    r#type: "range",
                    min: "2",
                    max: "36",
                    step: "1",
                    value: "{v}",
                    onchange: click,
                }
            }
        )
    }
    pub fn base_display(&self) -> usize {
        self.table.borrow().config.base_display
    }
    pub fn base_display_slider(&self) -> LazyNodes {
        let click = move |e: FormEvent| {
            match usize::from_str(e.value.as_str()) {
                Ok(v) => {
                    self.table.borrow_mut().config.base_display = v;
                }
                Err(e) => {
                    log::warn!("range {:?}", e);
                }
            }
            self.needs_update();
        };
        let v = self.table.borrow().config.base_display;
        rsx!(
            label {
                class: "label flex",
                span {
                    class: "mr-auto label-text",
                    "Display base"
                }
                input {
                    class: "range range-sm",
                    style: "width: 200px;",
                    r#type: "range",
                    min: "2",
                    max: "60",
                    step: "1",
                    value: "{v}",
                    onchange: click,
                }
            }
        )
    }
    /// Force a re-render of the component.
    pub fn needs_update(&self) {
        (self.updater)()
    }
}
