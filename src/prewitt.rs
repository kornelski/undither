use imgref::{Img, ImgRef, ImgVec};
use loop9::{loop9, Triple};
use rgb::{ComponentMap, RGB8, RGBA8};

pub trait ToGray {
    fn to_gray(self) -> i16;
}
impl ToGray for RGBA8 {
    fn to_gray(self) -> i16 {
        self.rgb().to_gray()
    }
}
impl ToGray for RGB8 {
    fn to_gray(self) -> i16 {
        let px = self.map(i16::from);
        px.r + px.g + px.g + px.b
    }
}

pub fn prewitt_squared_img<T: ToGray + Copy>(input: ImgRef<'_, T>) -> ImgVec<u16> {
    let gray: Vec<_> = input.pixels().map(|px| px.to_gray()).collect();
    let gray = Img::new(gray, input.width(), input.height());

    let mut prew = Vec::with_capacity(gray.width() * gray.height());
    loop9(gray.as_ref(), 0,0, gray.width(), gray.height(), |_x,_y,top,mid,bot|{
        prew.push(prewitt_squared3(top, mid, bot));
    });

    ImgVec::new(prew, gray.width(), gray.height())
}

#[inline]
pub fn prewitt_squared3<T: Into<i16>>(top: Triple<T>, mid: Triple<T>, bot: Triple<T>) -> u16 {
    prewitt_squared(top.prev, top.curr, top.next, mid.prev, mid.next, bot.prev, bot.curr, bot.next)
}

#[inline]
pub fn prewitt_squared<T: Into<i16>>(top_prev: T, top_curr: T, top_next: T, mid_prev: T, mid_next: T, bot_prev: T, bot_curr: T, bot_next: T) -> u16 {
    let top_prev = top_prev.into();
    let top_curr = top_curr.into();
    let top_next = top_next.into();
    let mid_prev = mid_prev.into();
    let mid_next = mid_next.into();
    let bot_prev = bot_prev.into();
    let bot_curr = bot_curr.into();
    let bot_next = bot_next.into();

    let gx = i32::from(
        top_next - top_prev +
        mid_next - mid_prev +
        bot_next - bot_prev);

    let gy = i32::from(
        bot_prev + bot_curr + bot_next -
        top_prev - top_curr - top_next);

    ((gx*gx + gy*gy) / 256) as u16
}
