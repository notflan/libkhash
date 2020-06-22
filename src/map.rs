pub const KANA: &[char; 92] = &[
    'あ', 'い', 'う', 'え', 'お',
    'か', 'き', 'く', 'け', 'こ',
    'さ', 'し', 'す', 'せ', 'そ',
    'た', 'ち', 'つ', 'て', 'と',
    'な', 'に', 'ぬ', 'ね', 'の',
    'は', 'ひ', 'ふ', 'へ', 'ほ',
    'ま', 'み', 'む', 'め', 'も',
    'ら', 'り', 'る', 'れ', 'ろ',
    'や', 'ゆ', 'よ', 'わ', 'ん',
    'を', //45
    'ア', 'イ', 'ウ', 'エ', 'オ',
    'カ', 'キ', 'ク', 'ケ', 'コ',
    'サ', 'シ', 'ス', 'セ', 'ソ', 
    'タ', 'チ', 'ツ', 'テ', 'ト', 
    'ナ', 'ニ', 'ヌ', 'ネ', 'ノ', 
    'ハ', 'ヒ', 'フ', 'ヘ', 'ホ', 
    'マ', 'ミ', 'ム', 'メ', 'モ', 
    'ラ', 'リ', 'ル', 'レ', 'ロ', 
    'ヤ', 'ユ', 'ヨ', 
    'ワ', 'ン', 'ヲ', 
];
pub const KANA_SUB: &[char; 18] = &[
    'ゃ',
    'ゅ',
    'ょ',
    'ャ',
    'ュ',
    'ョ',
    'っ',
    'ッ',
    'ぁ','ぃ','ぅ','ぇ','ぉ',
    'ァ','ィ','ゥ','ェ','ォ',
];

use crate::def::Definition;

pub const KANA_SUB_VALID_FOR: &[Definition; 18] = &[ // Should we properly restrict these to only ones that make sense? (i.e. KI SHI HI etc..)
    Definition::single(5..=39),
    Definition::single(5..=39),
    Definition::single(5..=39),
    Definition::single(51..=85),
    Definition::single(51..=85),
    Definition::single(51..=85),
    Definition::any(),
    Definition::any(),
    Definition::single(5..=39),
    Definition::single(5..=39),
    Definition::single(5..=39),
    Definition::single(5..=39),
    Definition::single(5..=39),
    Definition::single(51..=85),
    Definition::single(51..=85),
    Definition::single(51..=85),
    Definition::single(51..=85),
    Definition::single(51..=85),
];

/// Find all subs that are okay for this kana. If `kana` is not in `KANA`, return None.
pub fn find_sub(kana: char) -> Option<Vec<char>>
{
    for (i,x) in (0..(KANA.len())).zip(KANA.iter()) {
	if *x == kana {
	    let mut output = Vec::with_capacity(KANA_SUB.len());
	    for (def,sub) in KANA_SUB_VALID_FOR.iter().zip(KANA_SUB.iter())
	    {
		if def.contains(i) {
		    output.push(sub.clone());
		}
	    }
	    return Some(output);
	}
    }
    None
}

/// Find subs by index.
pub fn sub(i: usize) -> Option<Vec<char>>
{
    if i < KANA.len() {
	let mut output = Vec::with_capacity(KANA_SUB.len());
	for (def,sub) in KANA_SUB_VALID_FOR.iter().zip(KANA_SUB.iter())
	{
	    if def.contains(i) {
		output.push(sub.clone());
	    }
	}
	Some(output)
    } else {
	None
    }
}
