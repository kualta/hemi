use dioxus::html::input_data::keyboard_types::Code;
use dioxus::html::input_data::keyboard_types::Key;
use rand::seq::SliceRandom;
use std::any::Any;
use std::{collections::HashMap, str::FromStr, vec::Vec};
use web_sys::HtmlAudioElement;

const LEFT_QWERTY_KEYS: &str = "QWERT ASDFG ZXCVB";
const RIGHT_QWERTY_KEYS: &str = "YUIOP HJKL\' NM,./";
const PUBLIC_URL: &str = "/HemiTyper/";

/// Stores pressed state of keys
#[derive(PartialEq)]
pub(crate) struct KeyState {
    key: Key,
    enabled: bool,
}

impl KeyState {
    pub(crate) fn new(key: &Key, enabled: bool) -> Self {
        KeyState {
            key: key.clone(),
            enabled,
        }
    }

    pub(crate) fn key(&self) -> &Key {
        &self.key
    }

    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }
}

/// Stores rows of [`KeyState`]s for the keyboard
pub(crate) struct KeyboardState {
    keys: Vec<Vec<KeyState>>,
}

impl KeyboardState {
    pub(crate) fn new(dictionary: &WordDictionary) -> Self {
        let keys = dictionary
            .keys()
            .split_whitespace()
            .map(|row| {
                row.chars()
                    .map(|key| KeyState {
                        key: Key::from_str(&key.to_string()).expect("Non-existent key supplied"),
                        enabled: false,
                    })
                    .collect()
            })
            .collect();

        KeyboardState { keys }
    }

    pub(crate) fn update_for(&mut self, key: &KeyState) {
        self.keys.iter_mut().for_each(|row| {
            if let Some(key_state) = row.iter_mut().find(|key_state| {
                // FIXME: slow, ugly, stupid
                key_state.key.to_string().to_uppercase() == key.key.to_string().to_uppercase()
            }) {
                key_state.enabled = key.enabled
            }
        });
    }

    pub(crate) fn keys(&self) -> &Vec<Vec<KeyState>> {
        self.keys.as_ref()
    }
}

/// Stores dictionaries of words and keys they consist of.
///
/// # Note
/// `keys` is expected to be a whitespace-separated uppercase sequence of key rows
pub(crate) struct WordDictionary<'a> {
    buffer: Vec<&'a str>,
    keys: String,
}

impl<'a> WordDictionary<'a> {
    pub(crate) fn keys(&self) -> &str {
        self.keys.as_ref()
    }
}

/// Maps Key [`Code`] to audio file path
pub(crate) struct AudioLibrary {
    sounds: HashMap<Code, String>,
}

impl Default for AudioLibrary {
    fn default() -> Self {
        let path = PUBLIC_URL.to_owned() + "assets/tealios/";
        let extra: Vec<String> = vec![
            "Space".to_owned(),
            "Enter".to_owned(),
            "Backspace".to_owned(),
        ];
        let keys: Vec<String> = ('A'..='Z').map(|c| c.to_string()).chain(extra).collect();
        let files = keys.iter().map(|key| path.to_owned() + key + ".mp3");
        let codes = keys.iter().map(|key| match key.as_str() {
            "Space" => Code::Space,
            "Enter" => Code::Enter,
            "Backspace" => Code::Backspace,
            other => Code::from_str(&("Key".to_owned() + other))
                .unwrap_or_else(|_| panic!("key {} not found!", other)),
        });
        let sounds = codes.zip(files).collect();

        Self { sounds }
    }
}

impl AudioLibrary {
    pub(crate) fn play(&self, key: Code) {
        if self.sounds.contains_key(&key) {
            let _ = HtmlAudioElement::new_with_src(self.sounds.get(&key).unwrap())
                .expect("Audio file not found!")
                .play();
        }
    }
}

