use mightmagic::loaders::faction_loader::FactionLoader;
use mightmagic::models::faction::Faction;

fn main() {
    let mut factions: Vec<Faction> = Vec::new();
    FactionLoader::load(&mut factions);

    for faction in factions {
        println!("{}, {}", faction.creatures.0, faction.creatures.1);
    }
}
