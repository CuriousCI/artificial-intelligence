use std::{ops::Deref, rc::Rc};

use ai::problem::{Exploration, Goal, Heuristic, Problem, Transition};

#[derive(Clone, Debug)]
pub enum Alphabet {
    H,
    P,
}

pub type Sequence = Vec<Alphabet>;

pub type Energy = i64;

pub type Pos = (i64, i64);

pub struct Protein(Sequence);

#[derive(Hash, Eq, PartialEq)]
pub struct AminoAcid {
    pub pos: Pos,
    pub prev: Option<Rc<AminoAcid>>,
    pub depth: usize,
    pub first_turn: bool,
}

impl Protein {
    pub fn new(sequence: Sequence) -> Self {
        Self(sequence)
    }
}

impl Deref for Protein {
    type Target = Sequence;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Problem for Protein {
    type State = Rc<AminoAcid>;
}

impl Transition for Protein {
    type Action = Pos;

    fn new_state(&self, amino_acid: &Self::State, &pos: &Self::Action) -> Self::State {
        Rc::new(AminoAcid {
            pos,
            prev: Some(amino_acid.clone()),
            depth: amino_acid.depth + 1,
            first_turn: amino_acid.first_turn || pos.0 != 0,
        })
    }
}

impl Goal for Protein {
    fn is_goal(&self, amino_acid: &Self::State) -> bool {
        amino_acid.depth == self.len() - 1
    }
}

impl Heuristic<Energy> for Protein {
    fn heuristic(&self, amino_acid: &Self::State) -> Energy {
        // Questa info si può portare dietro nello stato
        // - costa poco memorizzarla
        // - evito O(n) calcoli

        // si può calcolare in modo incrementale?
        // la calcolo una volta all'inizio, e poi la espando,
        // tecnicamente mi devo mantenere gli estremi, sia in verticale sia in orizzontale
        // praticamente non serve più di tanto

        0

        // let mut min_x = amino_acid.pos.0;
        // let mut max_x = amino_acid.pos.0;
        // let mut min_y = amino_acid.pos.1;
        // let mut max_y = amino_acid.pos.1;
        //
        // let mut prev = amino_acid.prev.as_ref();
        // while let Some(p) = prev {
        //     if let Alphabet::H = self[p.depth] {
        //         min_x = min_x.min(p.pos.0);
        //         max_x = max_x.max(p.pos.0);
        //         min_y = min_y.min(p.pos.1);
        //         max_y = max_y.max(p.pos.1);
        //     }
        //
        //     prev = p.prev.as_ref();
        // }
        //
        // ((max_x - min_x) + (max_y - min_y)) / 3
    }
}

impl Exploration<Energy> for Protein {
    fn expand(&self, amino_acid: &Self::State) -> impl Iterator<Item = (Self::Action, Energy)> {
        let (x, y) = amino_acid.pos;

        let actions = if amino_acid.depth == 0 {
            vec![(x, y + 1)]
        } else if amino_acid.depth == 1 || !amino_acid.first_turn {
            vec![(x + 1, y), (x, y + 1)]
        } else {
            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        };

        actions
            .into_iter()
            .filter(|pos| {
                let mut prev = amino_acid.prev.as_ref();
                while let Some(p) = prev {
                    if &p.pos == pos {
                        return false;
                    }
                    prev = p.prev.as_ref();
                }
                true
            })
            .map(|pos| match self[amino_acid.depth + 1] {
                Alphabet::P => (pos, 0),
                Alphabet::H => {
                    let mut count = 0;
                    let mut prev = amino_acid
                        .prev
                        .as_ref()
                        .and_then(|amino_acid| amino_acid.prev.as_ref());

                    while let Some(p) = prev {
                        if let Alphabet::H = self[p.depth] {
                            if p.pos.0.abs_diff(pos.0) + p.pos.1.abs_diff(pos.1) == 1 {
                                count += 1;
                            }
                        }
                        prev = p.prev.as_ref();
                    }
                    (pos, 3 - count)
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}
