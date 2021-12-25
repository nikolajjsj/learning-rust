fn main() {
    // Lifetimes prevents dangling references
    {
        let r;
        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
}