pub(crate) struct AppDictionaries<'a> {
    pub(crate) left: WordDictionary<'a>,
    pub(crate) right: WordDictionary<'a>,
}
impl Default for AppDictionaries<'_> {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            left: WordDictionary {
                /// Temporary workaround because wasm doesn't have direct access to fs yet. Too bad! 
                /// https://github.com/rust-lang/rust/issues/41619
                buffer: vec!["abracadabra", "accrete", "adverse", "affect", "age", "as", "ass", "assert", "asserted", "ave", "Avesta", "axes", "bad", "badge", "badger", "barge", "bastard", "baste", "bear", "brave", "brew", "card", "cards", "cart", "cascade", "cast", "caste", "caves", "crater", "crave", "created", "crest", "crested", "crew", "dab", "dart", "database", "date", "dear", "deface", "defaced", "dew", "draft", "drag", "draw", "drax", "dread", "drear", "dressed", "drew", "east", "eaves", "Edgar", "effect", "exec", "fade", "far", "farce", "fart", "fast", "faster", "fear", "feared", "fears", "feet", "fest", "free", "freeze", "frred", "gaff", "gas", "gated", "geez", "grade", "grass", "grave", "grease", "great", "greatest", "Greece", "greed", "qat", "Qatar", "race", "read", "reader", "rear", "red", "redfaced", "rest", "retard", "retarded", "retested", "retract", "retreave", "reverberate", "sad", "sarge", "sass", "sat", "sax", "seat", "see", "sex", "stabbed", "stargate", "start", "starter", "starve", "staves", "stewardesses", "Stewart", "strafe", "straw", "strawberry", "stress", "swear", "sweater", "Swedes", "tar", "taser", "tea", "tear", "tears", "terse", "trade", "treat", "tree", "tweed", "tweet", "vast", "veer", "verde", "vest", "vested", "vexes", "wage", "war", "wart", "", "te", "ted", "tewater", "trel", "water", "waver", "waves", "wax", "we", "weedeater", "were", "west", "wrest", "zed", "zest", "abas", "abba", "abbe", "abed", "abet", "aced", "aces", "acre", "acta", "acts", "adds", "adze", "afar", "agar", "agas", "aged", "agee", "ager", "ages", "arbs", "arcs", "area", "ares", "arfs", "arse", "arts", "asea", "ates", "aver", "aves", "awed", "awee", "awes", "axed", "axes", "baas", "baba", "babe", "bade", "bads", "baff", "bags", "barb", "bard", "bare", "barf", "bars", "base", "bass", "bast", "bate", "bats", "batt", "bawd", "bead", "bear", "beat", "beds", "beef", "beer", "bees", "beet", "begs", "berg", "best", "beta", "bets", "brad", "brae", "brag", "bras", "brat", "braw", "bred", "bree", "brew", "brrr", "cabs", "caca", "cade", "cads", "cafe", "caff", "cage", "carb", "card", "care", "carr", "cars", "cart", "casa", "case", "cast", "cate", "cats", "cave", "caws", "ceca", "cede", "cees", "cere", "cess", "cete", "crab", "crag", "craw", "crew", "czar", "dabs", "dace", "dada", "dads", "daff", "daft", "dags", "darb", "dare", "dart", "data", "date", "daws", "dawt", "daze", "dead", "deaf", "dear", "debs", "debt", "deed", "deer", "dees", "deet", "deft", "dere", "deva", "devs", "dews", "drab", "drag", "drat", "draw", "dree", "dreg", "drew", "ears", "ease", "east", "eats", "eave", "ebbs", "edge", "effs", "efts", "egad", "eger", "eggs", "eras", "ergs", "errs", "erst", "eses", "etas", "ever", "eves", "ewer", "ewes", "exec", "exes", "face", "fact", "fade", "fads", "fags", "fard", "fare", "fart", "fast", "fate", "fats", "fava", "fave", "faze", "fear", "feat", "feds", "feed", "fees", "feet", "fere", "fess", "feta", "fete", "fets", "frae", "frag", "frat", "free", "fret", "gabs", "gads", "gaed", "gaes", "gaff", "gaga", "gage", "gags", "garb", "gars", "gast", "gate", "gats", "gave", "gaze", "gear", "geds", "geed", "gees", "geez", "gest", "geta", "gets", "grab", "grad", "grat", "gree", "grew", "qats", "race", "rads", "raff", "raft", "raga", "rage", "rags", "rare", "rase", "rate", "rats", "rave", "raws", "raze", "razz", "read", "rear", "rebs", "recs", "redd", "rede", "reds", "reed", "reef", "rees", "refs", "reft", "regs", "rest", "rete", "rets", "revs", "sabe", "sabs", "sacs", "sade", "safe", "saga", "sage", "sags", "sard", "sass", "sate", "save", "saws", "scab", "scad", "scag", "scar", "scat", "sear", "seas", "seat", "secs", "sect", "seed", "seer", "sees", "segs", "sera", "sere", "serf", "sers", "seta", "sets", "sett", "sews", "sext", "stab", "stag", "star", "stat", "staw", "stet", "stew", "swab", "swag", "swat", "tabs", "tace", "tact", "tads", "tags", "tare", "tars", "tart", "tass", "tate", "tats", "tavs", "taws", "taxa", "tear", "teas", "teat", "teds", "teed", "tees", "teff", "tegs", "test", "tets", "tews", "text", "trad", "tree", "tref", "tret", "tsar", "twae", "t", "twat", "twee", "tzar", "vacs", "vara", "vars", "vasa", "vase", "vast", "vats", "vavs", "vaws", "veer", "vees", "vera", "verb", "vert", "vest", "vets", "vext", "wabs", "wade", "wads", "waes", "waff", "waft", "wage", "wags", "ward", "ware", "wars", "wart", "t", "wats", "watt", "wave", "waws", "wear", "webs", "weds", "weed", "weer", "wees", "weet", "weft", "were", "wert", "west", "wets", "zags", "zarf", "zeds", "zees", "zest", "zeta", "aas", "aba", "abs", "ace", "act", "add", "ads", "adz", "aff", "aft", "aga", "age", "arb", "arc", "are", "arf", "ars", "art", "ass", "ate", "att", "ava", "ave", "awa", "awe", "axe", "baa", "bad", "bag", "bar", "bas", "bat", "bed", "bee", "beg", "bet", "bra", "brr", "cab", "cad", "car", "cat", "caw", "cee", "dab", "dad", "dag", "daw", "deb", "dee", "dev", "dew", "dex", "ear", "eat", "ebb", "eff", "efs", "eft", "egg", "era", "ere", "erg", "err", "ers", "ess", "eta", "eve", "ewe", "fad", "fag", "far", "fas", "fat", "fax", "fed", "fee", "fer", "fet", "few", "fez", "gab", "gad", "gae", "gag", "gar", "gas", "gat", "ged", "gee", "get", "qat", "rad", "rag", "ras", "rat", "raw", "rax", "reb", "rec", "red", "ree", "ref", "reg", "res", "ret", "rev", "rex", "rts", "sab", "sac", "sad", "sae", "sag", "sat", "saw", "sax", "sea", "sec", "see", "seg", "ser", "set", "sew", "sex", "tab", "tad", "tae", "tag", "tar", "tas", "tat", "tav", "taw", "tax", "tea", "ted", "tee", "teg", "tet", "tew", "twa", "vac", "var", "vas", "vat", "vav", "vaw", "vee", "veg", "vet", "vex", "wab", "wad", "wae", "wag", "war", "", "wat", "waw", "wax", "web", "wed", "wee", "wet", "zag", "zax", "zed", "zee"],
                keys: LEFT_QWERTY_KEYS.to_owned(),
            },
            right: WordDictionary {
                buffer: vec!["hi", "hili", "hill", "hillo", "hilly", "hilum", "him", "hin", "hinny", "hip", "hippo", "hippy", "hm", "hmm", "ho", "hokily", "hokku", "hokum", "hokypoky", "holily", "holk", "hollo", "holloo", "holly", "holm", "holmium", "holp", "holy", "homily", "hominy", "homo", "homonym", "homonymy", "homophony", "homy", "hon", "honk", "honky", "hook", "hookup", "hooky", "hooly", "hoop", "hoopoo", "hop", "hoppy", "hoy", "huh", "huipil", "hulk", "hulky", "hull", "hullo", "hum", "hump", "humph", "humpy", "hun", "hunh", "hunk", "hunky", "hup", "hymn", "hyp", "hypo", "hypolimnion", "hypopyon", "ikon", "ilium", "ilk", "ill", "illinium", "illy", "imino", "immy", "imp", "impi", "imply", "in", "inion", "ink", "inky", "inly", "inn", "inulin", "ion", "ionium", "jill", "jillion", "jiminy", "jimminy", "jimmy", "jimp", "jimply", "jimpy", "jin", "jink", "jinn", "jinni", "jo", "john", "johnny", "join", "jokily", "joky", "jollily", "jolly", "jouk", "joy", "joypop", "juju", "jump", "jumpily", "jumpy", "jun", "junk", "junky", "jupon", "khi", "khoum", "kilim", "kill", "killjoy", "kiln", "kilo", "kimono", "kin", "kinin", "kink", "kinkily", "kinky", "kino", "kip", "knoll", "knolly", "knop", "kohl", "koi", "kolo", "konk", "kook", "kooky", "kop", "koph", "li", "lily", "limn", "limo", "limp", "limpkin", "limply", "limuli", "limy", "lin", "linin", "link", "linkup", "linky", "linn", "lino", "linum", "liny", "lion", "lip", "lipin", "lippy", "lo", "loin", "loll", "lollipop", "lollop", "lolly", "lollypop", "loo", "look", "lookup", "loom", "loon", "loony", "loop", "loopy", "lop", "loppy", "loup", "lull", "lulu", "lum", "lump", "lumpily", "lumpy", "lunk", "luny", "lupin", "lupulin", "lymph", "mho", "mi", "mil", "milium", "milk", "milkily", "milky", "mill", "millimho", "milliohm", "million", "milo", "mim", "mini", "minikin", "minim", "minimill", "minimum", "minion", "minium", "mink", "minny", "mm", "mo", "moil", "mojo", "mol", "moll", "molly", "moly", "mom", "momi", "mommy", "mon", "monk", "mono", "monohull", "monophony", "monophyly", "monopoly", "mony", "moo", "mool", "moon", "moonily", "moony", "mop", "mopy", "moujik", "moulin", "mu", "muhly", "mujik", "mukluk", "mull", "mullion", "mum", "mumm", "mummy", "mump", "mumu", "mun", "muni", "munnion", "muon", "muonium", "muumuu", "my", "myopy", "nihil", "nil", "nill", "nim", "ninny", "ninon", "nip", "nippily", "nippy", "no", "noh", "noil", "noily", "nolo", "nom", "nomoi", "nonillion", "nonoily", "nonunion", "nonyl", "noo", "nook", "nooky", "noon", "noun", "nu", "null", "nun", "nylon", "nymph", "nympho", "oh", "ohm", "oho", "oil", "oilily", "oily", "oink", "olio", "om", "on", "onion", "oniony", "onium", "only", "ooh", "oomph", "op", "opinion", "opium", "ouph", "oy", "phi", "phon", "phonily", "phono", "phonon", "phony", "phyllo", "phylon", "phylum", "pi", "piki", "pili", "pill", "pillion", "pily", "pimp", "pimply", "pin", "pinion", "pink", "pinkly", "pinko", "pinky", "pinny", "pinon", "pinup", "piny", "pinyin", "pinyon", "pion", "pip", "pipkin", "pippin", "pipy", "piu", "plink", "plonk", "plop", "ploy", "plum", "plummy", "plump", "plumply", "plumy", "plunk", "ply", "poh", "poi", "poilu", "pokily", "poky", "pol", "polio", "poll", "pollinium", "polo", "polonium", "poly", "polynyi", "polyp", "polyphony", "polypi", "pom", "pommy", "pomp", "pompom", "pompon", "pony", "pooh", "pool", "poon", "poop", "pop", "poplin", "poppy", "poyou", "pul", "puli", "pulik", "pull", "pullup", "pulp", "pulpily", "pulpy", "pump", "pumpkin", "pun", "punily", "punk", "punkin", "punky", "punny", "puny", "pup", "pupil", "puppy", "pyin", "pylon", "uh", "ulu", "um", "umm", "ump", "un", "unhip", "unholily", "unholy", "unhook", "union", "unkink", "unlink", "unpin", "up", "uphill", "uplink", "upo", "upon", "yill", "yin", "yip", "yo", "yok", "yolk", "yolky", "yom", "yomim", "yon", "yoni", "you", "youpon", "yuk", "yum", "yummy", "yup", "yupon", ],
                keys: RIGHT_QWERTY_KEYS.to_owned(),
            },
        }
    }
}

