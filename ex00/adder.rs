//For this version we have to specify that parameters are mutable so the compiler is happy
// fn adder(mut a: u32, mut b: u32) ->u32
// {
//     while b!= 0
//     {
//         let carry = (a & b) << 1;
//         a = a ^ b;
//         b = carry;
//     }
//     a
// }

fn adder(a: u32, b: u32) ->u32
{
    let mut mutable_a : u32 = a;
    let mut mutable_b : u32 = b;
    while mutable_b!= 0
    {
        println!("carry is {} before << 1", mutable_a & mutable_b);
        let carry = (mutable_a & mutable_b) << 1;
        println!("carry is {} after << 1", carry);
        mutable_a = mutable_a ^ mutable_b;
        println!("mutable_a is {}", mutable_a);
        mutable_b = carry;
        println!("mutable_b is {}", mutable_b);
    }
    mutable_a
}
/*
Truth table:

A   |   B       A&B     A|B     A^B
0       0       0       0       0
0       1       0       1       1
1       0       0       1       1
1       1       1       1       0

            40:     0 0 1 0 1 0 0 0
            2:      0 0 0 0 0 0 1 0


 */

fn main()
{
    println!("{}",adder(146, 765787));

}
