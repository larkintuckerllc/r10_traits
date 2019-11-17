fn main() {
    let st0 = St0 { 
        title: String::from("T"),
    };
    let st1 = St1 {
        body: String::from("B"),
        title: String::from("T"),
    };
    log_a(&st0); // log_a: TrA for St0 - Title: T
    log_a(&st1); // log_a: TrA for St0 - Title: T - Body: B
    log_b(&st0); // log_b: TrB for St0 - Title: T
    log_b(&st1); // log_b: TrB for St1 - Title: T - Body: B
    log_c(&st0); // log_c: TrC for St0 - Title: T
    log_c(&st1); // log_c: TrC - Default
    log_d(&st0); // log_d: TrD for St0 - Title: T
    log_d(&st1); // log_d: TrD - Default
    log_a_b(&st0); // log_a_b: TrA for St0 - Title: T log_a_b: TrB for St0 - Title: T
    let item = returns_tra();
    log_a(&item); // log_a: TrA for St0 - Title: T
}

struct St0 {
    title: String,
}

struct St1 {
    title: String,
    body: String,
}

trait TrA {
    fn to_string(&self) -> String;
}

impl TrA for St0 {
    fn to_string(&self) -> String {
        format!("TrA for St0 - Title: {}", self.title)
    }
}

impl TrA for St1 {
    fn to_string(&self) -> String {
        format!("TrA for St0 - Title: {} - Body: {}", self.title, self.body)
    }
}

trait TrB {
    fn to_another_string(&self) -> String;
}

impl TrB for St0 {
    fn to_another_string(&self) -> String {
        format!("TrB for St0 - Title: {}", self.title)
    }
}

impl TrB for St1 {
    fn to_another_string(&self) -> String {
        format!("TrB for St1 - Title: {} - Body: {}", self.title, self.body)
    }
}

trait TrC {
    fn to_third_string(&self) -> String {
        String::from("TrC - Default")
    }
}

impl TrC for St0 {
    fn to_third_string(&self) -> String {
        format!("TrC for St0 - Title: {}", self.title)
    }
}

impl TrC for St1 {
}

trait TrD {
    fn to_fourth_string(&self) -> String {
        String::from("TrD - Default")
    }
}

impl TrD for St0 {
    fn to_fourth_string(&self) -> String {
        format!("TrD for St0 - Title: {}", self.title)
    }
}

impl TrD for St1 {
}

fn log_a(item: &impl TrA) {
    let output = item.to_string();
    println!("log_a: {}", output);
}

fn log_b(item: &impl TrB) {
    let output = item.to_another_string();
    println!("log_b: {}", output);
}

fn log_c(item: &impl TrC) {
    let output = item.to_third_string();
    println!("log_c: {}", output);
}

fn log_d<T: TrD>(item: &T) {
    let output = item.to_fourth_string();
    println!("log_d: {}", output);
}

fn log_a_b<T: TrA + TrB>(item: &T) {
    let output = item.to_string();
    println!("log_a_b: {}", output);
    let output = item.to_another_string();
    println!("log_a_b: {}", output);
}

fn returns_tra() -> impl TrA {
    St0 { 
        title: String::from("T"),
    }
}
