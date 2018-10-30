extern crate iostream;

use iostream::*;

#[allow(unused_must_use)]
fn main() {
    let (mut a, mut b) = (0, 0);

    while cin >> &mut a >> &mut b != Eof {
        cout << (a + b) << endl;
    }
}
