/**
 * Dati specie Pokedex gen 1–4 (id 1–493). Nomi Gen 1 (1–151) da elenco ufficiale;
 * 152–493 fallback "Pokémon {id}" fino a integrazione Knowledge DB.
 * Vedi docs/project/pokedex-personal.md, docs/project/knowledge-database.md.
 */

export type PokedexStatus = "unseen" | "seen" | "caught";

export interface SpeciesEntry {
	id: number;
	name: string;
}

/** Nomi Gen 1 (1–151) in ordine nazional Pokédex. */
const GEN1_NAMES: ReadonlyArray<string> = [
	"Bulbasaur", "Ivysaur", "Venusaur", "Charmander", "Charmeleon", "Charizard",
	"Squirtle", "Wartortle", "Blastoise", "Caterpie", "Metapod", "Butterfree",
	"Weedle", "Kakuna", "Beedrill", "Pidgey", "Pidgeotto", "Pidgeot",
	"Rattata", "Raticate", "Spearow", "Fearow", "Ekans", "Arbok",
	"Pikachu", "Raichu", "Sandshrew", "Sandslash", "Nidoran♀", "Nidorina",
	"Nidoqueen", "Nidoran♂", "Nidorino", "Nidoking", "Clefairy", "Clefable",
	"Vulpix", "Ninetales", "Jigglypuff", "Wigglytuff", "Zubat", "Golbat",
	"Oddish", "Gloom", "Vileplume", "Paras", "Parasect", "Venonat", "Venomoth",
	"Diglett", "Dugtrio", "Meowth", "Persian", "Psyduck", "Golduck",
	"Mankey", "Primeape", "Growlithe", "Arcanine", "Poliwag", "Poliwhirl", "Poliwrath",
	"Abra", "Kadabra", "Alakazam", "Machop", "Machoke", "Machamp",
	"Bellsprout", "Weepinbell", "Victreebel", "Tentacool", "Tentacruel",
	"Geodude", "Graveler", "Golem", "Ponyta", "Rapidash", "Slowpoke", "Slowbro",
	"Magnemite", "Magneton", "Farfetch'd", "Doduo", "Dodrio", "Seel", "Dewgong",
	"Grimer", "Muk", "Shellder", "Cloyster", "Gastly", "Haunter", "Gengar",
	"Onix", "Drowzee", "Hypno", "Krabby", "Kingler", "Voltorb", "Electrode",
	"Exeggcute", "Exeggutor", "Cubone", "Marowak", "Hitmonlee", "Hitmonchan",
	"Lickitung", "Koffing", "Weezing", "Rhyhorn", "Rhydon", "Chansey", "Tangela",
	"Kangaskhan", "Horsea", "Seadra", "Goldeen", "Seaking", "Staryu", "Starmie",
	"Mr. Mime", "Scyther", "Jynx", "Electabuzz", "Magmar", "Pinsir", "Tauros",
	"Magikarp", "Gyarados", "Lapras", "Ditto", "Eevee", "Vaporeon", "Jolteon", "Flareon",
	"Porygon", "Omanyte", "Omastar", "Kabuto", "Kabutops", "Aerodactyl", "Snorlax",
	"Articuno", "Zapdos", "Moltres", "Dratini", "Dragonair", "Dragonite", "Mewtwo", "Mew",
];

const GEN1_MAX = 151;
const GEN4_MAX = 493;

function getSpeciesName(id: number): string {
	if (id >= 1 && id <= GEN1_NAMES.length) {
		return GEN1_NAMES[id - 1];
	}
	return `Pokémon ${id}`;
}

/** Restituisce la lista specie per id da 1 a 493 (gen 1–4). */
export function getSpeciesList(): SpeciesEntry[] {
	const list: SpeciesEntry[] = [];
	for (let id = 1; id <= GEN4_MAX; id++) {
		list.push({ id, name: getSpeciesName(id) });
	}
	return list;
}

export { GEN4_MAX as POKEDEX_GEN4_MAX };
