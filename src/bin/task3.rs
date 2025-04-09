#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}

impl Move {
    /// Bestimmt das Ergebnis einer Runde basierend auf dem eigenen Zug und dem des Gegners.
    fn play(&self, opponent: &Move) -> Result {
        use Move::*;
        match (self, opponent) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Result::Win,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Result::Lose,
            _ => Result::Draw,
        }
    }
}

impl Result {
    /// Kehrt das Ergebnis um.
    fn reverse(&self) -> Result {
        match self {
            Result::Win => Result::Lose,
            Result::Lose => Result::Win,
            Result::Draw => Result::Draw,
        }
    }
}

fn main() {
    let my_move = Move::Rock;
    let opponent_move = Move::Scissors;

    let result = my_move.play(&opponent_move);
    
    println!("Mein Zug: {:?}, Gegnerzug: {:?}, Ergebnis: {:?}", my_move, opponent_move, result);

    let reversed_result = result.reverse();
    
    println!("Umgekehrtes Ergebnis: {:?}", reversed_result);
}
