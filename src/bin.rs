extern crate imgref;
extern crate lodepng;
extern crate rgb;
extern crate undither;
use imgref::*;
use lodepng::*;
use rgb::*;
use undither::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<_> = env::args().take(3).collect();

    if args.len() != 3 {
        eprintln!(r"Usage: {} input-8bit.png output-32bit.png
Version {}, © 2017 Kornel Lesiński <kornel@geekhood.net>
https://github.com/kornelski/undither", args[0], env!("CARGO_PKG_VERSION"));
        process::exit(1);
    }

    let mut state = State::new();
    state.decoder.color_convert = false;
    state.info_raw.colortype = ColorType::PALETTE;
    state.info_raw.set_bitdepth(8);
    let decoded = state.decode_file(&args[1]).unwrap();
    if state.info_raw.bitdepth() != 8 || state.info_raw.colortype != ColorType::PALETTE {
        eprintln!("Only 256-color images are supported");
        process::exit(1);
    }
    let image = match decoded {
        Image::RawData(image) => image,
        _ => unreachable!(),
    };

    let pal: Vec<_> = state.info_raw.palette().iter().map(|p| p.rgb()).collect();

    let undith = undither::Undither::new(None);
    let mut out = Img::new(vec![RGB::new(0,0,0); image.width * image.height], image.width, image.height);

    undith.undith_into(ImgRef::new(&image.buffer, image.width, image.height), None, Some(&Pal::new(pal.as_bytes())),
        0,
        0,
        image.width,
        image.height,
        &mut out
    );

    let (buf, w, h) = out.into_contiguous_buf();

    encode24_file(&args[2], &buf, w, h).unwrap();
}
