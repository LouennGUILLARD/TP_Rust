fn main() {
    let my_rectangle = Rectangle{
        length:2.0,
        width:3.0,
    };
    print!("{}\n",perimeter2(&my_rectangle));
    print!("{}\n",perimeter2(&my_rectangle));
    print!("{}\n",perimeter(my_rectangle.clone()));
    print!("{}\n",perimeter(my_rectangle.clone()));
    print!("{}\n",perimeter(my_rectangle));
    print!("{}\n",perimeter(my_rectangle));
    
}
#[derive(Clone)]
#[derive(Copy)]
struct Rectangle{
    length : f64,
    width : f64,
}

fn perimeter(rect:Rectangle)->f64{
    return (rect.length+rect.width)*2.0;
}

fn perimeter2(rect:&Rectangle)->f64{
    return (rect.length+rect.width)*2.0;
}

