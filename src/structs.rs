pub fn run(){
     println!("MULTIPLE LIFETIME PARAMETER ");
    fn select <'a ,'b>(x:&'a str, _y:&'b str,first:bool) -> &'a str{
        if first{
            x 
        }
        else{
            panic!("mustchoose first");
        }
    }
    let s1 =String::from("hello");
    let s2=String::from("world");
    let result=select(&s1.as_str(),&s2.as_str(),true);
    println!("RESULT :  {}",result);
}