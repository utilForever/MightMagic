use mightmagic::loaders::faction_loader::FactionLoader;
use mightmagic::models::faction::Faction;

fn print_splash() {
    println!("{}", "
     __  __ _       _     _   __  __             _      
    |  \\/  (_)     | |   | | |  \\/  |           (_)     
    | \\  / |_  __ _| |__ | |_| \\  / | __ _  __ _ _  ___ 
    | |\\/| | |/ _` | '_ \\| __| |\\/| |/ _` |/ _` | |/ __|
    | |  | | | (_| | | | | |_| |  | | (_| | (_| | | (__ 
    |_|  |_|_|\\__, |_| |_|\\__|_|  |_|\\__,_|\\__, |_|\\___|
               __/ |                        __/ |       
              |___/                        |___/        ");
}

fn main() {
    print_splash();

    let mut factions: Vec<Faction> = Vec::new();
    FactionLoader::load(&mut factions);

    for faction in factions {
        println!("{}, {}", faction.creatures.0, faction.creatures.1);
    }
}
