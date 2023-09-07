use std::fs::read_dir;

fn main() {
    let paths = read_dir("./src/lib/images").unwrap();

    let mut i = 0, j = 0;
    for path in paths {
        println!(
            "{{\n\tpos: {} {},\n\tpath: \"{}\",\n}},",
            i,
            path.unwrap().path().display()
        );
        j += 1;
        if j == 7 {
            i += 1;
            j = 0;
        }
    }
}


