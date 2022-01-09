use druid::im::Vector;
use druid::widget::{
    Checkbox, Controller, Flex, Label, LensWrap, List, MainAxisAlignment, Padding, Scroll, Switch,
    TextBox, WidgetExt,
};
use druid::{
    AppDelegate, AppLauncher, Color, Data, Handled, Lens, LocalizedString, Selector, Widget,
    WindowDesc,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, Lens, Debug)]
struct AppState {
    pub checked: bool,
    pub on: bool,
    pub stepper_value: f64,
    pub selected: Vector<String>,
    pub search: String,
    pub raw_sources: Vector<SourceItem>,
    pub filtered_sources: Vector<SourceItem>,
}

impl AppState {
    fn filter(&mut self) -> &mut Self {
        if self.has_filter() {
            self.filtered_sources = self
                .raw_sources
                .iter()
                .filter(|s| s.text.contains(&self.search))
                .map(|s| SourceItem {
                    text: s.text.to_string(),
                    value: s.value.to_string(),
                    checked: self.selected.contains(&s.value),
                })
                .collect();
        } else {
            self.filtered_sources = self
                .raw_sources
                .iter()
                .map(|s| SourceItem {
                    text: s.text.to_string(),
                    value: s.value.to_string(),
                    checked: self.selected.contains(&s.value),
                })
                .collect();
        }

        self.filtered_sources = self
            .filtered_sources
            .iter()
            .map(|s| SourceItem {
                text: s.text.to_string(),
                value: s.value.to_string(),
                checked: self.selected.contains(&s.value),
            })
            .collect();

        self
    }

    fn on_toggle_source(&mut self, _checked: bool) -> &mut Self {
        self.filtered_sources.iter().for_each(|s1| {
            if s1.checked {
                if !self.selected.contains(&s1.value) {
                    self.selected.push_back(s1.value.to_string());
                }
            } else {
                if self.selected.contains(&s1.value) {
                    let idx = self.selected.iter().position(|i| *i == s1.value).unwrap();
                    self.selected.remove(idx);
                }
            }
        });

        println!("{:?}", self.selected);
        self
    }

    fn has_filter(&mut self) -> bool {
        self.search.chars().count() > 0
    }
}

#[derive(Clone, Data, Lens, Serialize, Deserialize, Debug)]
struct SourceItem {
    pub checked: bool,
    pub text: String,
    pub value: String,
}

#[derive(Debug)]
struct UpdateCallback();

impl Controller<String, TextBox<String>> for UpdateCallback {
    fn update(
        &mut self,
        child: &mut TextBox<String>,
        ctx: &mut druid::UpdateCtx,
        old_data: &String,
        data: &String,
        env: &druid::Env,
    ) {
        if old_data != data {
            ctx.submit_command(FILTER);
        }

        child.update(ctx, old_data, data, env)
    }
}

impl Controller<bool, Checkbox> for UpdateCallback {
    fn update(
        &mut self,
        child: &mut Checkbox,
        ctx: &mut druid::UpdateCtx,
        old_data: &bool,
        data: &bool,
        env: &druid::Env,
    ) {
        if old_data != data {
            ctx.submit_command(if data == &true {
                SELECT_SOURCE
            } else {
                REMOVE_SOURCE
            });
        }

        child.update(ctx, old_data, data, env)
    }
}

pub struct Delegate;

