db.classes.insertMany([
    {
        class_name: "Cultural Aficionado", 
        class_enum: "Aficianado",
        class_desc: "A collector of artifacts, art, and pleasures from all over the kingdom. They have a keen mind and with deep planning and knowledge of the kingdom is adept at sabotage.",
        class_perks: "Add 2 to your Cultural Might and -1 to you Military Might. When making a Espionage Might roll, add your Cultural Might Bonus to the roll instead of your Espionage Might Bonus.",
        class_abilities: [
            {
                "ability_name": "Cultural Preservation",
                "ability_desc": "Before an episode, roll a Cultural Might Roll, with a Neutral roll you cannot get a result that would be less than a Neutral roll for the rest of the event.",
                "ability_unlock": {
                    "might" : "Culture",
                    "roll_tier": "Neutral",
                    "unlock": 0
                }
            },
            {
                "ability_name": "Artistic Endeavors",
                "ability_desc": "The Cultural Aficionado can organize and promote artistic endeavors, increasing the chances of success in the event. Use a Cultural Might roll to determine the outcome. A roll of 12 or higher yields 2 tokens for the Cultural Aficionado. If the Cultural Aficionado fails, they lose two favor.",
                "ability_unlock": {
                    "might" : "Culture",
                    "roll_tier": "None",
                    "unlock": 0
                }
            },
            {
                "ability_name": "Cultural Diplomacy",
                "ability_desc": "The Cultural Aficionado can use propaganda to shape public opinion and increase the chances of success in the event. Use a Cultural Might roll to play the event to your favor. If you get a Good, or better roll, gains a strategem (the ability to roll two dice) on a later roll.",
                "ability_unlock": {
                    "might" : "Culture",
                    "roll_tier": "Good",
                    "unlock": 0
                }
            },
            {
                "ability_name": "Noble Heritage",
                "ability_desc": "The Cultural Aficionado can spend up to 8 Cultural tokens. With a Cultural Might under 5, add a number equal to half the Cultural Tokens spent rounded down to the roll. With a Cultural Might including 5 to 10 excluded, gain equal to the amount of Cultural Tokens spent. With a Cultural Might greater than 10, add double the Cultural Tokens spent to the roll.",
                "ability_unlock": {
                    "might" : "Culture",
                    "roll_tier": "None",
                    "unlock": 0
                }
            },
            {
                "ability_name": "Forgotten History",
                "ability_desc": "When a player applies sabotage on you, pay 4 Cultural Tokens, and negate the sabotage.",
                "ability_unlock": {
                    "might" : "Culture",
                    "roll_tier": "None",
                    "unlock": 4
                }
            },
            {
                "ability_name": "Back Alley Dealings",
                "ability_desc": "This can only be done during the beginning of an episode. Spend 10 of any combination of tokens. When you do, choose another player. The two of you share a copy of all your rewards with each other.",
                "ability_unlock": {
                    "might" : "Culture",
                    "roll_tier": "None",
                    "unlock": 5
                }
            },
            {
                "ability_name": "Sic Scelestus Insidiae",
                "ability_desc": "This can only be done during the beginning of an episode. Declare a Might secretly. A clock is set for 2 episodes. If a character fails a Might roll with the secretly declared roll with in the clock; gain 5 Cultural Tokens.",
                "ability_clock": {
                    "clock_duration": 2,
                    "clock_remaining": 2,
                    "clock_name": "Sic Scelestus Insidiae",
                    "clock_desc": "If a character fails a Might roll with the secretly declared roll with in the clock; gain 5 Cultural Tokens.",
                    "clock_conf": false, 
                },
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "None",
                    "unlock": 4
                }
            },
            {
                "ability_name": "Magnum Opus",
                "ability_desc": "This can only be used once per game. The player can declare that they are starting their Magnum Opus, which is a work of art that will bring glory to the kingdom. They set a clock of 4 episodes and must contribute 15 Cultural Tokens to it before the clock runs out. If they succeed, they gain a permanent +2 to their Cultural Might.",
                "ability_clock": {
                    "clock_duration": 4,
                    "clock_remaining": 4,
                    "clock_name": "Magnum Opus",
                    "clock_desc": "They set a clock of 4 episodes and must contribute 15 Cultural Tokens to it before the clock runs out. If they succeed, they gain a permanent +2 to their Cultural Might.",
                    "clock_conf": false, 
                },
                "ability_unlock": {
                    "might" : "Culture",
                    "roll_tier": "None",
                    "unlock": 6
                }
            },
        ],
    },
    {
        class_name: "War General", 
        class_enum: "WarGeneral",
        class_desc: "The grizzled war general has been through so many wars that the srategems and movements of ware are written on the back of his eyelids.",
        class_perks: "Add +2 to your Military Might and -1 to you Cultural Might. When making a Military Might roll, you can spend 1 Military Token to initiate combat. If you win the combat, gain +2 to your next roll, or +5 if your Military Might is above 5.",
        class_abilities: [
            {
                "ability_name": "Reinforcements",
                "ability_desc": "The military advisor can gather intelligence by rolling Military Might or Espionage Might and gain insight into the enemy's weaknesses, increasing the chances of success in the event. A roll to be considered successful if the result is 15 or higher. On success, you gain 1 king's favor. If you are helping a Player gain 2 King's Favor. Any other type of roll will cause a sabotage and the event is failed.",
                "ability_unlock": {
                    "might" : "Military",
                    "roll_tier": "Good",
                    "unlock": 0
                }
            },
            {
                "ability_name": "Spoils of War",
                "ability_desc": "Choose another player at the beginning of the episode. If that player successfully completes an event, gain 2 coins. If that player fails an event, they lose 2 coins.",
                "ability_unlock": {
                    "might" : "Military",
                    "roll_tier": "None",
                    "unlock": 0
                }
            },
            {
                "ability_name": "Inspiration",
                "ability_desc": "The military advisor can inspire and lead the kingdom's soldiers, Use a military roll to play the event to your favor. Your roll must be a Good roll or better. A success gains, a strategem (the ability to roll two dice) on a later roll.",
                "ability_unlock": {
                    "might" : "Military",
                    "roll_tier": "Good",
                    "unlock": 0
                }    
            },
            {
                "ability_name": "Sabotage!",
                "ability_desc": "The military advisor can sabotage the enemy's plans, weakening their ability to succeed in the event. Use a military roll to play the event to your favor. Spend 2 Military tokens and gain a sabotage (the ability to bring down the Tier of another player) on a later roll.",
                "ability_unlock": {
                    "might" : "Military",
                    "roll_tier": "Good",
                    "unlock": 0
                }
            },
            {
                "ability_name": "Mercenary Legion",
                "ability_desc": "The Mercenary Legion can infiltrate and sabotage the enemy's defenses. Use a Espionage Might roll to determine the success of the sabotage. Neutral: +1 bonus to a later Military Might roll in the same episode. Good: +2 bonus to a later Military Might roll in the same episode. Great: +3 bonus to a later Military Might roll in the same episode. Critical: +5 bonus to a later Military Might roll in the same episode.",
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "Neutral",
                    "unlock": 2
                }
            },
            {
                "ability_name": "Vine Vidi Vici",
                "ability_desc": "Can only be used in combat. Add +4 to your Combat roll. If you win the combat, gain +3 Military Tokens; however, if you loose, lose -3 Military Tokens.",
                "ability_unlock": {
                    "might" : "Military",
                    "roll_tier": "None",
                    "unlock": 4,
                }
            },
            {
                "ability_name": "Man of Action",
                "ability_desc": "Can only be used before a roll. Declare that this roll will be Neutral or better. Neutral: gain +1 Military Tokens. Good: gain +2, Great: gain +3. Critical: gain +5",
                "ability_unlock": {
                    "might" : "Military",
                    "roll_tier": "Fail", 
                    "unlock": 4
                }
            },
            {
                "ability_name": "Total War",
                "ability_desc": "Can only be used once per game. An episode roll of 90 is made. The kingdom goes to war. Based on the combat roll of each player during the episode the War General gains Military Tokens. Fail: +5, Bad: +3, Neutral: +1.",
                "ability_unlock": {
                    "might" : "Military",
                    "roll_tier": "Fail",
                    "unlock": 6
                }
            }
        ]
    },
    {
        class_name: "Royal Scientist", 
        class_enum: "Scientist",
        class_desc: "A Scientist tasked with aiding and enriching the kingdom with new great technologie. The Royal Scientist thrives off drawing cards.",
        class_perks: "Each turn, draw two Abilities instead of one. When you are at 10 cards and would normally throw away any cards drawn put them back in the deck.",
        class_abilities: [
            {
                "ability_name": "Reseach",
                "ability_desc": "Draw 2 Abilities from your deck.",
                "ability_unlock": {
                    "might" : "Science",
                    "roll_tier": "None",
                    "unlock": 0
                }
            },
            {
                "ability_name": "Eureka",
                "ability_desc": "Roll a Scientific Might roll, On a Fantastic Roll, Gain a new Ability from the pool.",
                "ability_unlock": {
                    "might" : "Science",
                    "roll_tier": "Fantastic",
                    "unlock": 3
                }
            },
            {
                "ability_name": "Scientific Inquery",
                "ability_desc": "Roll a Scientific Might roll, on a Good or better roll, Discard a card, from your Deck. Gain a new Ability from the pool.",
                "ability_unlock": {
                    "might" : "Science",
                    "roll_tier": "Good",
                    "unlock": 3
                }
            },
            {
                "ability_name": "Reseach Papers",
                "ability_desc": "Make a Diplomatic Might Roll. On a Great or better roll, trade anability with another character.",
                "ability_unlock": {
                    "might" : "Diplomacy",
                    "roll_tier": "Great",
                    "unlock": 5
                }
            },
            {
                "ability_name": "Forsight",
                "ability_desc": "Can only be played at the beginning of the season. No matter the results of the Season, you still get to pull an Ability during the Resolution Faze.",
                "ability_unlock": {
                    "might" : "Science",
                    "roll_tier": "None",
                    "unlock": 5
                }
            },
            {
                "ability_name": "The Scientific Method.",
                "ability_desc": "Can only be played at the beginning of the season. When a Character plays a confidential clock during the time that this clock is in effect, look at the effect of the clock. You cannot tell anyone else until this clock ends.",
                "ability_clock": {
                    "clock_duration": 6,
                    "clock_remaining": 6,
                    "clock_name": "The Scientific Method",
                    "clock_desc": "Set a clock. Announce that you get to know the effects of any confidential clock. During the lifetime of this clock, you get to see the effect of any confidential Clock in play. You cannot tell anyone other than the Clock's owner until this Clock ends",
                    "clock_conf": false, 
                },
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "None",
                    "unlock": 5
                }
            },
            {
                "ability_name": "Sic Parvus Magnum",
                "ability_desc": "Can only be played at the beginning of the season. Set a Clock, activates when you draw a card.",
                "ability_clock": {
                    "clock_duration": 4,
                    "clock_remaining": 4,
                    "clock_name": "Sic Parvus Magnum",
                    "clock_desc": "During the time that this clock is active, when you draw a card, you can you can immediatley discard it, and draw another.",
                    "clock_conf": true, 
                },
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "None",
                    "unlock": 5
                }
            },
            {
                "ability_name": "Scientific Revolution.",
                "ability_desc": "Can only be played at the beginning of the round. On a Great roll or higher, discard your entire hand. Draw 10 Abilities.",
                "ability_unlock": {
                    "might" : "Science",
                    "roll_tier": "Great",
                    "unlock": 7
                }
            }
        ]
    },
    {
        class_name: "Spy Master", 
        class_enum: "SpyMaster",
        class_desc: "The Royal Spy Master is a cunning adversary, but then again those who are on her side can find that their plans always seem to come through at the end of the day. But whose side is she on anyway?",
        class_perks: "Every turn, You can either: Choose a character other than your self. You can sabotage their roll. Or: Choose a character and give them a strategem.",
        class_abilities: [
            {
                "ability_name": "Insight",
                "ability_desc": "Can only be used at the beginning of a round, roll an Espionage Roll, if the result is Neutral or better, you can choose a character and you know what they will play during ability phase.",
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "Neutral",
                    "unlock": 0
                }
            },
            {
                "ability_name": "Open for buisness",
                "ability_desc": "At any point when a character has to make a roll, you can play this Ability. The Character in question has the option of paying you 3 of any token. If they do they get a strategem.",
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "None",
                    "unlock": 2
                }
            },
            {
                "ability_name": "Know your Enemy",
                "ability_desc": "You can only play this ability during a season witht the word 'Siege' in the name. Choose a target and Roll an Military roll. If the result is Great or better, the target will give you their Resources won during the Siege. If the result is Bad or worse they get yours.",
                "ability_unlock": {
                    "might" : "Military",
                    "roll_tier": "Great",
                    "unlock": 2
                }
            },
            {
                "ability_name": "'Would I Lie to You?'",
                "ability_desc": "At any point when a character has to make a roll, you can play this Ability, Choose a target and Roll an Espionage roll. If the result is Great or better, the target will need to get one lower tier or better on thier roll in order to activater it. If the result is Bad or worse they must make a one tier better roll or better.",
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "Great",
                    "unlock": 4
                }
            },
            {
                "ability_name": "Spy Network",
                "ability_desc": "You can only play this ability during the start of the Season. Roll an Espionage roll. If the roll is Good or better, You get a strategem. If your Espionage might is 6 or 7 you get two. If your Espionage Might is 8 or 9 you get three.",
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "Good",
                    "unlock": 4
                }
            },
            {
                "ability_name": "'All part of my master plan!'",
                "ability_desc": "This can only be played during the beginning of an episode. Spend the Sum of 10 tokens. Declare to the Table That this Ability's Clock is in Effect.",
                "ability_clock": {
                    "clock_duration": 5,
                    "clock_remaining": 5,
                    "clock_name": "Coup de ta",
                    "clock_desc": "During this Clock any roll other than the Spy Master takes minus one roll_tier to their rolls. Fantastic rolls are not Affected. Failure rolls contribute 3 tokens to your resources.",
                    "clock_conf": false, 
                },
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "None",
                    "unlock": 6
                }
            },
            {
                "ability_name": "Assasination",
                "ability_desc": "Can only be used at the beginning of the season. Roll a dice and make a note of the Roll Tier.",
                "ability_clock": {
                    "clock_duration": 4,
                    "clock_remaining": 4,
                    "clock_name": "Assasination",
                    "clock_desc": "The First player to make the Roll Tier as you, is marked for Assasination. Make a combat roll. If you win you can now Assasinate the target. They must create a new character.",
                    "clock_conf": true, 
                },
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "None",
                    "unlock": 6
                }
            },
            {
                "ability_name": "Grand Theft.",
                "ability_desc": "Can only be used during the resolution of the Season. Target the Character that has won the Season. They must make Either a Military Might or an Espionage Might roll. If they do not get a Fantastic roll, take their rewards.",
                "ability_unlock": {
                    "might" : "Espionage",
                    "roll_tier": "None",
                    "unlock": 8
                }
            },
        ],
    },
]);

