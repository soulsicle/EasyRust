mod EasyRust;
[#tokio main]
async fn main() {
    let number_example:i128 = 123456789012345678; 
    let get_request_example = EasyRust::webget("https://www.google.com/");
    println!(body);
    println!("Break on int running...");
    EasyRust::breakonint(number_example);
    println!("writing to file test");
    let file_example = EasyRust::write("test.txt", "Hello World!");
}