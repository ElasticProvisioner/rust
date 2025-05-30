#![warn(clippy::filter_map_next)]

fn main() {
    let a = ["1", "lol", "3", "NaN", "5"];

    let element: Option<i32> = a.iter().filter_map(|s| s.parse().ok()).next();
    //~^ filter_map_next
    assert_eq!(element, Some(1));
}

#[clippy::msrv = "1.29"]
fn msrv_1_29() {
    let a = ["1", "lol", "3", "NaN", "5"];
    let _: Option<i32> = a.iter().filter_map(|s| s.parse().ok()).next();
}

#[clippy::msrv = "1.30"]
fn msrv_1_30() {
    let a = ["1", "lol", "3", "NaN", "5"];
    let _: Option<i32> = a.iter().filter_map(|s| s.parse().ok()).next();
    //~^ filter_map_next
}
