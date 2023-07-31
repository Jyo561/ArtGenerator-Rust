use image::{GenericImageView};

fn get_str_ascii(intent: u8)-> &'static str{
    let index = intent/32;
    let ascii = [" ",".","+","=","@","$","#","%"];
    return ascii[index as usize];
}

fn get_image(dir: &str,scale: u32){
    let img = image::open(dir).unwrap();
    println!("{:?}", img.dimensions());
    let (width,height) = img.dimensions();
    for y in 0..height{
        for x in 0..width{
            if y % (scale * 2) == 0 && x % scale == 0 {
                let plx = img.get_pixel(x,y);
                let mut intent = plx[0]/3 + plx[1]/3 + plx[2]/3;
                if plx[3] == 0{
                    intent = 0;
                }
                print!("{}", get_str_ascii(intent));
            }
        }
        if y%(scale*2)==0{
            println!("");
        }
    }
}
fn main() {
    get_image("418c74d3758a045bd29a3da57893ab6c.jpg",10);
    
}
