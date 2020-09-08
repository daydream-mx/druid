// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of various text layout features.

use druid::widget::{Controller, Flex, Label, Scroll, Slider};
use druid::{
    AppLauncher, Color, Data, Env, Lens, LocalizedString, UpdateCtx, Widget, WidgetExt, WindowDesc,
};

const WINDOW_TITLE: LocalizedString<AppState> = LocalizedString::new("Text Examples");

const TEXT: &str = r#"The idea of "structurelessness," however, has moved from a healthy counter to those tendencies to becoming a goddess in its own right. The idea is as little examined as the term is much used, but it has become an intrinsic and unquestioned part of women's liberation ideology. For the early development of the movement this did not much matter. It early defined its main goal, and its main method, as consciousness-raising, and the "structureless" rap group was an excellent means to this end. The looseness and informality of it encouraged participation in discussion, and its often supportive atmosphere elicited personal insight. If nothing more concrete than personal insight ever resulted from these groups, that did not much matter, because their purpose did not really extend beyond this."#;

const DEFAULT_WRAP_WIDTH: f64 = 200.0;
const SPACER_SIZE: f64 = 8.0;

#[derive(Clone, Data, Lens)]
struct AppState {
    ///  the width at which to wrap lines.
    wrap_width: f64,
}

/// A controller that sets properties on a label.
struct LabelController;

impl Controller<AppState, Label<AppState>> for LabelController {
    #[allow(clippy::float_cmp)]
    fn update(
        &mut self,
        child: &mut Label<AppState>,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        env: &Env,
    ) {
        if old_data.wrap_width != data.wrap_width {
            child.set_wrap_width(data.wrap_width);
        }
        child.update(ctx, old_data, data, env);
    }
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((600.0, 800.0));

    // create the initial app state
    let initial_state = AppState {
        wrap_width: DEFAULT_WRAP_WIDTH,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    let label = Scroll::new(
        Label::new(TEXT)
            .with_wrap_width(DEFAULT_WRAP_WIDTH)
            .with_text_color(Color::BLACK)
            .controller(LabelController)
            .padding(SPACER_SIZE)
            .background(Color::WHITE),
    )
    .background(Color::grey8(222));

    let size_slider = Flex::row()
        .must_fill_main_axis(true)
        .with_spacer(SPACER_SIZE)
        .with_child(Label::new("Wrap width:"))
        .with_child(Slider::new().with_range(10.0, 1000.0))
        .with_child(Label::dynamic(|d: &f64, _| format!("{:.0}", d)))
        .lens(AppState::wrap_width);

    Flex::column()
        .with_spacer(SPACER_SIZE)
        .with_child(size_slider)
        .with_spacer(SPACER_SIZE)
        .with_flex_child(label, 1.0)
}
