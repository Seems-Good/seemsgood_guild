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
        // tanks
        Player { name: "Celinka", class: PlayerClass::Monk, realm: "Illidan" },
        Player { name: "Hardwhare", class: PlayerClass::Warrior, realm: "Illidan" },
        // healers
        Player { name: "Alkirie", class: PlayerClass::Druid, realm: "Illidan" },
        Player { name: "Evethyr", class: PlayerClass::Evoker, realm: "Kel'Thuzad" },
        Player { name: "Purplesneeze", class: PlayerClass::Evoker, realm: "Stormrage" },
        Player { name: "Delusionol", class: PlayerClass::Priest, realm: "Stormrage" },
        Player { name: "Holydora", class: PlayerClass::Paladin, realm: "Stormrage" },
        // dps
        Player { name: "Oldmanzand", class: PlayerClass::Shaman, realm: "Illidan" },
        Player { name: "Fliptwisty", class: PlayerClass::DemonHunter, realm: "Stormrage" },
        Player { name: "Notshodo", class: PlayerClass::Evoker, realm: "Stormrage" },
        Player { name: "Nuzzler", class: PlayerClass::Druid, realm: "Stormrage" },
        Player { name: "Nyanslok", class: PlayerClass::Warlock, realm: "Stormrage" },
        Player { name: "Obiscuit", class: PlayerClass::DeathKnight, realm: "Stormrage" },
        Player { name: "Ppddk", class: PlayerClass::DeathKnight, realm: "Stormrage" },
        Player { name: "Rogerport", class: PlayerClass::Mage, realm: "Stormrage" },
        Player { name: "Classylad", class: PlayerClass::Rogue, realm: "Thrall" },
        Player { name: "Prankbear", class: PlayerClass::Shaman, realm: "Thrall" },
        Player { name: "Rektribute", class: PlayerClass::Paladin, realm: "Thrall" },
        Player { name: "Caael", class: PlayerClass::DemonHunter, realm: "Tichondrius" },
        Player { name: "Harbormastr", class: PlayerClass::Evoker, realm: "Tichondrius" },
        Player { name: "Pipfi", class: PlayerClass::Mage, realm: "Tichondrius" },
        Player { name: "Kolzane", class: PlayerClass::Hunter, realm: "Ysera" },
        Player { name: "Decension", class: PlayerClass::DemonHunter, realm: "Area 52" },
        Player { name: "Infi", class: PlayerClass::Hunter, realm: "Azshara" },
        Player { name: "Jaycelock", class: PlayerClass::Warlock, realm: "Bleeding Hollow" }
    ];



    // Rendering the template with the player data
    let template = RaidFramesTemplate { 
        base: BaseTemplate::new(true),
        players
    };
    let rendered = template.render().unwrap();
    axum::response::Html(rendered)
}
