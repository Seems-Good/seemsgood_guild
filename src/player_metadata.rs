#[derive(Debug)]
pub enum PlayerRole {
    Tank,
    Healer,
    Dps,
}

// map a PlayerRole to a known icon url.
impl PlayerRole {
    pub fn icon_url(&self) -> &'static str {
        match self {
            PlayerRole::Tank => "https://r2.seemsgood.org/content/icons/tank.png",
            PlayerRole::Healer => "https://r2.seemsgood.org/content/icons/healer.png",
            PlayerRole::Dps => "https://r2.seemsgood.org/content/icons/dps.png",
        }
    }
}

// map a PlayerRole to its string value so 'player.role.tank' would format to 'tank' string.
// if a icon url cant resolve or render on a device, it will default to 
impl std::fmt::Display for PlayerRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role_str = match self {
            PlayerRole::Tank => "Tank",
            PlayerRole::Healer => "Healer",
            PlayerRole::Dps => "Dps",
        };
        write!(f, "{}", role_str)
    }
}

#[derive(Debug)]
pub enum PlayerClass {
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
    pub fn rgb(&self) -> &'static str {
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

impl std::fmt::Display for PlayerClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class_str = match self {
            PlayerClass::DeathKnight => "DeathKnight",
            PlayerClass::DemonHunter => "DemonHunter",
            PlayerClass::Druid => "Druid",
            PlayerClass::Evoker => "Evoker",
            PlayerClass::Hunter => "Hunter",
            PlayerClass::Mage => "Mage",
            PlayerClass::Monk => "Monk",
            PlayerClass::Paladin => "Paladin",
            PlayerClass::Priest => "Priest",
            PlayerClass::Rogue => "Rogue",
            PlayerClass::Shaman => "Shaman",
            PlayerClass::Warlock => "Warlock",
            PlayerClass::Warrior => "Warrior",
        };
        write!(f, "{}", class_str)
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: &'static str,
    pub class: PlayerClass,
    pub realm: &'static str,
    pub role: PlayerRole,
}


#[derive(Debug)]
pub struct RaidMetaData {
   pub fight_name: &'static str,
   pub season: &'static str,
   pub expansion: &'static str,
   pub group_photo: &'static str,
   pub log_id: &'static str,
   pub datetime: &'static str,
   pub pretty_datetime: &'static str,
   pub fight_key: &'static str,
   pub fight_is_video: bool, 

}

/// include a video:
/// - set fight_is_video: true
/// - ensure video in r2 follows pattern: $fight_name-kill-video.mp4
/// TODO: ideally this takes a json file with a arbitrary amount of RaidMetaData structs.
pub fn build_raid() -> Vec<RaidMetaData> {
    let raid_metadata = vec![
        RaidMetaData {
            fight_name: "Dimensius",
            season: "Season 3",
            expansion: "The War Within",
            group_photo: "dimensius-kill",
            log_id: "Nmh3PAJ6kzYKGb2D",
            datetime: "2025-12-18",
            pretty_datetime: "10:45pm - 18 December 2025",
            fight_key: "Dimensius",
            fight_is_video: true,
        },
        RaidMetaData {
            fight_name: "Gallywix",
            season: "Season 2",
            expansion: "The War Within",
            group_photo: "gallywix-kill-group.png",
            log_id: "FBvTzZPLVmdApbN6",
            datetime: "2025-06-26",
            pretty_datetime: "9:13pm - 26 June 2025",
            fight_key: "Gallywix",
            fight_is_video: false,
        },
        RaidMetaData {
            fight_name: "Kyvesa",
            season: "Season 1",
            expansion: "The War Within",
            group_photo: "kyvesa-kill", 
            log_id: "Lfx3nrBVRWtNFzMQ",
            datetime: "2024-12-12",
            pretty_datetime: "10:15pm - 12 December 2024",
            fight_key: "Kyvesa",
            fight_is_video: true,
        },
        RaidMetaData {
            fight_name: "Fyrakk",
            season: "Season 4",
            expansion: "Dragonflight",
            group_photo: "fyrakk-group-pic.jpg",
            log_id: "F8fxkdGnVQmRNCJrv",
            datetime: "2024-07-01",
            pretty_datetime: "10:03pm - 1 July 2024",
            fight_key: "Fyrakk",
            fight_is_video: false,
        },

    ];
    raid_metadata
}

