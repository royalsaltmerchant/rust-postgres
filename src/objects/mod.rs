pub struct Character {
    pub name: String,
    pub age: i32,
    pub class: String,
    pub race: String,
    pub hp: i32,
    pub exp: i32,
}

// pub struct DBCharacter {
//     id: i32,
//     pub name: String,
//     pub age: i32,
//     pub class: String,
//     pub race: String,
//     pub hp: i32,
//     pub exp: i32,
// }

impl Character {
    pub fn new(name: String, age: i32, class: String, race: String) -> Character {
        Character {
            name,
            age,
            class,
            race,
            hp: 30,
            exp: 0,
        }
    }
}

pub struct Monster {
    pub name: String,
    pub kind: String,
    pub hp: i32,
}

impl Monster {
    pub fn new(name: String, kind: String) -> Monster {
        Monster { name, kind, hp: 50 }
    }
}
