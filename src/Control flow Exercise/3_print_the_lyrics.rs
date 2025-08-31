// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn main(){
    let num = 5;
    let lyrics: &str = "The Twelve Days of Christmas!";
    for _i in 0..num{
       println!("{}", lyrics);
    }
}



