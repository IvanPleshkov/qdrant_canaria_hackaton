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

    Window1,
    Window2,

    Cup1,
    Cup2,
    Cup3,
    Cup4,

    Bottle1,
    Bottle2,
}

#[derive(Clone)]
pub struct EntityRef {
    pub name: String,
    pub objects: Vec<Entity>,
}

impl Entity {
    pub fn is_takeable(&self) -> bool {
        match self {
            Entity::Cup1
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
            Entity::Window1 => "Window",
            Entity::Window2 => "Window",
            Entity::Cup1 => "Cup",
            Entity::Cup2 => "Cup",
            Entity::Cup3 => "Cup",
            Entity::Cup4 => "Cup",
            Entity::Bottle1 => "Bottle",
            Entity::Bottle2 => "Bottle",
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
    let mut result = vec![
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
                Entity::Window1,
                Entity::Window2,
                Entity::Cup1,
                Entity::Cup2,
                Entity::Cup3,
                Entity::Cup4,
                Entity::Bottle1,
                Entity::Bottle2,
            ],
        },
        EntityRef {
            name: "Window".to_owned(),
            objects: vec![Entity::Window1, Entity::Window2],
        },
        EntityRef {
            name: "Cup".to_owned(),
            objects: vec![Entity::Cup1, Entity::Cup2, Entity::Cup3, Entity::Cup4],
        },
        EntityRef {
            name: "Bottle".to_owned(),
            objects: vec![Entity::Bottle1, Entity::Bottle1],
        },
    ];

    for names in get_arnaud_names() {
        result.push(EntityRef {
            name: format!("{} cup", names.name),
            objects: vec![Entity::Cup1],
        });
    }

    for names in get_luis_names() {
        result.push(EntityRef {
            name: format!("{} cup", names.name),
            objects: vec![Entity::Cup2],
        });
    }

    for names in get_roman_names() {
        result.push(EntityRef {
            name: format!("{} cup", names.name),
            objects: vec![Entity::Cup3],
        });
    }

    for names in get_tim_names() {
        result.push(EntityRef {
            name: format!("{} cup", names.name),
            objects: vec![Entity::Cup4],
        });
    }

    for names in get_ivan_names() {
        result.push(EntityRef {
            name: format!("{} bottle", names.name),
            objects: vec![Entity::Bottle1, Entity::Bottle2],
        });
    }

    result.push(EntityRef {
        name: "Left Window".to_owned(),
        objects: vec![Entity::Window2],
    });

    result.push(EntityRef {
        name: "Right Window".to_owned(),
        objects: vec![Entity::Window1],
    });

    result
}
