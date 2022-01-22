#[derive(Debug)]
pub enum GameMessage {
    Board { state_str: String },
    Selection { row: u8, col: u8 },
    End { winner: u8 }, // 1 or 2 (or 0, in the case of a draw)
}

impl GameMessage {
    pub fn read(text: String) -> Result<Self, &'static str> {
        let text = text.to_lowercase();

        if text.starts_with("board") {
            return Ok(Self::Board { state_str: text });
        }

        let parts = text.split(" ").collect::<Vec<&str>>();
        if parts.len() == 0 {
            return Err("empty message!");
        }

        if parts[0] == "selection" && parts.len() == 3 {
            if let (Ok(row), Ok(col)) = (parts[1].parse(), parts[2].parse()) {
                return Ok(GameMessage::Selection { row, col });
            }
            return Err("could not parse selection message");
        }

        if parts[0] == "end" && parts.len() == 2 {
            if let Ok(winner) = parts[1].parse() {
                return Ok(GameMessage::End { winner });
            }
            return Err("could not parse game end message");
        }

        Err("could not parse message")
    }
}
