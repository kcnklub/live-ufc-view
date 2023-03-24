use tauri::utils::assets::phf::phf_map;

pub static PHOTOS: phf::Map<&'static str, &'static str> = phf_map! {
    "marlon vera" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/3155424.png",
    "cory sandhagen" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4294504.png",
    "holly holm" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/3028404.png",
    "yana santos" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/3154898.png",
    "nate landwehr" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/3821549.png",
    "austin lingo" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4570669.png",
    "andrea lee" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/3153106.png",
    "maycee barber" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4246307.png",
    "alex perez" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/3155425.png",
    "manel kape" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4236504.png",
    "chidi njokuani" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/2959422.png",
    "albert duraev" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/3088682.png",
    "daniel pineda" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/2514828.png",
    "tucker lutz" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4684678.png",
    "steven peterson" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/3041138.png",
    "lucas alexander" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/5077398.png",
    "trevin giles" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4024732.png",
    "preston parsons" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4038105.png",
    "cj vergara" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4396359.png",
    "daniel lacerda" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4895772.png",
    "victor altamirano" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4713584.png",
    "vinicius salvador" => "https://a.espncdn.com/combiner/i?img=/i/headshots/mma/players/full/4869425.png",
};

pub fn get_photo(name: &str) -> String {
    match PHOTOS.get(name.to_lowercase().as_str()).copied() {
        Some(value) => String::from(value),
        None => String::from("https://secure.espncdn.com/combiner/i?img=/i/headshots/nophoto.png"),
    }
}
