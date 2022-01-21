#[derive(Debug)]
pub enum GameMessage {
    Selection { row: u16, col: u16 },
    End { winner: u16 }, // 1 or 2 (or 0, in the case of a draw)
}

impl GameMessage {
    pub fn read(text: String) -> Result<Self, &'static str> {
        let _text = text.to_lowercase();
        let parts = _text.split(" ").collect::<Vec<&str>>();
        if parts.len() == 0 {
            return Err("empty message!");
        }

        if parts[0] == "selection" && parts.len() == 3 {
            if let (Ok(row), Ok(col)) = (parts[1].parse(), parts[2].parse()) {
                return Ok(GameMessage::Selection { row, col });
            }
            return Err("could not parse selection message");
        }

        if parts[0] == "End" && parts.len() == 2 {
            if let Ok(winner) = parts[1].parse() {
                return Ok(GameMessage::End { winner });
            }
            return Err("could not parse game end message");
        }

        Err("could not parse message")
    }
}
