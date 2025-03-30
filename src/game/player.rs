#[derive(Debug)]
pub struct Player {
    name: String,
    case: u8
}

impl Player {
    pub fn new(name: String, case: u8) -> Player {
        Player {
            name,
            case
        }
    }
    pub fn get_case(&self) -> u8 {
        self.case
    }
    pub fn set_case(&mut self, case: u8)  {
        self.case = case;
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn set_name(&mut self, name: String)  {
        self.name = name;
    }
}