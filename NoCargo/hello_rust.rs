fn main() {
    println!("Hello rust!\n");
    let coco_eng = "coco";
    let coco_hin = "कोको";
    println!("{}, Number of bytes: {}, Number of char count: {}", coco_eng, coco_eng.len(), coco_eng.chars().count());
    println!("{}, Number of bytes: {}, Number of char count: {}", coco_hin, coco_hin.len(), coco_hin.chars().count());
    println!("{:?}", b"coco");

    let number = 555;
    let number_ref = &number;
    println!("{:p}", number_ref);
    println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", number, number, number);
    
    let letter = "a";
    println!("{:~^5}", letter);
    println!("{:~<5}", letter);
    println!("{:~>5}", letter);
}
