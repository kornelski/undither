extern crate test;

#[bench]
fn blur_loop(b: &mut test::Bencher) {
    use prewitt::*;
    let screen = vec![30u8; 320*200];

    b.iter(||{
        let mut sum = 0;
        for y in 0..200 {
            let row1 = &screen[(y-0)*320..(y-0)*320+320];
            let row0 = if y > 0 {&screen[(y-1)*320..(y-1)*320+320]} else {row1};
            let row2 = if y+1 < 200 {&screen[(y+1)*320..(y+1)*320+320]} else {row1};
            for x in 0..320 {
                let tc = row0[x];
                let tp = if x > 0 {row0[x-1]} else {tc};
                let tn = if x+1 < 320 {row0[x+1]} else {tc};
                let mc = row1[x];
                let mp = if x > 0 {row1[x-1]} else {mc};
                let mn = if x+1 < 320 {row1[x+1]} else {mc};
                let bc = row2[x];
                let bp = if x > 0 {row2[x-1]} else {bc};
                let bn = if x+1 < 320 {row2[x+1]} else {bc};
                sum += prewitt_squared(tp,tc,tn,mp,mn,bp,bc,bn) as usize;
            }
        }
        sum
    })
}

#[bench]
fn blur_loop_mut(b: &mut test::Bencher) {
    use prewitt::*;
    let screen = vec![30u8; 320*200];

    b.iter(||{
        let mut sum = 0;
        let mut row0 = &screen[0..0+320];
        let mut row1 = row0;
        let mut row2 = &screen[320..320+320];
        for y in 0..200 {
            row0 = row1;
            row1 = row2;
            row2 = if y+1 < 200 {&screen[(y+1)*320..(y+1)*320+320]} else {row1};
            let mut tp;
            let mut tc = row0[0];
            let mut tn = if 1 < 320 {row0[1]} else {tc};
            let mut mp;
            let mut mc = row1[0];
            let mut mn = if 1 < 320 {row1[1]} else {mc};
            let mut bp;
            let mut bc = row2[0];
            let mut bn = if 1 < 320 {row2[1]} else {bc};
            for x in 0..320 {
                tp = tc;
                tc = tn;
                tn = if x+1 < 320 {row0[x+1]} else {tc};
                mp = mc;
                mc = mn;
                mn = if x+1 < 320 {row1[x+1]} else {mc};
                bp = bc;
                bc = bn;
                bn = if x+1 < 320 {row2[x+1]} else {bc};
                sum += prewitt_squared(tp,tc,tn,mp,mn,bp,bc,bn) as usize;
            }
        }
        sum
    })
}


#[bench]
fn blur_loop_cb(b: &mut test::Bencher) {
    use prewitt::*;
    use loop9::*;
    use imgref::*;
    let screen = vec![30u8; 320*200];

    b.iter(||{
        let mut sum = 0;
        loop9_img(Img::new(&screen, 320, 200), |_x,_y,top,mid,bot|{
            sum += prewitt_squared3(top,mid,bot) as usize;
        });
        sum
    })
}
