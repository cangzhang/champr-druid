use druid::im::Vector;
use druid::widget::{
    Checkbox, Controller, Flex, Label, LensWrap, List, MainAxisAlignment, Padding, Parse, Scroll,
    Stepper, Switch, TextBox, WidgetExt,
};
use druid::{
    AppDelegate, AppLauncher, Data, Handled, Lens, LensExt, LocalizedString, Selector, Widget,
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
    pub sources: Vector<SourceItem>,
    pub filtered_sources: Vector<SourceItem>,
}

impl AppState {
    fn filter(&mut self) -> &mut Self {
        if self.search.chars().count() == 0 {
            self.filtered_sources = self.sources.clone();
        } else {
            self.filtered_sources = self
                .sources
                .clone()
                .into_iter()
                .filter(|s| s.text.contains(&self.search))
                .collect();
        }

        self
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

pub struct Delegate;

const FILTER: Selector = Selector::new("source.filter");

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        target: druid::Target,
        cmd: &druid::Command,
        data: &mut AppState,
        env: &druid::Env,
    ) -> druid::Handled {
        if cmd.is(FILTER) {
            data.filter();
            Handled::Yes
        } else {
            Handled::No
        }
    }
}

fn build_widget() -> impl Widget<AppState> {
    let mut col = Flex::column();
    let mut row = Flex::row();
    let switch = LensWrap::new(Switch::new(), AppState::on);
    let check_box = LensWrap::new(Checkbox::new(""), AppState::checked);
    let switch_label = Label::new("Setting 标签");

    let switch = switch.on_click(|_ctx, state, _| {
        state.on = !state.on;
        state.checked = !state.on;
    });

    row.add_child(Padding::new(5.0, switch_label));
    row.add_child(Padding::new(5.0, switch));
    row.add_child(Padding::new(5.0, check_box));

    let stepper = LensWrap::new(
        Stepper::new()
            .with_range(0.0, 10.0)
            .with_step(0.5)
            .with_wraparound(false),
        AppState::stepper_value,
    );

    let mut textbox_row = Flex::row();
    let textbox = LensWrap::new(
        Parse::new(TextBox::new()),
        AppState::stepper_value.map(|x| Some(*x), |x, y| *x = y.unwrap_or(0.0)),
    );
    textbox_row.add_child(Padding::new(5.0, textbox));
    textbox_row.add_child(Padding::new(5.0, stepper.center()));

    let mut label_row = Flex::row();
    let label = Label::new(|data: &AppState, _env: &_| {
        format!("Stepper value: {0:.2}", data.stepper_value)
    });

    label_row.add_child(Padding::new(5.0, label));

    col.set_main_axis_alignment(MainAxisAlignment::Start);
    col.add_child(Padding::new(5.0, row));
    col.add_child(Padding::new(5.0, textbox_row));
    col.add_child(Padding::new(5.0, label_row));

    let search_box = TextBox::new()
        .with_placeholder("搜索")
        .controller(UpdateCallback())
        .lens(AppState::search);
    let mut search_col = Flex::row().with_child(Padding::new(5.0, search_box));
    search_col.set_main_axis_alignment(MainAxisAlignment::Center);
    search_col.set_must_fill_main_axis(true);
    col.add_flex_child(search_col, 1.0);

    col.add_flex_child(build_list(), 1.0);
    col.center()
}

fn build_list() -> impl Widget<AppState> {
    let list = List::new(build_item).lens(AppState::filtered_sources);

    Scroll::new(
        Flex::column()
            .must_fill_main_axis(true)
            .with_child(list)
            .with_default_spacer(),
    )
    .vertical()
    .expand_height()
}

fn build_item() -> impl Widget<SourceItem> {
    let cb = Checkbox::new("").lens(SourceItem::checked);
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
    for i in 1..30 {
        let text = format!("source {}", i);
        let value = format!("source-{}", i);
        let checked = selected.contains(&value);
        sources.push(SourceItem {
            text,
            value,
            checked,
        });
    }

    AppLauncher::with_window(window)
        .use_simple_logger()
        .delegate(Delegate {})
        .launch(AppState {
            checked: false,
            on: true,
            stepper_value: 1.0,
            search: String::new(),
            selected: Vector::from(selected),
            sources: Vector::from(sources),
            filtered_sources: Vector::new(),
        })
        .expect("launch failed");
}
