enum BoxColors {
    Brown,
    White,
}
impl BoxColors {
    fn print(&self) {
        match self {
            BoxColors::Brown => println!("Color: Brown"),
            BoxColors::White => println!("Color: White"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimensions {
    fn print(&self) {
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth)
    }
}

struct Package {
    dimension: Dimensions,
    weight: f64,
    color: BoxColors,
}

impl Package {
    fn new(dimension: Dimensions, weight: f64, color: BoxColors) -> Self {
        Self {
            dimension,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.dimension.print();
        println!("Weight: {:?}", self.weight);
        self.color.print()
    }
}

fn main() {
    let big = Dimensions {
        width: 5.2,
        height: 2.3,
        depth: 7.2,
    };
    let small = Dimensions {
        width: 2.3,
        height: 1.0,
        depth: 5.1,
    };

    let big_box = Package::new(big, 10.0, BoxColors::Brown);
    let small_box = Package::new(small, 5.0, BoxColors::White);

    big_box.print();
    small_box.print()
}
