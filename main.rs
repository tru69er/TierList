use std::fs::read_dir;

fn main() {
    let paths = read_dir("./src/lib/images").unwrap();

    let mut i = 0;
    for path in paths {
        i += 1;
        println!(
            "{{\n\tid: {},\n\tpath: \"{}\",\n}},",
            i,
            path.unwrap().path().display()
        );
    }
}


