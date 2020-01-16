/*
#[derive(Debug)]
struct Weeks
{
day1 : String,
day2 : String,
}

#[derive(Debug)]
enum Days
{
Sunday(String),
Monday(u8,u8),
Tuesday,

}

#[derive(Debug)]
enum Option
{
Some(u8),
None
}
*/
//generic
enum Option<T>
{
Some(T),
None
}

fn main() 
{
/*
let value1 = Some(5);
let name = Some(String::from("Rizwan"));
let data1 = Option::Some(10);
println!("{:?}",data1);
println!("{:?}",value1);
println!("{:?}",name);
*/
let nodata:Option<i32> = None;
let coa = String::from("Qamar Javeed Bajwa");
println!("Name length of {:?} is {}",coa,coa.len());

let age =[22,33,44,55];
let temp =100;
let data = age.get(temp);
println!("{:?}",data);

/*
let week1 = Weeks{
let day1 : String::from("Friday"),
let day2 : String::from("Saturday"),
};

let day1 = Days::Sunday(String::from("Holiday"));
let day1 = Days::Monday(22,33);

println!("{:#?}",week1);
println!("{:#?}",day1);
println!("{:#?}",day2);
*/
}


