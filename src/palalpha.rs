#[derive(Copy, Clone)]
pub struct PalAlpha {
    pub idx: u8,
    pub a: bool, // true = opaque
}

impl From<u8> for PalAlpha {
    fn from(idx: u8) -> Self {
        PalAlpha{idx:idx, a:true}
    }
}

impl From<bool> for PalAlpha {
    fn from(a: bool) -> Self {
        PalAlpha{idx:0, a:a}
    }
}


pub trait PixAlphaAble: Copy {
    fn pal_index(&self) -> usize;
    fn is_transparent(&self, transparent_index: Option<u8>) -> bool;
}

impl PixAlphaAble for u8 {
    #[inline(always)]
    fn pal_index(&self) -> usize {
        *self as usize
    }

    #[inline(always)]
    fn is_transparent(&self, transparent_index: Option<u8>) -> bool {
        if let Some(index) = transparent_index {
            return *self == index;
        }
        return false;
    }
}

impl PixAlphaAble for PalAlpha {
    #[inline(always)]
    fn pal_index(&self) -> usize {
        self.idx as usize
    }

    #[inline(always)]
    fn is_transparent(&self, transparent_index: Option<u8>) -> bool {
        if let Some(index) = transparent_index {
            return self.idx == index;
        }
        return !self.a;
    }
}
