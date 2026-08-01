mod locations;
mod movement;
mod photos;
mod player;

use crate::locations::*;
use crate::movement::*;
use crate::photos::*;
use crate::player::*;

use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<T> Either<T, T> {
    pub fn unwrap(&self) -> &T {
        match self {
            Either::Left(left) => left,
            Either::Right(right) => right,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum GameState {
    Debug,
    FPV,
    Quit,
}

struct Game {
    player: Player,
    location: Location,
    state: GameState,
}

impl Game {
    fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.state != GameState::Quit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        // Set the (maximum?) length of the frame to be 1/50 seconds.
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Up => {
                    self.player.take_step(&self.location);
                }
                KeyCode::Left => {
                    self.player.turn_left();
                }
                KeyCode::Right => {
                    self.player.turn_right();
                }
                KeyCode::Char('z') => {
                    self.player.eyesight += 1;
                }
                KeyCode::Char('x') => {
                    self.player.eyesight = self.player.eyesight.saturating_sub(1);
                }
                KeyCode::Char('q') => {
                    self.state = GameState::Quit;
                }
                KeyCode::Char('f') => {
                    self.state = GameState::FPV;
                }
                KeyCode::Char('d') => {
                    self.state = GameState::Debug;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        // +---------+------------+
        // |         | (2)        |
        // |         +------------+
        // | (1)     | (3)        |
        // |         +------------+
        // |         | (4)        |
        // +---------+------------+

        let columns = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        let (lhs, rhs) = (columns[0], columns[1]);
        let rhs_rows = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(rhs);
        let (rhs_top, rhs_middle, rhs_bottom) = (rhs_rows[0], rhs_rows[1], rhs_rows[2]);

        // Conversation.
        let speech_block = Block::bordered().title("Hearing");
        let speech = Paragraph::new("Things have gone awry.").centered();

        // Map
        let map_block = Block::bordered().title("Map");
        let map = Paragraph::new(
            self.player
                .take_bird_photo(&self.location)
                .to_vec_string()
                .join("\n"),
        )
        .centered();

        // Controls
        let controls_block = Block::bordered().title("Controls");
        let controls = Paragraph::new(format!("[d] debug; [f] full FPV"))
            .centered()
            .wrap(Wrap { trim: true });

        // FPV
        let fpv_block = Block::bordered().title("Sight");
        let fpv = Paragraph::new(
            self.player
                .take_eye_photo(&self.location)
                .to_vec_string()
                .join("\n"),
        )
        .centered();

        // Debug
        let debug_block = Block::bordered().title("Debug");
        let debug = Paragraph::new(format!("{}", self.player))
            .centered()
            .wrap(Wrap { trim: true });

        // Rendering
        frame.render_widget(&speech_block, rhs_top);
        frame.render_widget(speech, speech_block.inner(rhs_top));

        frame.render_widget(&map_block, rhs_middle);
        frame.render_widget(&map, map_block.inner(rhs_middle));

        frame.render_widget(&controls_block, rhs_bottom);
        frame.render_widget(&controls, controls_block.inner(rhs_bottom));

        match self.state {
            GameState::Debug => {
                frame.render_widget(&debug_block, lhs);
                frame.render_widget(&debug, debug_block.inner(lhs));
            }
            GameState::FPV => {
                frame.render_widget(&fpv_block, lhs);
                frame.render_widget(&fpv, fpv_block.inner(lhs));
            }
            GameState::Quit => {}
        };
    }
}

fn main() -> Result<()> {
    let location = Location::from_string(LEVEL_MAP, '.', '0');
    let player = Player::new(&location, 6, 0, DiscreteDirection::South, 6);

    let game = Game {
        player,
        location,
        state: GameState::FPV,
    };

    ratatui::run(|terminal| game.run(terminal))
}