db.seasons.insertMany([
    {
        "event_name": "Bread and Circuses",
        "event_desc": "The kingdom is celebrating a festival, accumulate any Token (can be accuried this season). The character who accumilated the most gets 3 exp to Diplomacy",
        "event_length": 2,
        "event_reward": {
            "Experience": {
                "stat_enum": "Diplomacy",
                "stat_name": "Diplomacy",
                "stat_value": 0,
                "stat_exp": 3,
                "stat_token": 0,
            }
        },
    },
    {
        "event_name": "Under Siege!",
        "event_desc": "The Kingdom is under siege from a rival kingdom. Any tokens won this season is sent to the kingdom. (You cannot gain tokens this season.) The person who has given the most tokens to the kingdom, gains RESOLVE",
        "event_length": 3,
        "event_reward": {
            "Ability": {
                "ability_name": "Resolve",
                "ability_desc": "During a Season where Siege is in the name, Play this ability during after the Season is revealed. You gain any tokensduring the siege as well as contributing to the kingdom's need.",
                "ability_unlock": {
                    "might" : "Military",
                    "roll_tier": "None", 
                    "unlock": 0
                }
            }
        },
    },
    {
        "event_name": "Drug Epidemic",
        "event_desc": "During this time, any rolls from abilities have -3 to their bonus. The Character with the Highest Roll at the end of the season gains 2 exp to Science",
        "event_length": 2,
        "event_reward": {
            "Experience": {
                "stat_enum": "Science",
                "stat_name": "Science",
                "stat_value": 0,
                "stat_exp": 2,
                "stat_token": 0,
            }
        }
    },
    {
        "event_name": "Prince's Coming of Age",
        "event_desc": "The prince comes of age, and the kingdom must pitch in for a masterful tournament. Only abilities with Military, Religion, or Diplomacy may be used.",
        "event_length": 3,
        "event_reward": {
            "Ability": {
                "ability_name": "New Generation",
                "ability_desc": "Can only be used once per game. A child is born in your family. Set a 3 season clock. When the clock is over: Gain +1 in every Might, and change your name. You play your new child.",
                "ability_clock": {
                    "clock_duration": 3,
                    "clock_remaining": 3,
                    "clock_name": "New Generation",
                    "clock_desc": "When the clock is over: Gain +1 in every Might, and change your name. You play your new child.",
                    "clock_conf": false, 
                },
                "ability_unlock": {
                    "might" : "Diplomacy",
                    "roll_tier": "None", 
                    "unlock": 0
                }
            }
        }
    },
    {
        "event_name": "Uneasy Alliance",
        "event_desc": "If there are more than 2 characters in the game, Compete with Diplomatic Rolls. The two characters with the most highest amount of success, become allies for 6 rounds. They cannot target sabotages against each other.",
        "event_length": 2,
        "event_reward": {
            "Clock": {
                "clock_duration": 6,
                "clock_remaining": 6,
                "clock_name": "Uneasy Alliance",
                "clock_desc": "The Characters who won the Uneasy Alliance Season: now cannot target each other for negative Abilities or Sabotages. They have to also include eachother in any confidential clocks that they start.",
                "clock_conf": false, 
            }
        },
    },
])


