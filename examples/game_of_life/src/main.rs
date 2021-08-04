use cell::Cellule;
use game::Game;
use gloo::timers::callback::Interval;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use yew::{html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};

mod cell;
mod game;

const DEFAULT_INTERVAL: usize = 33;
const DEFAULT_WIDTH: usize = 90;
const DEFAULT_HEIGHT: usize = 50;
const CANVAS_SIZE_FACTOR: usize = 8;
const COLOR_CELL_ALIVE: &str = "chocolate";
const COLOR_CELL_DEAD: &str = "#6d2f04";
const COLOR_GRID: &str = "#7d3605";

pub enum Msg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    ToggleCellule(usize),
    Test(String),
    SetIntervalInput(String),
    SetInterval,
    SetHeightInput(String),
    SetWidthInput(String),
    SetDimensions,
    Tick,
}

pub struct Model {
    link: ComponentLink<Self>,
    game: Game,
    canvas_ref: NodeRef,
    context_ref: Option<CanvasRenderingContext2d>,
    width_input: String,
    height_input: String,
    interval_input: String,
    interval_duration: usize,
    _interval: Interval,
}

impl Model {
    fn draw_cellules(&self, draw_all: bool) {
        let context = self.context_ref.as_ref().unwrap();
        let mut to_alive: Vec<(usize, usize)> = Vec::default();
        let mut to_dead: Vec<(usize, usize)> = Vec::default();
        for row in 0..self.game.cellules_height {
            for col in 0..self.game.cellules_width {
                let current_idx = self.game.row_col_as_idx(row as isize, col as isize);
                if draw_all
                    || self.game.cellules[current_idx] != self.game.prev_cellules[current_idx]
                {
                    if self.game.cellules[current_idx].is_alive() {
                        to_alive.push((col, row));
                    } else {
                        to_dead.push((col, row));
                    }
                }
            }
        }
        context.set_fill_style(&COLOR_CELL_ALIVE.into());
        to_alive.iter().for_each(|c| {
            context.fill_rect(
                (c.0 * CANVAS_SIZE_FACTOR) as f64,
                (c.1 * CANVAS_SIZE_FACTOR) as f64,
                CANVAS_SIZE_FACTOR as f64 - 1.0,
                CANVAS_SIZE_FACTOR as f64 - 1.0,
            )
        });
        context.set_fill_style(&COLOR_CELL_DEAD.into());
        to_dead.iter().for_each(|c| {
            context.fill_rect(
                (c.0 * CANVAS_SIZE_FACTOR) as f64,
                (c.1 * CANVAS_SIZE_FACTOR) as f64,
                CANVAS_SIZE_FACTOR as f64 - 1.0,
                CANVAS_SIZE_FACTOR as f64 - 1.0,
            )
        });
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let interval_duration = DEFAULT_INTERVAL;

        let callback = link.callback(|_| Msg::Tick);
        let interval = Interval::new(interval_duration as u32, move || callback.emit(()));

        let game = Game {
            active: false,
            cellules: vec![Cellule::new_dead(); DEFAULT_WIDTH * DEFAULT_HEIGHT],
            prev_cellules: vec![Cellule::new_dead(); DEFAULT_WIDTH * DEFAULT_HEIGHT],
            cellules_width: DEFAULT_WIDTH,
            cellules_height: DEFAULT_HEIGHT,
        };

        Self {
            link,
            game,
            canvas_ref: NodeRef::default(),
            context_ref: None,
            width_input: DEFAULT_WIDTH.to_string(),
            height_input: DEFAULT_HEIGHT.to_string(),
            interval_input: interval_duration.to_string(),
            interval_duration,
            _interval: interval,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Test(msg) => {
                let canvas_width = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap().width();
                log::info!("click x is {}, canvas width is {}", msg, canvas_width);
                false
            }
            Msg::Random => {
                self.game.random_mutate();
                self.draw_cellules(true);
                log::info!("Random");
                false
            }
            Msg::Start => {
                self.game.active = true;
                log::info!("Start");
                false
            }
            Msg::Step => {
                self.game.step();
                self.draw_cellules(false);
                false
            }
            Msg::Reset => {
                self.game.reset();
                self.draw_cellules(false);
                log::info!("Reset");
                false
            }
            Msg::Stop => {
                self.game.active = false;
                log::info!("Stop");
                false
            }
            Msg::SetIntervalInput(interval) => {
                self.interval_input = interval;
                false
            }
            Msg::SetInterval => {
                log::info!("In set interval");
                let interval_duration;
                match self.interval_input.parse::<usize>() {
                    Ok(n) => {
                        log::info!("parse was ok {}", n);
                        interval_duration = n;
                    }
                    Err(_) => {
                        log::info!("parse was error");
                        return false;
                    }
                }
                if interval_duration != self.interval_duration {
                    //self._interval.cancel();
                    let callback = self.link.callback(|_| Msg::Tick);
                    self._interval =
                        Interval::new(interval_duration as u32, move || callback.emit(()));
                    log::info!("interval was set");
                } else {
                    log::info!("interval duration was same");
                }
                false
            }
            Msg::SetHeightInput(height) => {
                self.height_input = height;
                false
            }
            Msg::SetWidthInput(width) => {
                self.width_input = width;
                false
            }
            Msg::SetDimensions => {
                let height;
                let width;
                match self.height_input.parse::<usize>() {
                    Ok(n) => height = n,
                    Err(_) => return false,
                }
                match self.width_input.parse::<usize>() {
                    Ok(n) => width = n,
                    Err(_) => return false,
                }
                self.game.active = false;
                self.game.set_dimensions(width, height);
                let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                canvas.set_width((self.game.cellules_width * CANVAS_SIZE_FACTOR) as u32);
                canvas.set_height((self.game.cellules_height * CANVAS_SIZE_FACTOR) as u32);
                self.draw_cellules(true);
                true
            }
            Msg::ToggleCellule(idx) => {
                let cellule = self.game.cellules.get_mut(idx).unwrap();
                cellule.toggle();
                false
            }
            Msg::Tick => {
                if self.game.active {
                    self.game.step();
                    self.draw_cellules(false);
                }
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        log::info!("view..");
        html! {
          <>
              <header class="app-header">
                  <h1 class="app-title">
                      { "Game of Life" }
                  </h1>
                  <div>
                    { "— a yew experiment " }
                  </div>
                  <div>
                    <a href="https://github.com/yewstack/yew" target="_blank">{ "source" }</a>
                  </div>
              </header>
              <div class="game-canvas-container">


              <canvas
              class="game-canvas"
              ref={self.canvas_ref.clone()}
              height={format!("{}px",self.game.cellules_height * CANVAS_SIZE_FACTOR)}
              width={format!("{}px",self.game.cellules_width * CANVAS_SIZE_FACTOR)}
              onmouseup={self.link.callback(|e: MouseEvent| Msg::Test(e.offset_x().to_string()))} />
              </div>

              <div class="game-area">

                <div class="game-controls">
                    <h2 class="game-controls-title">{"Initialize Pattern"}</h2>
                    <button class="game-button" onclick={self.link.callback(|_| Msg::Random)}>{ "Random" }</button>

                    <h2 class="game-controls-title">{"Game Controls"}</h2>
                    <button class="game-button" onclick={self.link.callback(|_| Msg::Step)}>{ "Step" }</button>
                    <button class="game-button" onclick={self.link.callback(|_| Msg::Start)}>{ "Start" }</button>
                    <button class="game-button" onclick={self.link.callback(|_| Msg::Stop)}>{ "Stop" }</button>
                    <button class="game-button" onclick={self.link.callback(|_| Msg::Reset)}>{ "Reset" }</button>

                    <h2 class="game-controls-title">{"Game Settings"}</h2>
                    <div class="game-setting">
                        <label
                            class="game-settings-input-label"
                            for="input-set-interval">
                            {"Interval (ms): "}
                        </label>
                        <input
                            class="game-settings-input"
                            type="text"
                            id="input-set-interval"
                            name="set-interval"
                            value={self.interval_input.clone()}
                            oninput={self.link.callback(move |e: InputData| Msg::SetIntervalInput(e.value))} />
                    </div>

                    <button class="game-button" onclick={self.link.callback(|_| Msg::SetInterval)}>{ "Set Step Delay" }</button>

                    <div class="game-setting">
                        <label
                            class="game-settings-input-label"
                            for="input-set-width">
                            {"Cell columns: "}
                        </label>
                        <input
                            class="game-settings-input"
                            type="text"
                            id="input-set-width"
                            name="set-width"
                            value={self.width_input.clone()}
                            oninput={self.link.callback(move |e: InputData| Msg::SetWidthInput(e.value))} />
                    </div>

                    <div class="game-setting">
                        <label
                            class="game-settings-input-label"
                            for="input-set-height">
                            {"Cell rows: "}
                        </label>
                        <input
                            class="game-settings-input"
                            type="text"
                            id="input-set-height"
                            name="set-height"
                            value={self.height_input.clone()}
                            oninput={self.link.callback(move |e: InputData| Msg::SetHeightInput(e.value))} />
                    </div>

                    <button class="game-button" onclick={self.link.callback(|_| Msg::SetDimensions)}>{ "Set Dimensions" }</button>


                </div>

              </div>
              <footer class="app-footer">
                <p><a href="https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life" target="_blank" >{ "Conway's Game of Life - Wikipedia" }</a></p>

              </footer>
          </>

        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();
            self.context_ref = Some(context);
        }
        let context = self.context_ref.as_ref().unwrap();
        context.set_fill_style(&COLOR_GRID.into());
        context.fill_rect(
            0.0,
            0.0,
            (self.game.cellules_width * CANVAS_SIZE_FACTOR) as f64 - 1.0,
            (self.game.cellules_height * CANVAS_SIZE_FACTOR) as f64 - 1.0,
        );
        self.draw_cellules(true);
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::start_app::<Model>();
}
