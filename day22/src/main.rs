use std::collections::{HashMap, VecDeque};

type Deck = VecDeque<u64>;

fn parse_deck(input: &str) -> VecDeque<u64> {
    let mut pieces = input.split("\n").filter(|s| !s.is_empty());
    let playerid = pieces
        .next()
        .unwrap()
        .split(|c| c == ' ' || c == ':')
        .filter(|s| !s.is_empty())
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let stack = pieces
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<VecDeque<_>>();
    stack
}

fn play_combat_round(player1_deck: &mut Deck, player2_deck: &mut Deck) {
    let player1_card = player1_deck.pop_front().unwrap();
    let player2_card = player2_deck.pop_front().unwrap();
    if player1_card > player2_card {
        player1_deck.push_back(player1_card);
        player1_deck.push_back(player2_card);
    } else if player1_card < player2_card {
        player2_deck.push_back(player2_card);
        player2_deck.push_back(player1_card);
    } else {
        panic!("Got the same card from both players. Someone was cheating")
    }
}

fn player_won<'a>(player1: &'a Deck, player2: &'a Deck) -> Option<&'a Deck> {
    if player1.len() == 0 {
        Some(player2)
    } else if player2.len() == 0 {
        Some(player1)
    } else {
        None
    }
}

fn calculate_deck_score(deck: &Deck) -> u64 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, v)| (idx + 1) as u64 * v)
        .sum()
}

fn check_previous_config(deck: &VecDeque<u64>, previous_decks: &Vec<Vec<u64>>) -> bool {
    for prev in previous_decks.iter() {
        if prev.len() != deck.len() {
            continue;
        }
        if prev.iter().zip(deck.iter()).map(|(a, b)| a == b).all(|b| b) {
            return true;
        }
    }

    false
}

fn rcombat(players: &mut (Deck, Deck)) -> u64 {
    let mut previous_decks_player1 = Vec::new();
    let mut previous_decks_player2 = Vec::new();

    loop {
        if check_previous_config(&players.0, &previous_decks_player1)
            || check_previous_config(&players.1, &previous_decks_player2)
        {
            return 0;
        }

        previous_decks_player1.push(players.0.iter().cloned().collect::<Vec<_>>());
        previous_decks_player2.push(players.1.iter().cloned().collect::<Vec<_>>());

        let player1_card = players.0.pop_front().unwrap();
        let player2_card = players.1.pop_front().unwrap();

        if players.0.len() >= player1_card as usize && players.1.len() >= player2_card as usize {
            let new_deck_player1 = players
                .0
                .iter()
                .take(player1_card as usize)
                .cloned()
                .collect::<Deck>();
            let new_deck_player2 = players
                .1
                .iter()
                .take(player2_card as usize)
                .cloned()
                .collect::<Deck>();
            let winner_subgame = rcombat(&mut (new_deck_player1, new_deck_player2));

            if winner_subgame == 0 {
                players.0.push_back(player1_card);
                players.0.push_back(player2_card);
            } else {
                players.1.push_back(player2_card);
                players.1.push_back(player1_card);
            }
        } else {
            if player1_card > player2_card {
                players.0.push_back(player1_card);
                players.0.push_back(player2_card);
            } else if player1_card < player2_card {
                players.1.push_back(player2_card);
                players.1.push_back(player1_card);
            } else {
                panic!("two players delt the same card. Someone cheated");
            }
        }

        if players.0.len() == 0 {
            return 1;
        } else if players.1.len() == 0 {
            return 0;
        }
    }
}

fn main() {
    let mut decks = include_str!("input.txt")
        .split("\n\n")
        .map(|s| parse_deck(s));

    let mut deck_player1 = decks.next().unwrap();
    let mut deck_player2 = decks.next().unwrap();

    let mut deck_player1_combat = deck_player1.clone();
    let mut deck_player2_combat = deck_player2.clone();
    while player_won(&deck_player1_combat, &deck_player2_combat).is_none() {
        play_combat_round(&mut deck_player1_combat, &mut deck_player2_combat);
    }

    let deck_won = player_won(&deck_player1_combat, &deck_player2_combat).unwrap();
    println!(
        "Winning deck score of combat {}",
        calculate_deck_score(deck_won)
    );

    let mut deck_player1_rcombat = deck_player1.clone();
    let mut deck_player2_rcombat = deck_player2.clone();
    let mut rcombat_decks = (deck_player1_rcombat, deck_player2_rcombat);

    let won_rcombat = rcombat(&mut rcombat_decks);
    let won_score = if won_rcombat == 0 {
        calculate_deck_score(&rcombat_decks.0)
    } else {
        calculate_deck_score(&rcombat_decks.1)
    };
    println!("Winning deck score of recursive combat {}", won_score);
}