/// Stores data for typing panel
#[derive(Default, Clone)]
pub(crate) struct WordData {
    input: String,
    last_word: String,
    buffer: Vec<String>,
}

impl WordData {
    /// Copies `amount` of elements from provided `dictionary` and constructs [WordBuffer] from them
    pub(crate) fn new(amount: usize, dictionary: &WordDictionary) -> Self {
        let mut rng = rand::thread_rng();

        let buffer = dictionary
            .buffer
            .choose_multiple(&mut rng, amount)
            .map(|str| str.to_string())
            .collect::<Vec<String>>();

        WordData {
            buffer,
            ..Default::default()
        }
    }

    pub(crate) fn submit(&mut self) {
        self.last_word = self.input.clone();
        if !self.buffer.is_empty() {
            self.buffer.remove(0);
        }
        self.input.clear();
    }

    pub(crate) fn last_word(&self) -> &str {
        self.last_word.as_ref()
    }

    pub(crate) fn next_word(&self) -> Option<&str> {
        match self.buffer.get(0) {
            Some(word) => Some(word.as_str()),
            None => None,
        }
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        self.input.push_str(string)
    }

    pub(crate) fn push(&mut self, ch: char) {
        self.input.push(ch)
    }

    pub(crate) fn input(&self) -> &str {
        self.input.as_ref()
    }

    pub(crate) fn pop(&mut self) -> Option<char> {
        self.input.pop()
    }

    pub(crate) fn buffer(&self) -> &Vec<String> {
        self.buffer.as_ref()
    }

    pub(crate) fn drain(&mut self) {
        self.buffer.drain(..);
    }
}