// Rosters are populated here. We could read a DB or config file of { $SEASON, { $Player, { $name, $class, $realm, $role }}}
// in future this will take a .json from wowaudit api to translate roster -> Player struct fields
pub fn build_roster(id: &str) -> Vec<Player> {
    // TODO: Throw a error here, we should always show _some_ roster not a fake default user.
    let default = vec![
        Player { name: "DefaultName", class: PlayerClass::Paladin, realm: "DefaultRealm", role: PlayerRole::Tank },

    ];

    // ----[ Season 4 DF Card ]----
    let fyrakk = vec![
        // Tanks
        Player { name: "Jugsmcgee", class: PlayerClass::DeathKnight, realm: "Stormrage", role: PlayerRole::Tank },
        Player { name: "Rogermeta", class: PlayerClass::DemonHunter, realm: "Stormrage", role: PlayerRole::Tank },

        // Healers
        Player { name: "Evelianne", class: PlayerClass::Monk, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Shdo", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Delusionol", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Sylvána", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Healer },

        // DPS
        Player { name: "Tusknight", class: PlayerClass::DeathKnight, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Amarelysa", class: PlayerClass::DemonHunter, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Fliptwisty", class: PlayerClass::DemonHunter, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Nuzzler", class: PlayerClass::Druid, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Nyansev", class: PlayerClass::Evoker, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Jennatullz", class: PlayerClass::Hunter, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Kaelirious", class: PlayerClass::Hunter, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Infilicious", class: PlayerClass::Mage, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Jakksparrow", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Paliduh", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Wallysaurous", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Nicechint", class: PlayerClass::Rogue, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Lanathallan", class: PlayerClass::Warlock, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Contradict", class: PlayerClass::Warrior, realm: "Stormrage", role: PlayerRole::Dps },
        ];
    
    // ----[ Season 1 TWW Card ]----
    let kyvesa = vec![
        // Tanks
        Player { name: "Crypticist", class: PlayerClass::DeathKnight, realm: "Zul'jin", role: PlayerRole::Tank },
        Player { name: "Paliduh", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Tank },
        // Healers
        Player { name: "Notshodo", class: PlayerClass::Evoker, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Evelianne", class: PlayerClass::Monk, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Spera", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Delusionil", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Piptide", class: PlayerClass::Shaman, realm: "Tichondrius", role: PlayerRole::Healer },
        // DPS
        Player { name: "Rogergrowth", class: PlayerClass::Druid, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Kolzane", class: PlayerClass::Hunter, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Hekthuzad", class: PlayerClass::Mage, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Infilicious", class: PlayerClass::Mage, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Filio", class: PlayerClass::Monk, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Quelstyle", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Emlay", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Vinneya", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Ppdx", class: PlayerClass::Rogue, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Dubshamm", class: PlayerClass::Shaman, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Lanathallan", class: PlayerClass::Warlock, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Nyanslok", class: PlayerClass::Warlock, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Chuubers", class: PlayerClass::Warrior, realm: "Stormrage", role: PlayerRole::Dps },
    ];

    // ----[ Season 2 TWW Card ]----
    let gallywix = vec![
        // Tanks
        Player { name: "Whare", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Tank },
        Player { name: "Jaemsy", class: PlayerClass::Warrior, realm: "Stormrage", role: PlayerRole::Tank },
        // Healers
        Player { name: "Pipmeow", class: PlayerClass::Druid, realm: "Tichondrius", role: PlayerRole::Healer },
        Player { name: "Evelianne", class: PlayerClass::Monk, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Delusionol", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Oldmanzand", class: PlayerClass::Shaman, realm: "Illidan", role: PlayerRole::Healer },
        // DPS
        Player { name: "Obiscuit", class: PlayerClass::DeathKnight, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Jedh", class: PlayerClass::DemonHunter, realm: "Dalaran", role: PlayerRole::Dps },
        Player { name: "Nuzzler", class: PlayerClass::Druid, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Rogergrowth", class: PlayerClass::Druid, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Indico", class: PlayerClass::Evoker, realm: "Zul'jin", role: PlayerRole::Dps },
        Player { name: "Notshodo", class: PlayerClass::Evoker, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Kolzane", class: PlayerClass::Hunter, realm: "Ysera", role: PlayerRole::Dps },
        Player { name: "Stormßreeð", class: PlayerClass::Hunter, realm: "Thrall", role: PlayerRole::Dps },
        Player { name: "Infilicious", class: PlayerClass::Mage, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Filio", class: PlayerClass::Monk, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Ppdx", class: PlayerClass::Rogue, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Dubshamm", class: PlayerClass::Shaman, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Nyanslok", class: PlayerClass::Warlock, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Aphitari", class: PlayerClass::Warrior, realm: "Stormrage", role: PlayerRole::Dps },
    ];

    // ----[ Season 3 TWW Card ]----
    let dimensius = vec![
        // Tanks
        Player { name: "Whare", class: PlayerClass::Paladin, realm: "Stormrage", role: PlayerRole::Tank },
        Player { name: "Purpformance", class: PlayerClass::Monk, realm: "Proudmoore", role: PlayerRole::Tank },
        // Healers
        Player { name: "Piptide", class: PlayerClass::Shaman, realm: "Tichondrius", role: PlayerRole::Healer },
        Player { name: "Evelianne", class: PlayerClass::Monk, realm: "Stormrage", role: PlayerRole::Healer },
        Player { name: "Philfishh", class: PlayerClass::Monk, realm: "Area-52", role: PlayerRole::Healer },
        Player { name: "Delusionol", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Healer },
        // DPS
        Player { name: "Obiscuit", class: PlayerClass::DeathKnight, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Fliptwisty", class: PlayerClass::DemonHunter, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Ovtlaw", class: PlayerClass::Rogue, realm: "Dalaran", role: PlayerRole::Dps },
        Player { name: "Nuzzler", class: PlayerClass::Druid, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Rogerport", class: PlayerClass::Mage, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Indico", class: PlayerClass::Evoker, realm: "Zul'jin", role: PlayerRole::Dps },
        Player { name: "Prankdaddy", class: PlayerClass::Evoker, realm: "Thrall", role: PlayerRole::Dps },
        Player { name: "Kolzane", class: PlayerClass::Hunter, realm: "Ysera", role: PlayerRole::Dps },
        Player { name: "Stormßreeð", class: PlayerClass::Hunter, realm: "Thrall", role: PlayerRole::Dps },
        Player { name: "Infilicious", class: PlayerClass::Mage, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Ppddk", class: PlayerClass::DeathKnight, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Purpleheal", class: PlayerClass::Priest, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Nyanslok", class: PlayerClass::Warlock, realm: "Stormrage", role: PlayerRole::Dps },
        Player { name: "Aphitari", class: PlayerClass::Warrior, realm: "Stormrage", role: PlayerRole::Dps },
    ];

    
     
    let id_str = match id {
        "Dimensius" => dimensius,
        "Gallywix" => gallywix,
        "Kyvesa" => kyvesa,
        "Fyrakk" => fyrakk,
        _ => default,
    };
    id_str
}


/* TODO use this api to get roster - 
 * curl -X 'GET' https://wowaudit.com/v1/characters -H 'accept: application/json' -H 'Authorization: $KEY' |  jq '.[] | select(.rank == "Officer" or .rank == "Raider") | {name, realm, class, rank}'
*/
