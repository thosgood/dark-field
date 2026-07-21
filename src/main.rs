use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};

enum Either<L, R> {
    Left(L),
    Right(R),
}

const LEVEL_MAP: &str = r#"......lorem0ipsum0dolor
......sit0amet0luus0to0
......quanto0id0ido0est
......ic0sindum0ninser0
......floran0flaer0dost......tfwnqein
......lorem0ipsum0dolorntfeiwqnneifts]
gtnfeisit0amet0luus0to0......tenifwqf
d.....quanto0id0ido0est
t.....ic0sindum0ninser0
s.....floran0flaer0dost
a.....lorem0ipsum0dolor
kuicfasit0amet0luus0to0
......quanto0id0ido0est
......ic0sindum0ninser0
......floran0flaer0dost"#;

// We think of `Map` and `Photo` as being indexed like matrices, i.e. with the
// top-left-most character being (0,0) and the bottom-right-most being (w, h),
// where w = width and h = height. We realise this by modelling them as a list
// of rows, where each row is a list of `char`.
struct Photo(Vec<Vec<char>>);

impl Photo {
    fn to_vec_string(&self) -> Vec<String> {
        self.0
            .iter()
            .cloned()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
    }
}

#[derive(Clone)]
struct Location {
    ascii_map: Vec<Vec<char>>,
    empty_space: char,
    obstacle: char,
}

impl Location {
    fn from_string(ascii: &str, empty_space: char, obstacle: char) -> Self {
        let lines: Vec<Vec<char>> = ascii
            .split("\n")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        Location {
            ascii_map: lines,
            empty_space,
            obstacle,
        }
    }

    fn _max_width(&self) -> usize {
        self.ascii_map
            .iter()
            .map(|line| line.len())
            .fold(0usize, |acc, x| acc.max(x))
    }

    fn _min_width(&self) -> usize {
        self.ascii_map
            .iter()
            .map(|line| line.len())
            .fold(self._max_width(), |acc, x| acc.min(x))
    }

    fn _naive_height(&self) -> usize {
        self.ascii_map.len()
    }

    fn take_photo(&self, centre: &GridPosition, half_height: usize, half_width: usize) -> Photo {
        // We interpret the half_height and half_width as excluding the space that
        // the player occupies, i.e. every photo will be of odd height and odd width.
        let photo_height = 2 * half_height + 1;
        let photo_width = 2 * half_width + 1;

        let mut cropped_photo: Vec<Vec<char>> = Vec::new();
        let empty_space = self.empty_space;
        // We build up the cropped photo by just checking if each coordinate is in bounds
        // or not and then pushing the character at that coordinate or the `empty_space`
        // character (respectively).
        for j in 0..photo_height {
            let mut cropped_line: Vec<char> = Vec::new();
            for i in 0..photo_width {
                let point_x = (centre.x + i)
                    .checked_signed_diff(half_width)
                    .expect("horizontal numbers should never be this big");
                let point_y = (centre.y + j)
                    .checked_signed_diff(half_height)
                    .expect("vertical numbers should never be this big");
                if let Some(point) = GridPosition::try_new(point_x, point_y) {
                    if self.is_in_bounds(&point) {
                        cropped_line.push(self.ascii_map[point.y][point.x]);
                    } else {
                        cropped_line.push(empty_space);
                    }
                } else {
                    cropped_line.push(empty_space);
                }
            }
            cropped_photo.push(cropped_line);
        }

        Photo(cropped_photo)
    }

    fn force_in_bounds(&self, position: &GridPosition) -> Either<GridPosition, GridPosition> {
        if let Some(line) = self.ascii_map.get(position.y) {
            if let Some(character) = line.get(position.x) {
                if *character != self.empty_space {
                    return Either::Left(position.clone());
                }
            }
        }
        // TODO: make this actually compute the nearest point... maybe just by
        //       spiralling outwards?
        Either::Right(GridPosition { x: 0, y: 0 })
    }

    fn is_in_bounds(&self, position: &GridPosition) -> bool {
        match self.force_in_bounds(position) {
            Either::Left(_) => true,
            Either::Right(_) => false,
        }
    }

    fn force_can_walk_on(&self, position: &GridPosition) -> Either<GridPosition, GridPosition> {
        // TODO: do you actually need this much information?  i.e. "remembering" the result of
        //       `force_in_bounds` in the `Left` clause?
        match self.force_in_bounds(position) {
            Either::Left(original) => {
                let character = self
                    .ascii_map
                    .get(original.y)
                    .expect("`is_in_bounds` made a vertical mistake")
                    .get(original.x)
                    .expect("`is_in_bounds` made a horizontal mistake");
                if *character == self.obstacle {
                    // TODO: make this also actually compute the nearest point
                    Either::Right(GridPosition { x: 1, y: 1 })
                } else {
                    Either::Left(original)
                }
            }
            Either::Right(forced_in_bounds) => {
                let character = self
                    .ascii_map
                    .get(forced_in_bounds.y)
                    .expect("`is_in_bounds` made a vertical mistake")
                    .get(forced_in_bounds.x)
                    .expect("`is_in_bounds` made a horizontal mistake");
                if *character == self.obstacle {
                    // TODO: make this also also actually compute the nearest point
                    Either::Right(GridPosition { x: 2, y: 2 })
                } else {
                    Either::Right(forced_in_bounds)
                }
            }
        }
    }

