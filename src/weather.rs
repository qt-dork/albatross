// The goal of this file is to generate the weather conditions for use in the game.rs file.
// Temporarily we will be using

use jandom;
use std::fmt;

pub enum WeatherOptions {
    Normal,
    Birds,
    SunPointOne,
    SunTwo,
    BlackHole,
    Unsun,
    Reverse,
}

trait Bird {
    fn BirdMessage(&self, i32) -> String;
}

impl Bird {
    fn BirdMessage(n: i32) -> String {
        match n {
        0 => "Seeing a lot of birds today.",
        1 => "The birds continue to stare.",
        2 => "[BIRD NOISES]",
        3 => "Have you ever seen so many birds?",
        4 => "Several birds pluck a fan from the stands.",
        5 => "The Birds circle....but we don't have those snacks here.",
        6 => "The birds are mad at you. You specifically",
        7 => "These birds hate Blaseball!",
        8 => "The birds are hungry.",
        9 => "The birds stare into the sun.",
        10 => "The birds give you the shivers.",
        11 => "Birds.",
        12 => format!("{} birds.", Random::next_f64() * 1000),
        13 => "Something isn't right about these birds.",
        14 => "A bird pecks the outfield for features."
        15 => format!("A rogue umpire incinerated Bird {}!\nThey're replaced by Bird {}.", Random::next_f64() * 1000, Random::next_f64() * 1000),
        16 => "Is that a normal number of eyes for a bird?",
        17 => "Another bird lands in the rafters.",
        18 => "I hate these birds.",
        19 => "The birds' cries almost sound like music.",
        20 => "The birds form a blanket over the stadium, blocking out the sun.",
        21 => "The birds squak of death.",
        22 => "Too many birds.",
        23 => "not implemented yet",
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
        24 => "You spot a shiny bird! It looks back at you, annoyed.",
        25 => "The cacophony of bird calls makes you sick.",
        26 => "The birds stare at their favorite Keeper.",
        27 => "Birds Birds Birds Birds Birds Birds Birds Birds.",
        28 => "What if birds were real?",
        }
    }
}
