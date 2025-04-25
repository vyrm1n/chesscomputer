type PiecePosition = u64;

static COL_MAP: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

fn index_to_position(index: usize) -> String {
    let column = index % 8;
    let row = index / 8 + 1;
    return format!("{}{}", COL_MAP[column], row);
}


fn main() {
    for i in 0..16 {
        println!("{} -> {}", i, index_to_position(i));
    }
}