    fn can_walk_on(&self, position: &GridPosition) -> bool {
        if self.is_in_bounds(position) {
            let character = self
                .ascii_map
                .get(position.y)
                .expect("`is_in_bounds` made a mistake")
                .get(position.x)
                .expect("`is_in_bounds` made a mistake");
            *character != self.obstacle
        } else {
            false
        }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct GridPosition {
    x: usize,
    y: usize,
}

impl GridPosition {
    fn try_new(x: isize, y: isize) -> Option<Self> {
        if x >= 0 && y >= 0 {
            Some(GridPosition {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
}

// TODO: impl `Iterator` by spiraling around or something
//       (probably use `max_width` and stuff)

#[derive(Debug)]
struct RealPosition {
    x: f32,
    y: f32,
}

impl RealPosition {
    fn from_grid_position(grid_position: &GridPosition) -> Self {
        // TODO: check this is ok? what is the usize is too big...
        RealPosition {
            x: grid_position.x as f32,
            y: grid_position.y as f32,
        }
    }

    fn nearest_grid_position(&self) -> GridPosition {
        GridPosition {
            x: self.x.round() as usize,
            y: self.y.round() as usize,
        }
    }
}

#[derive(Debug)]
struct Player {
    grid_position: GridPosition,
    real_position: RealPosition,
    direction: Direction,
    eyesight: usize,
    debug: String,
}

impl Player {
    fn new(
        location: &Location,
        starting_x: usize,
        starting_y: usize,
        direction: Direction,
        eyesight: usize,
    ) -> Self {
        let grid_position: GridPosition;
        // TODO: isn't there a slicker way to do this? surely
        match location.force_can_walk_on(&GridPosition {
            x: starting_x,
            y: starting_y,
        }) {
            Either::Left(desired_point) => grid_position = desired_point,
            Either::Right(forced_point) => grid_position = forced_point,
        };
        let real_position = RealPosition::from_grid_position(&grid_position);

        Player {
            grid_position,
            real_position,
            direction,
            eyesight,
            debug: String::new(),
        }
    }

    fn take_step(&mut self, map: &Location) {
        let tentative_position = match self.direction {
            Direction::North => GridPosition {
                x: self.grid_position.x,
                y: (self.grid_position.y).saturating_sub(1),
            },
            Direction::East => GridPosition {
                x: self.grid_position.x + 1,
                y: self.grid_position.y,
            },
            Direction::South => GridPosition {
                x: self.grid_position.x,
                y: self.grid_position.y + 1,
            },
            Direction::West => GridPosition {
                x: (self.grid_position.x).saturating_sub(1),
                y: self.grid_position.y,
            },
        };

        if !map.can_walk_on(&tentative_position) {
            self.debug = format!("obstacle: {:?}", &tentative_position);
        }
        if !map.is_in_bounds(&tentative_position) {
            self.debug = "out of bounds".to_string();
        }

        if map.is_in_bounds(&tentative_position) && map.can_walk_on(&tentative_position) {
            self.grid_position = tentative_position;
        }
    }

    // TODO: can't you do something like Direction.x => Direction.(x-1) ?
    //       (maybe even by just implementing Iterate? but surely simpler...)
    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn selfie(&self) -> char {
        match self.direction {
            Direction::North => '⮝',
            Direction::East => '⮞',
            Direction::South => '⮟',
            Direction::West => '⮜',
        }
    }

    fn take_photo(&self, map: &Location) -> Photo {
        let Photo(mut photo) = map.take_photo(&(self.grid_position), self.eyesight, self.eyesight);
        // Because we specify photos as having centre points, but we access points
        // in a `Vec<Vec<_>>` by working from the top left, we always need to shift
        // by the (half) size of the photo (which, here, is given by `eyesight`).
        photo[self.eyesight][self.eyesight] = self.selfie();
        Photo(photo)
    }
}

#[derive(PartialEq, Eq)]
enum GameState {
    Running,
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
                _ => {}
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        // +---------+------------+
        // |         | (2) Speech |
        // |         +------------+
        // | (1) FPV | (3) Map    |
        // |         +------------+
        // |         | (4) Debug  |
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

        // (1) FPV (first-person view)
        let fpv_block = Block::bordered().title("Sight");
        let fpv = Paragraph::new("[tbd]").centered();
        frame.render_widget(&fpv_block, lhs);
        frame.render_widget(fpv, fpv_block.inner(lhs));

        // (2) Speech
        let speech_block = Block::bordered().title("Hearing");
        let speech = Paragraph::new("Things have gone awry.").centered();
        frame.render_widget(&speech_block, rhs_top);
        frame.render_widget(speech, speech_block.inner(rhs_top));

        // (3) Map
        let map_block = Block::bordered().title("Map");
        let map = Paragraph::new(
            self.player
                .take_photo(&self.location)
                .to_vec_string()
                .join("\n"),
        )
        .centered();
        frame.render_widget(&map_block, rhs_middle);
        frame.render_widget(map, map_block.inner(rhs_middle));

        // (4) Debug
        let debug_block = Block::bordered().title("Debug");
        let debug = Paragraph::new(format!("{:?}", self.player))
            .centered()
            .wrap(Wrap { trim: true });
        frame.render_widget(&debug_block, rhs_bottom);
        frame.render_widget(debug, debug_block.inner(rhs_bottom));
    }
}

fn main() -> Result<()> {
    let location = Location::from_string(LEVEL_MAP, '.', '0');
    let player = Player::new(&location, 6, 0, Direction::South, 6);

    let game = Game {
        player,
        location,
        state: GameState::Running,
    };

    ratatui::run(|terminal| game.run(terminal))
}
