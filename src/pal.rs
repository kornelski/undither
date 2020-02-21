use rgb::*;
use crate::acc::Similarity;
use std::ops::Index;
use std::cmp::PartialEq;
use std::cell::{RefMut, RefCell};

pub struct Pal {
    pal: Vec<RGB8>,
    similarity: RefCell<Similarity>,
}

impl Pal {
    pub fn new(palette_bytes: &[u8]) -> Self {
        let pal = to_rgb(palette_bytes);
        Pal {
            similarity: RefCell::new(Similarity::new(pal.clone())),
            pal: pal,
        }
    }

    pub fn similarity(&self) -> RefMut<'_, Similarity> {
        self.similarity.borrow_mut()
    }
}

impl Index<usize> for Pal {
    type Output = RGB8;
    fn index(&self, i: usize) -> &Self::Output {
        &self.pal[i]
    }
}

impl PartialEq for Pal {
    fn eq(&self, other: &Pal) -> bool {
        self.pal.eq(&other.pal)
    }
}

fn to_rgb(palette_bytes: &[u8]) -> Vec<RGB8> {
    palette_bytes.chunks(3).map(|byte| RGB8{r:byte[0], g:byte[1], b:byte[2]}).collect()
}

#[test]
fn paltest() {
    let a = vec![1,2,3];
    let b = vec![1,2,3];
    let c = vec![1,1,1];

    assert!(Pal::new(&a) == Pal::new(&a));
    assert!(Pal::new(&b) == Pal::new(&a));
    assert!(&Pal::new(&a) == &Pal::new(&b));

    assert!(Pal::new(&a) != Pal::new(&c));
}
