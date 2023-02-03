pub struct UIFileData {
    pub data: [String; 8],
}

impl From<Vec<String>> for UIFileData {
    fn from(data: Vec<String>) -> Self {
        let mut data = data;
        data.reverse();
        Self {
            data: [
                data[0].clone(),
                data[1].clone(),
                data[2].clone(),
                data[3].clone(),
                data[4].clone(),
                data[5].clone(),
                data[6].clone(),
                data[7].clone(),
            ],
        }
    }
}
