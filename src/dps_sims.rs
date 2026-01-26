use axum::response::Html;
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
    sim_url: &'static str,
}

#[derive(Template)]
#[template(path = "dps-sims.html")]
struct DamageSimsTemplate {
    base: BaseTemplate,
    players: Vec<Player>,
}

pub async fn damagesimspage() -> Html<String> {
    let players = vec![
        Player {
            name: "Nuzz",
            class: PlayerClass::Rogue,
            sim_url: "https://r2.seemsgood.org/roster/Nuzzsin.html",
        },
        Player {
            name: "Infi",
            class: PlayerClass::Mage,
            sim_url: "https://r2.seemsgood.org/roster/Infilicious.html",
        },
        Player {
            name: "Shodo",
            class: PlayerClass::Evoker,
            sim_url: "https://r2.seemsgood.org/roster/Notshodo.html",
        },
        Player {
            name: "Chint",
            class: PlayerClass::DemonHunter,
            sim_url: "https://r2.seemsgood.org/roster/Chinterfel.html",
        },
        Player {
            name: "Roger",
            class: PlayerClass::Druid,
            sim_url: "https://r2.seemsgood.org/roster/Bigtittyrog.html",
        },
        Player {
            name: "Chuubers",
            class: PlayerClass::Warrior,
            sim_url: "https://r2.seemsgood.org/roster/Chuubers.html",
        },
        Player {
            name: "Delulu",
            class: PlayerClass::Priest,
            sim_url: "https://r2.seemsgood.org/roster/Delusionil.html",
        },
        Player {
            name: "Filio",
            class: PlayerClass::Monk,
            sim_url: "https://r2.seemsgood.org/roster/Filio.html",
        },
        Player {
            name: "Jakk",
            class: PlayerClass::Paladin,
            sim_url: "https://r2.seemsgood.org/roster/Jakksparrow.html",
        },
        Player {
            name: "Dub",
            class: PlayerClass::Shaman,
            sim_url: "https://r2.seemsgood.org/roster/Dubshamm.html",
        },
        Player {
            name: "Hek",
            class: PlayerClass::Mage,
            sim_url: "https://r2.seemsgood.org/roster/Hekthuzad.html",
        },
        Player {
            name: "Lan",
            class: PlayerClass::Warlock,
            sim_url: "https://r2.seemsgood.org/roster/Lanathallan.html",
        },
        Player {
            name: "James",
            class: PlayerClass::Warrior,
            sim_url: "https://r2.seemsgood.org/roster/Jaemsy.html",
        },
        Player {
            name: "Ppd",
            class: PlayerClass::Rogue,
            sim_url: "https://r2.seemsgood.org/roster/Ppdx.html",
        },
        Player {
            name: "Vinnea",
            class: PlayerClass::Shaman,
            sim_url: "https://r2.seemsgood.org/roster/Vinnea.html",
        },
        Player {
            name: "Ladora",
            class: PlayerClass::Evoker,
            sim_url: "https://r2.seemsgood.org/roster/Spyrodora.html",
        },
        Player {
            name: "Kael",
            class: PlayerClass::Hunter,
            sim_url: "https://r2.seemsgood.org/roster/Kaelirious.html",
        },
        Player {
            name: "Nyans",
            class: PlayerClass::Warlock,
            sim_url: "https://r2.seemsgood.org/roster/Nyanslok.html",
        },
        // no sim currently, needed a deathknight to remove warning for being unused in enum
        Player {
            name: "Cryptic",
            class: PlayerClass::DeathKnight,
            sim_url: "https://r2.seemsgood.org/roster/Sodo.html",
        },

    ];

    let template = DamageSimsTemplate{ 
        base: BaseTemplate::new(true),
        players 
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}
