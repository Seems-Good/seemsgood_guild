use askama_axum::Template;
use crate::BaseTemplate;

#[derive(Debug)]
enum PlayerClass {
    Warrior,
    Mage,
    Rogue,
    Hunter,
    Druid,
    Paladin,
    Priest,
    Warlock,
    Monk,
    DeathKnight,
    Shaman,
    DemonHunter,
    Evoker,
}

impl PlayerClass {
    fn rgb(&self) -> &'static str {
        match self {
            PlayerClass::DeathKnight => "rgb(196, 30, 58)",
            PlayerClass::DemonHunter => "rgb(163, 48, 201)",
            PlayerClass::Druid => "rgb(255, 124, 10)",
            PlayerClass::Evoker => "rgb(51, 147, 127)",
            PlayerClass::Hunter => "rgb(170, 211, 114)",
            PlayerClass::Mage => "rgb(63, 199, 235)",
            PlayerClass::Monk => "rgb(0, 255, 152)",
            PlayerClass::Paladin => "rgb(244, 140, 186)",
            PlayerClass::Priest => "rgb(255, 255, 255)",
            PlayerClass::Rogue => "rgb(255, 244, 104)",
            PlayerClass::Shaman => "rgb(0, 112, 221)",
            PlayerClass::Warlock => "rgb(135, 136, 238)",
            PlayerClass::Warrior => "rgb(198, 155, 109)",
        }
    }
}

#[derive(Debug)]
struct Player {
    name: &'static str,
    class: PlayerClass,
    realm: &'static str,
}

#[derive(Template)]
#[template(path = "mythic-plus.html")]
struct RaidFramesTemplate {
    base: BaseTemplate,
    players: Vec<Player>,
}


pub async fn mythicplus_page() -> axum::response::Html<String> {
    // Creating the list of players, including class RGB
    let players = vec![
        Player { name: "Aphitari", class: PlayerClass::Warrior, realm: "Area 52" },
        Player { name: "Philfishh", class: PlayerClass::Monk, realm: "Area 52" },
        Player { name: "Ovtlaw", class: PlayerClass::Rogue, realm: "Dalaran" },
        Player { name: "Oldmanzand", class: PlayerClass::Shaman, realm: "Illidan" },
        Player { name: "Ppddk", class: PlayerClass::DeathKnight, realm: "Mal'Ganis" },
        Player { name: "Delusionol", class: PlayerClass::Priest, realm: "Stormrage" },
        Player { name: "Evelianne", class: PlayerClass::Monk, realm: "Stormrage" },
        Player { name: "Fliptwisty", class: PlayerClass::DemonHunter, realm: "Stormrage" },
        Player { name: "Infilicious", class: PlayerClass::Mage, realm: "Stormrage" },
        Player { name: "Notshodo", class: PlayerClass::Evoker, realm: "Stormrage" },
        Player { name: "Nuzzler", class: PlayerClass::Druid, realm: "Stormrage" },
        Player { name: "Nyanslok", class: PlayerClass::Warlock, realm: "Stormrage" },
        Player { name: "Obiscuit", class: PlayerClass::DeathKnight, realm: "Stormrage" },
        Player { name: "Ppdx", class: PlayerClass::Rogue, realm: "Stormrage" },
        Player { name: "Purpleheal", class: PlayerClass::Priest, realm: "Stormrage" },
        Player { name: "Rogerport", class: PlayerClass::Mage, realm: "Stormrage" },
        Player { name: "Whare", class: PlayerClass::Paladin, realm: "Stormrage" },
        Player { name: "Prankdaddy", class: PlayerClass::Evoker, realm: "Thrall" },
        Player { name: "Rektribute", class: PlayerClass::Paladin, realm: "Thrall" },
        Player { name: "Stormbreed", class: PlayerClass::Hunter, realm: "Thrall" },
        Player { name: "Piptide", class: PlayerClass::Shaman, realm: "Tichondrius" },
        Player { name: "Kolzane", class: PlayerClass::Hunter, realm: "Ysera" },
        Player { name: "Indico", class: PlayerClass::Evoker, realm: "Zul'jin" },

        // // tanks
        // Player { name: "Jtusendh", class: PlayerClass::DemonHunter, realm: "Stormrage" },
        // Player { name: "Paliduh", class: PlayerClass::Paladin, realm: "Stormrage" },
        // // healers
        // Player { name: "Delusionol", class: PlayerClass::Priest, realm: "Stormrage" },
        // Player { name: "Evelianne", class: PlayerClass::Monk, realm: "Stormrage" },
        // Player { name: "Pipmeow", class: PlayerClass::Druid, realm: "Tichondrius" },
        // Player { name: "Auraelia", class: PlayerClass::Priest, realm: "Zul'jin" },
        // Player { name: "Caael", class: PlayerClass::Paladin, realm: "Zul'jin" },
        // // damage 
        // Player { name: "Infilicious", class: PlayerClass::Mage, realm: "Stormrage" },
        // Player { name: "Hekthuzad", class: PlayerClass::Mage, realm: "Stormrage" },
        // Player { name: "Indico", class: PlayerClass::Evoker, realm: "Zul'jin" },
        // Player { name: "Ladöra", class: PlayerClass::Priest, realm: "Stormrage" },
        // Player { name: "Lanathallan", class: PlayerClass::Warlock, realm: "Stormrage" },
        // Player { name: "Notshodo", class: PlayerClass::Evoker, realm: "Stormrage" },
        // Player { name: "Kolzane", class: PlayerClass::Hunter, realm: "Ysera" },
        // Player { name: "Speara", class: PlayerClass::Druid, realm: "Kel'Thuzad" },
        // Player { name: "Nyanslok", class: PlayerClass::Warlock, realm: "Stormrage" },
        // Player { name: "Dubshamm", class: PlayerClass::Shaman, realm: "Stormrage" },
        // Player { name: "Chuubers", class: PlayerClass::Warrior, realm: "Stormrage" },
        // Player { name: "Conncrete", class: PlayerClass::Rogue, realm: "Tichondrius" },
        // Player { name: "Filio", class: PlayerClass::Monk, realm: "Stormrage" },
        // Player { name: "Jakksparrow", class: PlayerClass::Paladin, realm: "Stormrage" },
        // Player { name: "Juukmonk", class: PlayerClass::Monk, realm: "Zul'jin" },
        // Player { name: "Ppdx", class: PlayerClass::Rogue, realm: "Stormrage" },
        // Player { name: "Queldk", class: PlayerClass::DeathKnight, realm: "Zul'jin" },
    ];



    // Rendering the template with the player data
    let template = RaidFramesTemplate { 
        base: BaseTemplate::new(true),
        players
    };
    let rendered = template.render().unwrap();
    axum::response::Html(rendered)
}
