// Contact Info for about page.
#[derive(Debug)]    
pub struct ContactInfo {
    pub is_gm: bool,
    pub name: &'static str,   
    pub discord: &'static str,   
    pub battlenet: &'static str,   
    pub discord_icon: &'static str,
    pub battlenet_icon: &'static str,
}

pub fn build_contacts() -> Vec<ContactInfo> {
    let discord_icon = "nf nf-fa-discord";
    let battlenet_icon = "nf nf-fa-battle_net";
        
    let contact_info = vec![
        ContactInfo {
            is_gm: true,
            name: "Dub",
            discord: ".whitechoc",
            battlenet: "WhiteChoc#1172",
            discord_icon: discord_icon,
            battlenet_icon:  battlenet_icon       
        },
        ContactInfo {
            is_gm: true,
            name: "Ladora",
            discord: "ladora",
            battlenet: "Ladora#1644",
            discord_icon: discord_icon,
            battlenet_icon:  battlenet_icon       
        },

        ContactInfo {
            is_gm: false,
            name: "Delusionol",
            discord: "delusionol",
            battlenet: "Delusionol#1509",
            discord_icon: discord_icon,
            battlenet_icon:  battlenet_icon       
        },

        ContactInfo {
            is_gm: false,
            name: "Kolzane",
            discord: "oogamama",
            battlenet: "Oogamama#1843",
            discord_icon: discord_icon,
            battlenet_icon:  battlenet_icon       
        },
        ContactInfo {
            is_gm: false,
            name: "Infi",
            discord: "windfi",
            battlenet: "Infi#11812",
            discord_icon: discord_icon,
            battlenet_icon:  battlenet_icon       
        },

    ];
    contact_info
}
