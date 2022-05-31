// The goal of this file is to generate the weather conditions for use in the game.rs file.
// Temporarily we will be using

use crate::java_random::Random;
//use std::fmt;

pub enum WeatherOptions {
    Normal,
    Birds,
    SunPointOne,
    SunTwo,
    BlackHole,
    Unsun,
    Reverse,
}

pub struct Bird {

}

impl Bird {
    fn BirdMessage(n: i32, mut random: Random) -> String {
        match n {
        0 => "Seeing a lot of birds today.".to_string(),
        1 => "The birds continue to stare.".to_string(),
        2 => "[BIRD NOISES]".to_string(),
        3 => "Have you ever seen so many birds?".to_string(),
        4 => "Several birds pluck a fan from the stands.".to_string(),
        5 => "The Birds circle....but we don't have those snacks here.".to_string(),
        6 => "The birds are mad at you. You specifically".to_string(),
        7 => "These birds hate Blaseball!".to_string(),
        8 => "The birds are hungry.".to_string(),
        9 => "The birds stare into the sun.".to_string(),
        10 => "The birds give you the shivers.".to_string(),
        11 => "Birds.".to_string(),
        12 => format!("{} birds.", (random.next_f64() * 1000.0).trunc() as i32),
        13 => "Something isn't right about these birds.".to_string(),
        14 => "A bird pecks the outfield for features.".to_string(),
        15 => format!("A rogue umpire incinerated Bird {}!\nThey're replaced by Bird {}.", (random.next_f64() * 1000.0).trunc() as i32, (random.next_f64() * 1000.0).trunc() as i32),
        16 => "Is that a normal number of eyes for a bird?".to_string(),
        17 => "Another bird lands in the rafters.".to_string(),
        18 => "I hate these birds.".to_string(),
        19 => "The birds' cries almost sound like music.".to_string(),
        20 => "The birds form a blanket over the stadium, blocking out the sun.".to_string(),
        21 => "The birds squak of death.".to_string(),
        22 => "Too many birds.".to_string(),
        23 => "not implemented yet".to_string(),
                /*don't ruin the surprise for this one!
                Player player = randomActivePlayer();
                if(player.pregameRitual.equals("Eating a bird"))
                    println(player.name + " eats a bird!");
                else if(player.pregameRitual.equals("Eating multiple birds"))
                    println(player.name + " gobbles down an ungodly amounts of birds!");
                else if(player.pregameRitual.equals("Shaking their fist at a bird's nest"))
                    println(player.name + " REALLY hates birds!");
                else if(player.pregameRitual.equals("Birdwatching"))
                    println(player.name + " watches the birds.");
                else if(player.pregameRitual.equals("Feeding the birds"))
                    println("The birds feed off of " + player.name + ".");
                else
                    println(player.name + " is thinking about birds.");
                break;*/
        24 => "You spot a shiny bird! It looks back at you, annoyed.".to_string(),
        25 => "The cacophony of bird calls makes you sick.".to_string(),
        26 => "The birds stare at their favorite Keeper.".to_string(),
        27 => "Birds Birds Birds Birds Birds Birds Birds Birds.".to_string(),
        28 => "What if birds were real?".to_string(),
        _ => "ERROR: BIRDS".to_string()
        }
    }
}
