use crate::acc::Acc;
use crate::pal::Pal;
use crate::palalpha::PixAlphaAble;
use crate::prewitt::prewitt_squared_img;
use imgref::{ImgRef, ImgVec};
use loop9::loop9;
use rgb::RGB8;

pub struct Undither {
    global_pal: Option<Pal>,
}

impl Undither {
    #[must_use]
    pub fn new(global_raw_pal: Option<&[u8]>) -> Self {
        Undither {
            global_pal: global_raw_pal.map(Pal::new),
        }
    }

    pub fn undith_into<Pixel>(&self, src_img: ImgRef<'_, Pixel>, transparent: Option<u8>, local_pal: Option<&Pal>,
        left: usize, top: usize, width: usize, height: usize, inout: &mut ImgVec<RGB8>)
        where Pixel: PixAlphaAble + Copy {

        let (left, top, width, height) = expand_by_1(left, top, width, height, inout.width(), inout.height());

        let pal = local_pal.or(self.global_pal.as_ref()).expect("there must be some palette");
        let sim = &mut *pal.similarity();

        let prewitt_image = {
            let out = inout.as_ref();
            let out = out.sub_image(left, top, width, height);
            prewitt_squared_img(out)
        };

        let prewitt_high_threshold = 256;
        let prewitt_low_threshold = 160;

        let mut out = inout.sub_image_mut(left, top, width, height);

        loop9(src_img, left, top, width, height, |x,y, prev, curr, next|{
                let center = curr.curr;
                if center.is_transparent(transparent) {
                    return;
                }

                let prewitt = prewitt_image[(x, y)];
                let center_weight = if prewitt > prewitt_low_threshold {
                    if prewitt > prewitt_high_threshold {
                        return;
                    }
                    24
                } else {
                    8
                } as u32;

                let mut acc = Acc::new(center.pal_index(), center_weight, transparent, sim);

                acc.add(prev.prev);
                acc.add(prev.curr);
                acc.add(prev.next);

                acc.add(curr.prev);
                acc.add(curr.next);

                acc.add(next.prev);
                acc.add(next.curr);
                acc.add(next.next);

                out[(x, y)] = acc.result();
            },
        );
    }
}

fn expand_by_1(mut left: usize, mut top: usize, mut width: usize, mut height: usize, max_width: usize, max_height: usize) -> (usize, usize, usize, usize) {
    if top > 0 { top -= 1; height += 1;}
    if left > 0 { left -= 1; width += 1;}
    if left+width+1 < max_width { width +=1; }
    if top+height+1 < max_height { height +=1; }
    (left, top, width, height)
}
