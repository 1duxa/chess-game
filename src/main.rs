use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL_ALLOCATOR: MiMalloc = MiMalloc;
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

struct BoardState {
    tiles: [[Option<Piece>; 8]; 8],
}
impl BoardState {
    pub fn new() -> BoardState {
        Self {
            tiles: [
                [
                    Some(Piece::Rook),
                    Some(Piece::Knight),
                    Some(Piece::Bishop),
                    Some(Piece::Queen),
                    Some(Piece::King),
                    Some(Piece::Bishop),
                    Some(Piece::Knight),
                    Some(Piece::Rook),
                ],
                [
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                ],
                [
                    Some(Piece::Rook),
                    Some(Piece::Knight),
                    Some(Piece::Bishop),
                    Some(Piece::Queen),
                    Some(Piece::King),
                    Some(Piece::Bishop),
                    Some(Piece::Knight),
                    Some(Piece::Rook),
                ],
            ],
        }
    }
    /// true - white, false - black
    /// + 2 cause i want to count from 1 not from 0
    pub fn color(x: usize, y: usize) -> bool {
        (x + y + 2) % 2 == 0
    }
}

struct ChessApp {
    state: BoardState,
    moves: Vec<[char; 5]>,
}
impl ChessApp {
    pub fn new() -> Self {
        Self {
            state: BoardState::new(),
            moves: Vec::new(),
        }
    }
    pub fn move_piece(piece: Piece, from: usize, to: usize) {}
}

fn main() {
    println!("Hello, world!");
}
