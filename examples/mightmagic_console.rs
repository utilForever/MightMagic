use mightmagic::managers::resource_manager::ResourceManager;

fn print_splash() {
    println!(
        "{}",
        "     __  __ _       _     _   __  __             _      
    |  \\/  (_)     | |   | | |  \\/  |           (_)     
    | \\  / |_  __ _| |__ | |_| \\  / | __ _  __ _ _  ___ 
    | |\\/| | |/ _` | '_ \\| __| |\\/| |/ _` |/ _` | |/ __|
    | |  | | | (_| | | | | |_| |  | | (_| | (_| | | (__ 
    |_|  |_|_|\\__, |_| |_|\\__|_|  |_|\\__,_|\\__, |_|\\___|
               __/ |                        __/ |       
              |___/                        |___/        "
    );
}

fn main() {
    ResourceManager::initialize();

    print_splash();

    let factions = ResourceManager::get_all_factions();
    for faction in factions.iter() {
        for creature in faction.creatures.iter() {
            println!("{}, {}", creature.0, creature.1);
        }
    }
}
