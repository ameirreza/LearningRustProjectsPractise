
impl rectangle
{

fn area(&self) -> u32{
    self.width * self.height
}
fn can_hold(&self,other: &rectangle) -> bool{
    self.width > other.width && self.height  > other.height
}
}

impl rectangle
{
    fn square(size:u32) -> rectangle
    {
        rectangle { width: (size), height: (size) }
    }
}



fn main() {

let rect = rectangle
{
    width : 30,
    height : 50
};
let rect2 = rectangle { width: (20), height: (40) };
let rect3 = rectangle { width: (10), height: (10) };
let rect4 = rectangle::square(25);
println!("the square is :{:?}",rect4);

println!("rect can hold rect 2 : {}", rect2.can_hold(&rect2));
println!("rect can hold rect 3 : {}", rect2.can_hold(&rect3));
println!("rect : {:#?} ,",rect);
println!("the are of the rectangle is {} square pixels;" , rect.area());



 let name = String::from("another one");


 let user2 = build_user(String::from("user2@mail.com"), String::from("ali"));
 let user3 = User
 {
    email:String::from("asfasfdafsa"),
    username:String::from("safasdf"),
    ..user2
 };

 let user1 = User
    {
        email :String::from("test@test.com"),
        username : String::from("test"),
        active : false,
        sign_in_count : 1,

       
    };

}


#[derive(Debug)]
struct rectangle
{
    width: u32,
    height : u32
}







     

 


 struct  Color (i32,i32,i32);
 struct  Point (i32,i32,i32);



struct User
{
    email : String,
    username : String,
    active : bool,
    sign_in_count : u64,


}
fn build_user (email:String,username:String) ->User
{
    User{
        email,
        username,
        active : true,
        sign_in_count : 1,
    }
}
     