use std::cmp::Ordering;

#[derive(Debug)]
struct Image {
    id: String,
    width: u64,
    height: u64,
    aspect: f64,
}

impl Image {
    fn new(id: String, width: u64, height: u64) -> Self {
        Image { id, width, height, aspect: Image::aspect(width, height) }
    }
    fn aspect(width: u64, height: u64) -> f64 {
        width as f64 / height as f64
    }
}

//  If the struct does *not* have an `f64` field, you can omit
//  `impl Eq` and `impl Ord` and just `#[derive(Eq, Ord)]`
impl Eq for Image {}

impl Ord for Image {
    fn cmp(&self, other: &Self) -> Ordering {
        let area1 = self.width * self.height;
        let area2 = other.width * other.height;
        area1.cmp(&area2)
    }
}

impl PartialOrd for Image {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let area1 = self.width * self.height;
        let area2 = other.width * other.height;
        area1.partial_cmp(&area2)
    }
}

impl PartialEq for Image {
    // two images are the same if their ids are the same
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn main() {
    let a = Image::new("a".to_string(), 1920, 1080);
    let b = Image::new("b".to_string(), 1280, 720);
    let c = Image::new("c".to_string(), 640, 480);
    let d = Image::new("c".to_string(), 480, 640);

    assert_eq!(c == d, true); // `PartialEq`, c.id == d.id

    let mut list = vec![a, b, c, d];

    println!("Sorted, smallest first:");
    list.sort();
    for img in &list {
        println!("{:?}", img)
    }

    println!("\nSorted by width:");
    list.sort_unstable_by(|this, that| this.width.cmp(&that.width));
    for img in &list {
        println!("{:?}", img)
    }
}