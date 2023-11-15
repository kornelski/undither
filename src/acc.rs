use crate::palalpha::PixAlphaAble;
use rgb::{ComponentMap, RGB, RGB8};
use vpsearch::{BestCandidate, MetricSpace, Tree};

fn diff(p1: RGB8, p2: RGB8) -> u32 {
    ((i32::from(p1.r) - i32::from(p2.r)) * (i32::from(p1.r) - i32::from(p2.r)) +
            (i32::from(p1.g) - i32::from(p2.g)) * (i32::from(p1.g) - i32::from(p2.g)) +
            (i32::from(p1.b) - i32::from(p2.b)) * (i32::from(p1.b) - i32::from(p2.b))) as u32
}

struct OrphanRulesSuck;

impl MetricSpace<OrphanRulesSuck> for RGB8 {
    type UserData = ();
    type Distance = u32;
    fn distance(&self, other: &RGB8, (): &()) -> Self::Distance {
        diff(*self, *other)
    }
}
struct ExceptTheseTwo {
    index1: usize,
    index2: usize,
    distance: u32,
}

impl BestCandidate<RGB8, OrphanRulesSuck> for ExceptTheseTwo {
    type Output = u32;
    fn consider(&mut self, _: &RGB8, dist: u32, idx: usize, (): &()) {
        if dist < self.distance {
            if idx == self.index1 || idx == self.index2 {
                return;
            }
            self.distance = dist;
        }
    }
    fn distance(&self) -> u32 {
        self.distance
    }
    fn result(self, (): &()) -> u32 {
        self.distance
    }
}

pub struct Similarity {
    pal: Vec<RGB8>,
    cache: [i8; 256 * 256],
    vp: Tree<RGB8, OrphanRulesSuck>,
}

impl Similarity {
    pub fn new(pal: Vec<RGB8>) -> Self {
        let uhpal = unsafe {
            use std::slice;
            slice::from_raw_parts(pal.as_ptr().cast(), pal.len())
        };

        let mut s = Similarity {
            vp: Tree::new(uhpal),
            pal,
            cache: [-1; 256 * 256],
        };
        for i in 0..255 {
            s.cache[i << 8 | i] = 7;
        }
        s
    }

    #[inline]
    pub fn pal(&self, index1: usize) -> RGB8 {
        self.pal[index1]
    }

    pub fn compare(&mut self, index1: usize, index2: usize) -> (u32, RGB8) {
        let pos = if index1 < index2 {index1<<8|index2} else {index2<<8|index1};
        let cached = self.cache[pos];
        let p2 = self.pal[index2];
        if cached >= 0 {
            return (cached as u32, p2);
        }

        let p1 = self.pal[index1];

        let avg = RGB8 {
            r: ((u16::from(p1.r) + u16::from(p2.r)) / 2) as u8,
            g: ((u16::from(p1.g) + u16::from(p2.g)) / 2) as u8,
            b: ((u16::from(p1.b) + u16::from(p2.b)) / 2) as u8,
        };

        let distance = diff(avg, p1);

        // This is not exactly accurate, because vptree is laser-focused on the goal,
        // and won't look around enough even if the indexes are ignored,
        // but it seems to work out in practice anyway.
        let min_diff = self.vp.find_nearest_custom(&avg, &(), ExceptTheseTwo{
            distance: 1<<31,
            index1,
            index2,
        });

        let res = if min_diff >= distance*2 {8}
             else if min_diff >= distance {6}
             else if min_diff*3 >= distance*2 {1}
             else {0};

        self.cache[pos] = res;
        (res as u32, p2)
    }
}

pub struct Acc<'sim> {
    similarity: &'sim mut Similarity,
    acc: RGB<u32>,
    center_index: usize,
    weight: u32,
    transparent: Option<u8>,
}

impl<'pal, 'sim> Acc<'sim> {
    #[inline]
    pub fn new(center_index: usize, weight: u32, transparent: Option<u8>, similarity: &'sim mut Similarity) -> Self {
        let px = similarity.pal(center_index);
        Acc {
            transparent,
            similarity,
            center_index,
            weight,
            acc: px.map(|c| u32::from(c) * weight),
        }
    }

    #[inline]
    pub fn add<Pixel: PixAlphaAble>(&mut self, px: Pixel) {
        if px.is_transparent(self.transparent) {
            return;
        }
        let new_index = px.pal_index();
        let (weight, new_px) = self.similarity.compare(self.center_index, new_index);
        if weight > 0 {
            self.acc.r += u32::from(new_px.r) * weight;
            self.acc.g += u32::from(new_px.g) * weight;
            self.acc.b += u32::from(new_px.b) * weight;
            self.weight += weight;
        }
    }

    pub fn result(&self) -> RGB8 {
        self.acc.map(|c| (c / self.weight) as u8)
    }
}
