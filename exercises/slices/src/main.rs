// Optional practice — not a chapter-completion marker.
// Skill: borrowing an image row out of a buffer with slices (Ch 4).
// Task: implement `row(frame: &[u8], width: usize, y: usize) -> &[u8]` that
// returns the y-th row of a row-major grayscale frame as a slice; use slice
// syntax `&frame[start..end]`. In `main`, build a 3x4 frame as the fixed array
// `[u8; 12]`, then call `row` and print each row.
//
// Prediction test: make `frame` mutable, keep `let r = row(&frame, 4, 0);`
// alive, then write `frame[0] = 99;` and predict the compiler error before running.

fn main() {
    let mut frame: [u8; 12] = [0; 12];
    const WIDTH: usize = 4;

    frame[5] = 7;
    frame[9] = 9;
    for i in 0..3 {
        let r = row(&frame, WIDTH, i);
        println!("{r:?}");
    }
}

fn row(frame: &[u8], width: usize, y: usize) -> &[u8] {
    let start = y * width;
    let end = start + width;
    &frame[start..end]
}
