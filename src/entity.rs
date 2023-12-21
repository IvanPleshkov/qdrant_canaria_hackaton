use std::vec;

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Entity {
    Andrey,
    Arnaud,
    Ivan,
    Luis,
    Tim,
    Roman,
    Kumar,

    Computer1,
    Computer2,
    Computer3,

    Lamp1,
    Lamp2,
    Lamp3,

    Chair1,
    Chair2,
    Chair3,
    Chair4,

    Plant1,
    Plant2,

    Table1,
    Table2,

    Window1,
    Window2,
    Door,

    Cooler,
    Printer,
    Picture1,
    Picture2,

    Cup1,
    Cup2,
    Cup3,
    Cup4,

    Bottle1,
    Bottle2,

    LeftEnd,
    RightEnd,
    Left,
    Right,
}

#[derive(Clone)]
pub struct EntityRef {
    pub name: String,
    pub objects: Vec<Entity>,
}

impl Entity {
    pub fn is_takeable(&self) -> bool {
        match self {
            Entity::Lamp1
            | Entity::Lamp2
            | Entity::Lamp3
            | Entity::Cup1
            | Entity::Cup2
            | Entity::Cup3
            | Entity::Cup4
            | Entity::Bottle1
            | Entity::Bottle2 => true,
            _ => false,
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            Entity::Andrey => "Andrey",
            Entity::Arnaud => "Arnaud",
            Entity::Ivan => "Ivan",
            Entity::Luis => "Luis",
            Entity::Tim => "Tim",
            Entity::Roman => "Roman",
            Entity::Kumar => "Kumar",
            Entity::Computer1 => "Computer",
            Entity::Computer2 => "Computer",
            Entity::Computer3 => "Computer",
            Entity::Lamp1 => "Lamp",
            Entity::Lamp2 => "Lamp",
            Entity::Lamp3 => "Lamp",
            Entity::Chair1 => "Chair",
            Entity::Chair2 => "Chair",
            Entity::Chair3 => "Chair",
            Entity::Chair4 => "Chair",
            Entity::Plant1 => "Plant",
            Entity::Plant2 => "Plant",
            Entity::Table1 => "Table",
            Entity::Table2 => "Table",
            Entity::Window1 => "Window",
            Entity::Window2 => "Window",
            Entity::Door => "Door",
            Entity::Cooler => "Cooler",
            Entity::Printer => "Printer",
            Entity::Picture1 => "Picture",
            Entity::Picture2 => "Picture",
            Entity::Cup1 => "Cup",
            Entity::Cup2 => "Cup",
            Entity::Cup3 => "Cup",
            Entity::Cup4 => "Cup",
            Entity::Bottle1 => "Bottle",
            Entity::Bottle2 => "Bottle",
            Entity::LeftEnd => "LeftEnd",
            Entity::RightEnd => "RightEnd",
            Entity::Left => "Left",
            Entity::Right => "Right",
        }
    }
}

pub fn get_all_entity_refs() -> Vec<EntityRef> {
    let mut result = vec![];

    result.extend(get_andrey_names());
    result.extend(get_arnaud_names());
    result.extend(get_ivan_names());
    result.extend(get_luis_names());
    result.extend(get_tim_names());
    result.extend(get_roman_names());
    result.extend(get_kumar_names());

    result.extend(get_item_names());

    result
}

fn get_andrey_names() -> Vec<EntityRef> {
    vec![
        EntityRef {
            name: "Andrey".to_owned(),
            objects: vec![Entity::Andrey],
        },
        EntityRef {
            name: "Vasnetsov".to_owned(),
            objects: vec![Entity::Andrey],
        },
        EntityRef {
            name: "Andrey Vasnetsov".to_owned(),
            objects: vec![Entity::Andrey],
        },
    ]
}

fn get_arnaud_names() -> Vec<EntityRef> {
    vec![
        EntityRef {
            name: "Arnaud".to_owned(),
            objects: vec![Entity::Arnaud],
        },
        EntityRef {
            name: "Gourlay".to_owned(),
            objects: vec![Entity::Arnaud],
        },
        EntityRef {
            name: "Arnaud Gourlay".to_owned(),
            objects: vec![Entity::Arnaud],
        },
    ]
}

fn get_ivan_names() -> Vec<EntityRef> {
    vec![
        EntityRef {
            name: "Ivan".to_owned(),
            objects: vec![Entity::Ivan],
        },
        EntityRef {
            name: "Pleshkov".to_owned(),
            objects: vec![Entity::Ivan],
        },
        EntityRef {
            name: "Ivan Pleshkov".to_owned(),
            objects: vec![Entity::Ivan],
        },
    ]
}

