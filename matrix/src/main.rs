use ndarray::{arr2, Array1};

fn main() {
    // A and B
    let a = arr2(&[[4, 0, 0], [0, 3, 0], [0, 0, 2]]);
    let b = arr2(&[[1, 0, 0], [0, 2, 0], [0, 0, 3]]);
    // C = A . B
    let c = arr2(&[[4, 0, 0], [0, 6, 0], [0, 0, 6]]);

    // Select a random r and generate x = (r^0, r^1, r^2)
    let n = 3u32;
    let r = 2u32;
    let x: Array1<_> = (0..n).map(|i| r.pow(i)).collect();

    // Check that Cx = A . B x
    let cx = c.dot(&x);
    let bx = b.dot(&x);
    let abx = a.dot(&bx);
    assert_eq!(cx, abx);
}