db.abilities.insertMany([
    {
        "ability_name": "Conflict",
        "ability_desc": "You can start a conflict with another character. You both roll either Military, Scientific, or Espionage Might. The one with the higher roll, wins the conflict. The winner chooses one of the Mights to take 2 Experience in.",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    },
    {
        "ability_name": "Conflict",
        "ability_desc": "You can start a conflict with another character. You both roll either Military, Scientific, or Espionage Might. The one with the higher roll, wins the conflict. The winner chooses one of the Mights to take 2 Experience in.",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    },
    {
        "ability_name": "Plot",
        "ability_desc": "Plot for the turn to gain 1 Experience.",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    },
    {
        "ability_name": "Plot",
        "ability_desc": "Plot for the turn to gain 1 Experience.",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    },
    {
        "ability_name": "Backstab",
        "ability_desc": "You can backstab another character. You both roll either Military, Scientific, or Espionage Might. The one with the higher roll, wins the backstab. The loser loses 1 Experience and the winner gains 1 Experience.",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    },
    {
        "ability_name": "Backstab",
        "ability_desc": "You can backstab another character. You both roll either Military, Scientific, or Espionage Might. The one with the higher roll, wins the backstab. The loser loses 1 Experience and the winner gains 1 Experience in an agreed upon Might",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    },
    {
        "ability_name": "Vandalize",
        "ability_desc": "You can vandalize another character. You both roll either Military, Scientific, or Espionage Might. The one with the higher roll, wins the vandalize. The loser loses 2 tokens in an agreed upon Might and the winner gains 2 tokens in the same Might",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    },
    {
        "ability_name": "Vandalize",
        "ability_desc": "You can vandalize another character. You both roll either Military, Scientific, or Espionage Might. The one with the higher roll, wins the vandalize. The loser loses 2 tokens in an agreed upon Might and the winner gains 2 tokens in the same Might",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    },
    {
        "ability_name": "Accumulate Assets",
        "ability_desc": "Gain 2 tokens in a Might",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    },
    {
        "ability_name": "Accumulate Assets",
        "ability_desc": "Gain 2 tokens in a Might",
        "ability_unlock": {
            "might" : "None",
            "roll_tier": "None", 
            "unlock": 0
        }
    }
])
