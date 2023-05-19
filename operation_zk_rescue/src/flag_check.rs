use std::{thread, time,io};
use sha2::{Sha256,Digest};

pub fn flag_quest(){
    //Woe Jinx Validation
    thread::sleep(time::Duration::from_secs(2));
    println!("\n\nWoe Jinx: You have not been Jinxed");
    thread::sleep(time::Duration::from_secs(2));
    println!("Identity verified and message authenticated.");
    thread::sleep(time::Duration::from_secs(2));
    println!("Communication terminated.\n");

    //Security loophole window: HQ must act fast and make contact
    for _ in 0..10 {
        thread::sleep(time::Duration::from_secs(1));
        println!(" ");
    }
    thread::sleep(time::Duration::from_secs(5));
    println!("Beep.. Beep..Incoming transmission....");
    thread::sleep(time::Duration::from_secs(5));
    println!("HQ: Awaiting status zulu");
    let mut hasher = Sha256::new();
    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");
    hasher.update(message.trim().as_bytes());
    let res_hash = hasher.finalize();
    let comp= hex::encode(res_hash).to_string();
    let expected_msg = "25b565e4cd3a97540a07210ff3805e20da57fb925927b08af54f241360e75d19";
    if comp == expected_msg{
        thread::sleep(time::Duration::from_secs(5));
        println!("HQ: Good job!, Flag success!! we are on our way!, stay safe.");
        for _ in 0..10 {
            thread::sleep(time::Duration::from_secs(1));
            println!(" ");
        }
        println!("Red: Hello there! Seems like you have proven your mettle!\n");
        thread::sleep(time::Duration::from_secs(2));
        println!("Have you heard about the evolution of ZK?, Does it seem like a Philosophical question?\n");
        thread::sleep(time::Duration::from_secs(3)); 
        println!("Even for the most philosophical questions, the answer is often right in front of you.\n");
        thread::sleep(time::Duration::from_secs(4)); 
        println!("As you probably know by now, your HQ guys call me a concierge of crime.\n");
        thread::sleep(time::Duration::from_secs(4)); 
        println!("I prefer the term connoisseur, since I am a man of good taste, even in crime.\n");
        thread::sleep(time::Duration::from_secs(4));
        println!("31 years ago, I was at the Louvre, trying to find the real Mona Lisa smile \n");
        thread::sleep(time::Duration::from_secs(4)); 
        println!("I was treated with exquisite wine, and the most delicately sauteed french toast ever,\n");
        thread::sleep(time::Duration::from_secs(4)); 
        println!("not to mention the assortment of cheeses. Did I tell you that I stumbled when I saw her at first?\n"); 
        thread::sleep(time::Duration::from_secs(4)); 
        println!("Then it hit me, the answer was there staring at me right from the beginning of my journey.\n");
        thread::sleep(time::Duration::from_secs(4)); 
        println!("She wasn't even smiling, in fact she is not even who you think she is.\n");
        thread::sleep(time::Duration::from_secs(4)); 
        println!("In fact, I have visited her every year since then.\n"); 
        thread::sleep(time::Duration::from_secs(2));
        println!("Anyways, Good job with the message and everything. I knew it anyway. \n and what you get from all this is entirely up to you. \n");
    }
    else {
        println!("HQ: This must be a mistake, mission aborted, Flag Failure!");
    }
}

//Unfortunately you die if he discovers that you are working for HQ
pub fn woe_jinx_death() {
    println!("\nWoe Jinx: Well, Well , Well...\n");
    thread::sleep(time::Duration::from_secs(2));
    println!("If it moves like a rat \n");
    thread::sleep(time::Duration::from_secs(2));
    println!("and smells like a rat \n");
    thread::sleep(time::Duration::from_secs(2));
    println!("It must be a rat! \n");
    thread::sleep(time::Duration::from_secs(2));
    println!("End of the line for you!!");
}