fn get_luis_names() -> Vec<EntityRef> {
    vec![
        EntityRef {
            name: "Luis".to_owned(),
            objects: vec![Entity::Luis],
        },
        EntityRef {
            name: "Cossío".to_owned(),
            objects: vec![Entity::Luis],
        },
        EntityRef {
            name: "Cossio".to_owned(),
            objects: vec![Entity::Luis],
        },
        EntityRef {
            name: "Luis Cossío".to_owned(),
            objects: vec![Entity::Luis],
        },
        EntityRef {
            name: "Luis Cossio".to_owned(),
            objects: vec![Entity::Luis],
        },
    ]
}

fn get_tim_names() -> Vec<EntityRef> {
    vec![
        EntityRef {
            name: "Tim".to_owned(),
            objects: vec![Entity::Tim],
        },
        EntityRef {
            name: "Visée".to_owned(),
            objects: vec![Entity::Tim],
        },
        EntityRef {
            name: "Visee".to_owned(),
            objects: vec![Entity::Tim],
        },
        EntityRef {
            name: "Tim Visée".to_owned(),
            objects: vec![Entity::Tim],
        },
        EntityRef {
            name: "Tim Visee".to_owned(),
            objects: vec![Entity::Tim],
        },
    ]
}

fn get_roman_names() -> Vec<EntityRef> {
    vec![
        EntityRef {
            name: "Roman".to_owned(),
            objects: vec![Entity::Roman],
        },
        EntityRef {
            name: "Titov".to_owned(),
            objects: vec![Entity::Roman],
        },
        EntityRef {
            name: "Roman Titov".to_owned(),
            objects: vec![Entity::Roman],
        },
    ]
}

fn get_kumar_names() -> Vec<EntityRef> {
    vec![
        EntityRef {
            name: "Kumar".to_owned(),
            objects: vec![Entity::Kumar],
        },
        EntityRef {
            name: "Shivendu".to_owned(),
            objects: vec![Entity::Kumar],
        },
        EntityRef {
            name: "Kumar Shivendu".to_owned(),
            objects: vec![Entity::Kumar],
        },
    ]
}

fn get_item_names() -> Vec<EntityRef> {
    vec![
        EntityRef {
            name: "Person".to_owned(),
            objects: vec![
                // Entity::Andrey,
                Entity::Arnaud,
                Entity::Ivan,
                Entity::Luis,
                Entity::Tim,
                Entity::Roman,
                Entity::Kumar,
            ],
        },
        EntityRef {
            name: "This".to_owned(),
            objects: vec![
                Entity::Computer1,
                Entity::Computer2,
                Entity::Computer3,
                Entity::Lamp1,
                Entity::Lamp2,
                Entity::Lamp3,
                Entity::Chair1,
                Entity::Chair2,
                Entity::Chair3,
                Entity::Chair4,
                Entity::Plant1,
                Entity::Plant2,
                Entity::Table1,
                Entity::Table2,
                Entity::Window1,
                Entity::Window2,
                Entity::Door,
                Entity::Cooler,
                Entity::Printer,
                Entity::Picture1,
                Entity::Picture2,
                Entity::Cup1,
                Entity::Cup2,
                Entity::Cup3,
                Entity::Cup4,
                Entity::Bottle1,
                Entity::Bottle2,
            ],
        },
        EntityRef {
            name: "Computer".to_owned(),
            objects: vec![Entity::Computer1, Entity::Computer2, Entity::Computer3],
        },
        EntityRef {
            name: "Lamp".to_owned(),
            objects: vec![Entity::Lamp1, Entity::Lamp2, Entity::Lamp3],
        },
        EntityRef {
            name: "Chair".to_owned(),
            objects: vec![
                Entity::Chair1,
                Entity::Chair2,
                Entity::Chair3,
                Entity::Chair4,
            ],
        },
        EntityRef {
            name: "Plant".to_owned(),
            objects: vec![Entity::Plant1, Entity::Plant2],
        },
        EntityRef {
            name: "Table".to_owned(),
            objects: vec![Entity::Table1, Entity::Table2],
        },
        EntityRef {
            name: "Window".to_owned(),
            objects: vec![Entity::Window1, Entity::Window2],
        },
        EntityRef {
            name: "Door".to_owned(),
            objects: vec![Entity::Door],
        },
        EntityRef {
            name: "Cooler".to_owned(),
            objects: vec![Entity::Cooler],
        },
        EntityRef {
            name: "Printer".to_owned(),
            objects: vec![Entity::Printer],
        },
        EntityRef {
            name: "Picture".to_owned(),
            objects: vec![Entity::Picture1, Entity::Picture2],
        },
        EntityRef {
            name: "Cup".to_owned(),
            objects: vec![Entity::Cup1, Entity::Cup2, Entity::Cup3, Entity::Cup4],
        },
        EntityRef {
            name: "Bottle".to_owned(),
            objects: vec![Entity::Bottle1, Entity::Bottle1],
        },
    ]
}
