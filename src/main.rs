use druid::im::Vector;
use druid::widget::{Checkbox, Flex, Label, LensWrap, List, MainAxisAlignment, Padding, Parse, Scroll, Stepper, Switch, TextBox, WidgetExt};
use druid::{AppLauncher, Data, Lens, LensExt, LocalizedString, Widget, WindowDesc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, Lens, Debug)]
struct AppState {
    pub checked: bool,
    pub on: bool,
    pub stepper_value: f64,
    pub sources: Vector<SourceItem>,
}

#[derive(Clone, Data, Lens, Serialize, Deserialize, Debug)]
struct SourceItem {
    pub checked: bool,
    pub text: String,
    // pub value: String,
}

fn build_widget() -> impl Widget<AppState> {
    let mut col = Flex::column();
    let mut row = Flex::row();
    let switch = LensWrap::new(Switch::new(), AppState::on);
    let check_box = LensWrap::new(Checkbox::new(""), AppState::checked);
    let switch_label = Label::new("Setting 标签");

    let switch = switch.on_click(|ctx, state, _| {
        state.on = !state.on;
        state.checked = !state.on;
    });

    row.with_child(Padding::new(5.0, switch_label))
        .with_child(Padding::new(5.0, switch))
        .with_child(Padding::new(5.0, check_box));

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
    textbox_row.with_child(Padding::new(5.0, textbox))
        .with_child(Padding::new(5.0, stepper.center()));

    let mut label_row = Flex::row();
    let label = Label::new(|data: &AppState, _env: &_| {
        format!("Stepper value: {0:.2}", data.stepper_value)
    });

    label_row.add_child(Padding::new(5.0, label));

    col.set_main_axis_alignment(MainAxisAlignment::Start);
    col.with_child(Padding::new(5.0, row))
        .with_child(Padding::new(5.0, textbox_row))
        .with_child(Padding::new(5.0, label_row))
        .with_flex_child(build_list(), 1.0)
        .center()
}

fn build_list() -> impl Widget<AppState> {
    Scroll::new(
        Flex::column()
            .must_fill_main_axis(true)
            .with_child(List::new(build_item).lens(AppState::sources))
            .with_default_spacer()
    )
        .vertical()
        .expand_height()
}

fn build_item() -> impl Widget<SourceItem> {
    let c = Checkbox::new("").lens(SourceItem::checked);
    let label = Label::raw().lens(SourceItem::text);
    Flex::row()
        .with_child(c)
        .with_child(label)
        .with_default_spacer()
}

pub fn main() {
    let window = WindowDesc::new(build_widget)
        .title(LocalizedString::new("switch-demo-window-title").with_placeholder("Switch Demo"));

    let window = window.window_size((320., 540.));

    let mut sources = vec![];
    for i in 1..30 {
        let text = format!("source {}", i);
        sources.push(SourceItem {
            checked: false,
            text,
        })
    }

    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(AppState {
            checked: false,
            on: true,
            stepper_value: 1.0,
            sources: Vector::from(sources),
        })
        .expect("launch failed");
}
