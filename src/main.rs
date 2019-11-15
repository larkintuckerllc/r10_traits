fn main() {
    let st0 = St0 { // St0
        title: String::from("T"),
    };
    let st1 = St1 { // St1
        body: String::from("B"),
        title: String::from("T"),
    };
    print_a(&st0); // TrA for St0 - Title: T
    print_b(&st0); // TrB for St0 - Title: T
    print_a(&st1); // TrA for St0 - Title: T - Body: B
    print_b(&st1); // TrB for St0 - Title: T - Body: B
}

trait TrA {
    fn to_string(&self) -> String;
}

trait TrB {
    fn to_string(&self) -> String;
}

struct St0 {
    title: String,
}

struct St1 {
    title: String,
    body: String,
}

impl TrA for St0 {
    fn to_string(&self) -> String {
        format!("TrA for St0 - Title: {}", self.title)
    }
}

impl TrB for St0 {
    fn to_string(&self) -> String {
        format!("TrB for St0 - Title: {}", self.title)
    }
}

impl TrA for St1 {
    fn to_string(&self) -> String {
        format!("TrA for St0 - Title: {} - Body: {}", self.title, self.body)
    }
}

impl TrB for St1 {
    fn to_string(&self) -> String {
        format!("TrB for St0 - Title: {} - Body: {}", self.title, self.body)
    }
}

fn print_a(item: &impl TrA) {
    println!("{}", item.to_string());
}

fn print_b(item: &impl TrB) {
    println!("{}", item.to_string());
}
