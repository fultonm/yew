use cell::Cellule;
use gloo::timers::callback::Interval;
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};
mod cell;

const DEFAULT_INTERVAL: usize = 0;
const DEFAULT_WIDTH: usize = 180;
const DEFAULT_HEIGHT: usize = 86;

pub enum Msg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    ToggleCellule(usize),
    SetInterval(String),
    SetDimensions(usize, usize),
    Tick,
}

pub struct Model {
    link: ComponentLink<Self>,
    active: bool,
    cellules: Vec<Cellule>,
    cellules_width: usize,
    cellules_height: usize,
    interval_duration: usize,
    canvas_ref: NodeRef,
    context_ref: Option<CanvasRenderingContext2d>,
    _interval: Interval,
}

impl Model {
    pub fn random_mutate(&mut self) {
        for cellule in self.cellules.iter_mut() {
            if rand::thread_rng().gen() {
                cellule.set_alive();
            } else {
                cellule.set_dead();
            }
        }
    }

    fn reset(&mut self) {
        for cellule in self.cellules.iter_mut() {
            cellule.set_dead();
        }
    }

    fn step(&mut self) {
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let neighbors = self.neighbors(row as isize, col as isize);

                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                self.cellules[current_idx].set_render_flag(false);
                if self.cellules[current_idx].is_alive() {
                    if Cellule::alone(&neighbors) || Cellule::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else if Cellule::can_be_revived(&neighbors) {
                    to_live.push(current_idx);
                }
            }
        }
        to_dead
            .iter()
            .for_each(|idx| self.cellules[*idx].set_dead());
        to_live
            .iter()
            .for_each(|idx| self.cellules[*idx].set_alive());
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cellule; 8] {
        [
            self.cellules[self.row_col_as_idx(row + 1, col)],
            self.cellules[self.row_col_as_idx(row + 1, col + 1)],
            self.cellules[self.row_col_as_idx(row + 1, col - 1)],
            self.cellules[self.row_col_as_idx(row - 1, col)],
            self.cellules[self.row_col_as_idx(row - 1, col + 1)],
            self.cellules[self.row_col_as_idx(row - 1, col - 1)],
            self.cellules[self.row_col_as_idx(row, col - 1)],
            self.cellules[self.row_col_as_idx(row, col + 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cellules_height as isize);
        let col = wrap(col, self.cellules_width as isize);

        row * self.cellules_width + col
    }

    fn view_cellule(&self) {
        let context = self.context_ref.as_ref().unwrap();
        for row in 0..self.cellules_height {
            for col in 0..self.cellules_width {
                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                let cellule = self.cellules[current_idx];
                if cellule.should_render() {
                    if cellule.is_alive() {
                        context.set_fill_style(&"#000".into());
                    } else {
                        context.set_fill_style(&"#FFF".into());
                    }
                    context.fill_rect((col * 2) as f64, (row * 2) as f64, 2 as f64, 2 as f64);
                }
            }
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let interval_duration = DEFAULT_INTERVAL;

        let callback = link.callback(|_| Msg::Tick);
        let interval = Interval::new(interval_duration as u32, move || callback.emit(()));

        let (cellules_width, cellules_height) = (DEFAULT_WIDTH, DEFAULT_HEIGHT);

        Self {
            link,
            active: false,
            cellules: vec![Cellule::new_dead(); cellules_width * cellules_height],
            cellules_width,
            cellules_height,
            interval_duration,
            canvas_ref: NodeRef::default(),
            context_ref: None,
            _interval: interval,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Random => {
                self.random_mutate();
                self.view_cellule();
                log::info!("Random");
                true
            }
            Msg::Start => {
                self.active = true;
                log::info!("Start");
                false
            }
            Msg::Step => {
                self.step();
                true
            }
            Msg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            }
            Msg::Stop => {
                self.active = false;
                log::info!("Stop");
                false
            }
            Msg::SetInterval(interval_string) => {
                let interval_duration = interval_string.parse::<usize>().unwrap();
                if interval_duration != self.interval_duration {
                    let callback = self.link.callback(|_| Msg::Tick);
                    self._interval =
                        Interval::new(self.interval_duration as u32, move || callback.emit(()));
                    true
                } else {
                    false
                }
            }
            Msg::SetDimensions(width, height) => {
                self.active = false;
                log::info!("Stop");
                self.reset();
                log::info!("Reset");
                self.cellules_width = width;
                self.cellules_height = height;
                true
            }
            Msg::ToggleCellule(idx) => {
                let cellule = self.cellules.get_mut(idx).unwrap();
                cellule.toggle();
                true
            }
            Msg::Tick => {
                if self.active {
                    self.step();
                    self.view_cellule();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <section class="game-container">
                <header class="app-header">
                    <img src="favicon.ico" class="app-logo"/>
                    <h1 class="app-title">{ "Game of Life" }</h1>
                </header>
                <section class="game-area">
                  <canvas ref={self.canvas_ref.clone()} height="172px" width="360px" style="background:#FFF;"></canvas>
                    <div class="game-buttons">
                        <button class="game-button" onclick={self.link.callback(|_| Msg::Random)}>{ "Random" }</button>
                        <button class="game-button" onclick={self.link.callback(|_| Msg::Step)}>{ "Step" }</button>
                        <button class="game-button" onclick={self.link.callback(|_| Msg::Start)}>{ "Start" }</button>
                        <button class="game-button" onclick={self.link.callback(|_| Msg::Stop)}>{ "Stop" }</button>
                        <button class="game-button" onclick={self.link.callback(|_| Msg::Reset)}>{ "Reset" }</button>
                    </div>
                    <div class="game-settings">
                        <input class="game-input"
                               oninput={self.link.callback(move |e: InputData| Msg::SetInterval(e.value))} />
                    </div>
                </section>
            </section>
            <footer class="app-footer">
                <strong class="footer-text">
                  { "Game of Life - a yew experiment " }
                </strong>
                <a href="https://github.com/yewstack/yew" target="_blank">{ "source" }</a>
            </footer>
        </div>

          }
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();
            self.context_ref = Some(context);
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::start_app::<Model>();
}