const FILTER: Selector = Selector::new("source.filter");
const SELECT_SOURCE: Selector = Selector::new("source.toggle.add");
const REMOVE_SOURCE: Selector = Selector::new("source.toggle.remove");

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        if cmd.is(FILTER) {
            data.filter();
            Handled::Yes
        } else if cmd.is(SELECT_SOURCE) {
            data.on_toggle_source(true);
            Handled::Yes
        } else if cmd.is(REMOVE_SOURCE) {
            data.on_toggle_source(false);
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

fn build_widget() -> impl Widget<AppState> {
    let mut col = Flex::column();

    let mut switch_row = Flex::row();
    let switch = LensWrap::new(Switch::new(), AppState::on);
    let check_box = LensWrap::new(Checkbox::new(""), AppState::checked);
    let switch_label = Label::new("Setting 标签");

    let switch = switch.on_click(|_ctx, state, _| {
        state.on = !state.on;
        state.checked = !state.on;
    });

    switch_row.add_child(Padding::new(5.0, switch_label));
    switch_row.add_child(Padding::new(5.0, switch));
    switch_row.add_child(Padding::new(5.0, check_box));
    col.add_child(Padding::new(5.0, switch_row));

    // let stepper = LensWrap::new(
    //     Stepper::new()
    //         .with_range(0.0, 10.0)
    //         .with_step(0.5)
    //         .with_wraparound(false),
    //     AppState::stepper_value,
    // );
    // let mut textbox_row = Flex::row();
    // let textbox = LensWrap::new(
    //     Parse::new(TextBox::new()),
    //     AppState::stepper_value.map(|x| Some(*x), |x, y| *x = y.unwrap_or(0.0)),
    // );
    // textbox_row.add_child(Padding::new(5.0, textbox));
    // textbox_row.add_child(Padding::new(5.0, stepper.center()));
    // col.add_child(Padding::new(5.0, textbox_row));

    // let mut label_row = Flex::row();
    // let label = Label::new(|data: &AppState, _env: &_| {
    //     format!("Stepper value: {0:.2}", data.stepper_value)
    // });
    // label_row.add_child(Padding::new(5.0, label));
    // col.add_child(Padding::new(5.0, label_row));

    let search_box = TextBox::new()
        .with_placeholder("搜索")
        .controller(UpdateCallback())
        .lens(AppState::search);
    let mut search_col = Flex::row().with_child(Padding::new(5.0, search_box));
    search_col.set_main_axis_alignment(MainAxisAlignment::Center);
    // search_col.set_must_fill_main_axis(true);

    col.add_flex_child(search_col, 1.0);
    col.set_main_axis_alignment(MainAxisAlignment::Start);
    col.add_flex_child(build_list(), 1.0);
    col.center()
}

fn build_list() -> impl Widget<AppState> {
    let list = List::new(build_item).lens(AppState::filtered_sources);
    let col = Flex::column()
        .must_fill_main_axis(true)
        .with_child(list)
        .with_default_spacer();

    Scroll::new(col)
        .vertical()
        .expand_height()
        .background(Color::PURPLE)
}

fn build_item() -> impl Widget<SourceItem> {
    let cb = Checkbox::new("")
        .controller(UpdateCallback())
        .lens(SourceItem::checked);
    let label = Label::raw().lens(SourceItem::text);
    Flex::row()
        .with_child(cb)
        .with_child(label)
        .with_default_spacer()
}

pub fn main() {
    let window = WindowDesc::new(build_widget)
        .title(LocalizedString::new("switch-demo-window-title").with_placeholder("Switch Demo"));

    let window = window.window_size((320., 540.));

    let mut selected = vec![];
    selected.push(String::from("source-1"));
    selected.push(String::from("source-2"));

    let mut sources = vec![];
    let mut raw = vec![];
    for i in 1..30 {
        let text = format!("source {}", i);
        let value = &format!("source-{}", i);
        sources.push(SourceItem {
            text,
            value: String::from(value),
            checked: false,
        });
        raw.push(value.to_string());
    }

    let sources = Vector::from(sources);
    let filtered_sources = sources
        .iter()
        .map(|s| {
            let checked = selected.contains(&s.value);
            SourceItem {
                text: s.text.to_string(),
                value: s.value.to_string(),
                checked,
            }
        })
        .collect();

    AppLauncher::with_window(window)
        .use_simple_logger()
        .delegate(Delegate {})
        .launch(AppState {
            checked: false,
            on: true,
            stepper_value: 1.0,
            search: String::new(),
            selected: Vector::from(selected),
            raw_sources: sources,
            filtered_sources,
        })
        .expect("launch failed");
}
