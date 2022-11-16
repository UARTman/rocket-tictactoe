use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Cell {
    X,
    O,
}

impl Cell {
    pub fn flip(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct TicTacToeGame {
    pub field: Vec<Vec<Option<Cell>>>,
    #[serde(rename="currentPlayer")]
    pub current_player: Option<Cell>,
    pub winner: Option<Cell>,
    pub size: usize,
    pub criteria: usize,
    pub draw: bool,
}

impl TicTacToeGame {
    pub fn new(size: usize, criteria: usize) -> Self {
        let field = vec![vec![None; size]; size];
        Self {
            field,
            current_player: Some(Cell::X),
            winner: None,
            size,
            criteria,
            draw: false,
        }
    }

    pub fn turn(&mut self, x: usize, y: usize) -> Option<Cell> {
        if self.winner.is_some() || self.draw {
            return None;
        }
        if self.field.get(x)?.get(y)?.is_some() {
            return None;
        }
        self.field[x][y] = self.current_player;
        let winner = self.check_field_for_winner();
        if winner.is_some() {
            self.winner = self.current_player;
            return self.winner;
        }
        self.current_player = Some(self.current_player?.flip());
        self.draw = self.check_draw();
        None
    }

    pub fn check_field_for_winner(&self) -> Option<Cell> {
        for i in 0..self.size - self.criteria + 1 {
            for j in 0..self.size - self.criteria + 1 {
                let x = self.check_span_for_winner(i, j);
                if x.is_some() {
                    return x;
                }
            }
        }
        None
    }

    pub fn check_span_for_winner(&self, x: usize, y: usize) -> Option<Cell> {
        // Check for rows
        for i in x..x + self.criteria {
            let mut q: Option<Cell> = None;
            for j in 0..self.criteria {
                if j == 0 {
                    q = self.field[i][y + j];
                    if q == None {
                        break;
                    }
                } else if self.field[i][y + j] != q {
                    q = None;
                    break;
                }
            }
            if q.is_some() {
                return q;
            }
        }

        for i in y..y + self.criteria {
            let mut q: Option<Cell> = None;
            for j in 0..self.criteria {
                if j == 0 {
                    q = self.field[x + j][i];
                    if q == None {
                        break;
                    }
                } else if self.field[x + j][i] != q {
                    q = None;
                    break;
                }
            }
            if q.is_some() {
                return q;
            }
        }

        let mut q: Option<Cell> = None;
        for i in 0..self.criteria {
            if i == 0 {
                q = self.field[x + i][y + i];
                if q == None {
                    break;
                }
            } else if self.field[x + i][y + i] != q {
                q = None;
                break;
            }
        }
        if q.is_some() {
            return q;
        }

        let mut q: Option<Cell> = None;
        for i in 0..self.criteria {
            if i == 0 {
                q = self.field[x + i][y + self.criteria - 1 - i];
                if q == None {
                    break;
                }
            } else if self.field[x + i][y + self.criteria - 1 - i] != q {
                q = None;
                break;
            }
        }
        if q.is_some() {
            return q;
        }

        None
    }

    fn check_draw(&self) -> bool {
        self.field.iter().all(|x| x.iter().all(|y| y.is_some()))
    }
}

#[cfg(test)]
mod test {
    use super::Cell::*;
    use super::TicTacToeGame;

    #[test]
    fn detects_horizontal() {
        let mut x = TicTacToeGame::new(3, 2);
        x.field = vec![
            vec![None, None, None],
            vec![None, None, None],
            vec![None, None, None],
        ];
        assert_eq!(None, x.check_span_for_winner(0, 0));
        assert_eq!(None, x.check_field_for_winner());
        for i in 0..3 {
            for j in 0..2 {
                x.field = vec![
                    vec![None, None, None],
                    vec![None, None, None],
                    vec![None, None, None],
                ];
                x.field[i][j] = Some(X);
                x.field[i][j + 1] = Some(X);
                // assert_eq!(Some(X), x.check_span_for_winner(i, j), "check_span_for_winner({i}, {j}) (field: {:?})", x.field);
                assert_eq!(
                    Some(X),
                    x.check_field_for_winner(),
                    "check_field_for_winner (i={i}, j={j}, field={:?})",
                    x.field
                );
            }
        }
    }

    #[test]
    fn detects_vertical() {
        let mut x = TicTacToeGame::new(3, 2);
        x.field = vec![
            vec![None, None, None],
            vec![None, None, None],
            vec![None, None, None],
        ];
        assert_eq!(None, x.check_span_for_winner(0, 0));
        assert_eq!(None, x.check_field_for_winner());
        for i in 0..2 {
            for j in 0..3 {
                x.field = vec![
                    vec![None, None, None],
                    vec![None, None, None],
                    vec![None, None, None],
                ];
                x.field[i][j] = Some(X);
                x.field[i + 1][j] = Some(X);
                // assert_eq!(Some(X), x.check_span_for_winner(i, j), "check_span_for_winner({i}, {j}) (field: {:?})", x.field);
                assert_eq!(
                    Some(X),
                    x.check_field_for_winner(),
                    "check_field_for_winner (i={i}, j={j}, field={:?})",
                    x.field
                );
            }
        }
    }

    #[test]
    fn detects_diagonal() {
        let mut x = TicTacToeGame::new(3, 2);
        x.field = vec![
            vec![None, None, None],
            vec![None, None, None],
            vec![None, None, None],
        ];
        assert_eq!(None, x.check_span_for_winner(0, 0));
        assert_eq!(None, x.check_field_for_winner());
        for i in 0..2 {
            for j in 0..2 {
                x.field = vec![
                    vec![None, None, None],
                    vec![None, None, None],
                    vec![None, None, None],
                ];
                x.field[i][j] = Some(X);
                x.field[i + 1][j + 1] = Some(X);
                // assert_eq!(Some(X), x.check_span_for_winner(i, j), "check_span_for_winner({i}, {j}) (field: {:?})", x.field);
                assert_eq!(
                    Some(X),
                    x.check_field_for_winner(),
                    "check_field_for_winner (i={i}, j={j}, field={:?})",
                    x.field
                );

                x.field = vec![
                    vec![None, None, None],
                    vec![None, None, None],
                    vec![None, None, None],
                ];
                x.field[i + 1][j] = Some(X);
                x.field[i][j + 1] = Some(X);
                // assert_eq!(Some(X), x.check_span_for_winner(i, j), "check_span_for_winner({i}, {j}) (field: {:?})", x.field);
                assert_eq!(
                    Some(X),
                    x.check_field_for_winner(),
                    "check_field_for_winner (i={i}, j={j}, field={:?})",
                    x.field
                );
            }
        }
    }
}
