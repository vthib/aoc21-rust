pub fn part1(input: &str) -> impl std::fmt::Display {
    let mut game = parse_input(input);
    let mut dice = DeterministicDice::default();

    while game.player1.score < 1000 && game.player2.score < 1000 {
        let movement = dice.draw() + dice.draw() + dice.draw();
        game.apply_turn(movement);
    }

    std::cmp::min(game.player1.score, game.player2.score) * dice.nb_draws()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    let game = parse_input(input);
    let res = play_quantumly(game);

    std::cmp::max(res.0, res.1)
}

fn play_quantumly(game: Game) -> (u64, u64) {
    if game.player1.score >= 21 {
        return (1, 0);
    }
    if game.player2.score >= 21 {
        return (0, 1);
    }

    // Every draw creates a new game, where the draw is 1, 2 or 3.
    // This means every turn creates 27 new games, with movement:
    // 1..3 + 1..3 + 1..3
    // =
    //   111, 112, 113, 121, 122, 123, 131, 132, 133
    //   211, 212, 213, 221, 222, 223, 231, 232, 233
    //   311, 312, 313, 321, 322, 323, 331, 332, 333
    // =
    //   3, 4, 5, 4, 5, 6, 5, 6, 7,
    //   4, 5, 6, 5, 6, 7, 6, 7, 8,
    //   5, 6, 7, 6, 7, 8, 7, 8, 9,
    // =
    //   1 * 3, 3 * 4, 6 * 5, 7 * 6, 6 * 7, 3 * 8, 1 * 9
    //
    // Instead of simulating each of those 27 possibilities,
    // simulate the 7 possibilites, and multiply the number of wins
    // by the multiplier.
    const POSSIBLE_DRAWS: [(u64, u32); 7] =
        [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

    let mut res = (0, 0);
    for possible_draw in POSSIBLE_DRAWS {
        let multiplier = possible_draw.0;
        let movement = possible_draw.1;

        let mut inner_game = game.clone();
        inner_game.apply_turn(movement);
        let inner_res = play_quantumly(inner_game);
        res.0 += inner_res.0 * multiplier;
        res.1 += inner_res.1 * multiplier;
    }
    res
}

#[derive(Default)]
struct DeterministicDice {
    pos: u32,
    nb_draws: u32,
}

impl DeterministicDice {
    fn draw(&mut self) -> u32 {
        if self.pos >= 100 {
            self.pos = 0;
        }
        self.pos += 1;
        self.nb_draws += 1;
        self.pos
    }

    fn nb_draws(&self) -> u32 {
        self.nb_draws
    }
}

#[derive(Clone)]
struct Player {
    // Position between 0 and 9.
    pawn_position: u32,
    score: u32,
}

impl Player {
    fn apply_movement(&mut self, movement: u32) {
        self.pawn_position = (self.pawn_position + movement) % 10;
        self.score += self.pawn_position + 1;
    }
}

#[derive(Clone)]
struct Game {
    player1: Player,
    player2: Player,
    player1_turn: bool,
}

impl Game {
    fn new(player1_position: u32, player2_position: u32) -> Self {
        Self {
            player1: Player {
                pawn_position: player1_position - 1,
                score: 0,
            },
            player2: Player {
                pawn_position: player2_position - 1,
                score: 0,
            },
            player1_turn: true,
        }
    }

    fn apply_turn(&mut self, movement: u32) {
        if self.player1_turn {
            self.player1.apply_movement(movement);
        } else {
            self.player2.apply_movement(movement);
        }
        self.player1_turn = !self.player1_turn;
    }
}

fn parse_input(input: &str) -> Game {
    let mut lines = input.lines();
    let l1 = lines.next().unwrap();
    let l2 = lines.next().unwrap();
    assert!(lines.next().is_none());

    Game::new(
        l1.strip_prefix("Player 1 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
        l2.strip_prefix("Player 2 starting position: ")
            .unwrap()
            .parse()
            .unwrap(),
    )
}
