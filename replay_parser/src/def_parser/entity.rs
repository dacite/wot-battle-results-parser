pub struct Entity {
    name: String,
    properties: String,
    volatiles: String,
    methods: String,
}

impl Entity {
    pub fn parse_entity_from_file(name: &str, game_version: [u16; 4]) -> wot_replay_parser::Result<Entity> {
        let game_version = game_version.map(|x| x.to_string()).join("_");
        let ent_def_path = format!("definitions/{game_version}/{name}.def");
        let interface_path = format!("definitions/{game_version}/interfaces/");
    
    
        unimplemented!()
    }
}