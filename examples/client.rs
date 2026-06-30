use http_stat::{request, HttpStat};

#[tokio::main]
async fn main() {
    let mut stat = HttpStat::default();
    request("https://www.baidu.com/".try_into().unwrap(), &mut stat).await;
    // println!("{:?}", stat);
    println!("{stat}");
}
