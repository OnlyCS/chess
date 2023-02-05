use super::selection::SelectionType;

#[derive(Debug, Clone)]
pub struct UIFileData {
    pub pieces: Vec<String>,
    pub select: Vec<Option<SelectionType>>,
}

#[derive(Debug, Clone)]
pub struct SelectData {
    pub selection: Option<SelectionType>,
    pub piece: String,
}

impl UIFileData {
    pub fn create_from(data: Vec<SelectData>) -> Self {
        let mut pieces = Vec::new();
        let mut selections = Vec::new();

        for d in data {
            pieces.push(d.piece);
            selections.push(d.selection);
        }

        Self {
            pieces,
            select: selections,
        }
    }
}
