fn main() {
    println!("1 : {:#010b}", 1);
    println!("2 : {:#010b}", 2);
    println!("4 : {:#010b}", 4);
    println!("8 : {:#010b}", 8);

    let right_half_ones: u8 = 8 + 4 + 2 + 1;
    let off_on_ones: u8 = 64 + 16 + 4 + 1;
    println!("{:#010b} | {:#010b} = {:#010b}", right_half_ones, off_on_ones, right_half_ones | off_on_ones);
    println!("{:#010b} & {:#010b} = {:#010b}", right_half_ones, off_on_ones, right_half_ones & off_on_ones);
    println!("{:#010b} ^ {:#010b} = {:#010b}", right_half_ones, off_on_ones, right_half_ones ^ off_on_ones);
    println!("{:#010b} << 1 = {:#010b}", off_on_ones, off_on_ones << 1);
    println!("!{:#010b} = {:#010b}", off_on_ones, !off_on_ones);

    let mask: u8 = 240u8;
    println!("{:#010b} | 1 << 2 = {:#010b}", mask, mask | 1 << 2);
    println!("{:#010b} & 1 << 6 = {:#010b}", mask, mask & 1 << 6);

    let echo_flag: u8 = 1u8 << 3;
    let first_query: u8 = 10;
    println!("\nfirst query = {:#010b}, is echo query? {}", first_query, echo_flag & first_query == echo_flag);

    let second_query: u8 = 130;
    println!("second query = {:#010b}, is echo query? {}", second_query, echo_flag & second_query == echo_flag);
}