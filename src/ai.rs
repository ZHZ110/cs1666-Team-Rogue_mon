use crate::battle;
use crate::monster;

use rand::{self, Rng};

use std::collections::HashMap;

/// Returns an integer corresponding to an action the AI will take based upon the difficulty
///
/// [0-3] represent that the AI will attack with one of its moves
/// [4-8] represent that the AI will switch into another monster
/// 
/// * `difficulty` - The difficulty of the AI. 0 is random; 1 is small lookahead; 2 is large lookahead
/// * `monsters` - Maps strings onto their Monster objects; needed for damage calculation
/// * `state` - The current state of the battle
pub fn ai_agent(
    difficulty: usize,
    monsters: &HashMap<String, monster::Monster>,
    battle_state: &mut monster::BattleState,
) -> usize {
    if difficulty == 0 || matches!(battle_state.battle_type, monster::BattleType::Wild) {
        return rand::thread_rng().gen_range(0..4) as usize;
    } else if difficulty == 1 {
        match alphabeta(&monsters, battle_state, 5,  -f64::INFINITY, f64::INFINITY, true).1 {
            Some(s) => {s}
            None => 0
        }
    } else {
        match alphabeta(&monsters, battle_state, 8,  -f64::INFINITY, f64::INFINITY, true).1 {
            Some(s) => {s}
            None => 0
        }
    }
}
    

/// Returns the sum of health percentages (0-100) for all team monsters
///
/// * `team` - The team *(as vector of (str monster, health))*
fn total_team_health(team: &Vec<(String, f32, usize)>) -> f32 {
    return team.iter().map(|d| d.1).sum();
}

/// Returns a numeric evaluation of the current battle state
///
/// Used in the α-β algorithm to give an estimated payoff for non-terminal states.
/// In our case, the maximizing player is our AI; and the minimizing player is the player.
///
/// * `min_team` - The team *(as vector of (str monster, health))* of the minimizing player
/// * `max_team` - The team *(as vector of (str monster, health))* of the maximizing player
/// * `turn_number` - The number of turns elapsed in battle; Discourage AI from stalling
fn evaluation_function(min_team: &Vec<(String, f32, usize)>, max_team: &Vec<(String, f32, usize)>) -> f64 { // turn_number: usize) -> f64 {
    let min_team_health: f32 = total_team_health(min_team);
    let max_team_health: f32 = total_team_health(max_team);
    println!("Eval = {}", ((max_team_health - min_team_health) as f64));
    return (max_team_health - min_team_health) as f64;
}

/// Returns the number of monsters that can be switched into battle
///
/// * `team` - The team *(as vector of (str monster, health))*
fn num_switchable_mons(team: &Vec<(String, f32, usize)>) -> usize {
    let alive_mons = team.iter().filter(|d| d.1 > 0.0).count();
    return if alive_mons == 0 { 0 } else { alive_mons - 1 };
}

/// Runs the α-β algorithm and returns the payoff and action for the optimal path of play
///
/// * `monsters` - Maps strings onto their Monster objects; needed for damage calculation
/// * `state` - The current state of the battle
/// * `alpha` - Best available payoff for the max agent (AI) so far
/// * `beta` - Best available payoff for the min agent (player) so far
/// * `maximizing_player` - Determines which player we are optimizing for (max = AI; min = player)
pub fn alphabeta(
    monsters: &HashMap<String, monster::Monster>,
    state: &mut monster::BattleState,
    depth: i32,
    mut alpha: f64,
    mut beta: f64,
    maximizing_player: bool,
) -> (f64, Option<usize>) {
    // Value will store the payoff of any actions an agent takes
    let mut value: f64;

    // We will return the payoff, but more importantly the action taken to get that payoff
    let mut ret: (f64, Option<usize>) = (0.0, None);

    // Terminal test: if one team has no alive monsters
    let battle_end =
        total_team_health(&state.player_team) == 0.0 && total_team_health(&state.enemy_team) == 0.0;

    // If depth limit is reached or battle has ended, return the evaluation function of the game state
    if depth == 0 || battle_end {
        return (
            evaluation_function(&state.player_team, &state.enemy_team), // state.turn_number),
            None,
        );
    }

    // Execute a search of the game tree for the given player
    //   In our case, maximizing player is the AI (against the player)
    if maximizing_player {
        // Initialize the payoff as the WORST possible case for the maximizing player
        value = -f64::INFINITY;

        // Go thru all actions for the AI/opponent player
        //   0..=3 being one of the current lead's 4 moves
        //   4..=(up to 8) being one of the possible (up to 5) other monsters to switch into
        for action in 0..=(3 + num_switchable_mons(&state.enemy_team)) {
            let temp = value;

            // Create a new state to update based upon the action taken
            let mut new_state = monster::BattleState {
                player_turn: !state.player_turn,
                player_team: state.player_team.clone(),
                enemy_team: state.enemy_team.clone(),
                self_attack_stages: state.self_attack_stages,
                self_defense_stages: state.self_defense_stages,
                opp_attack_stages: state.opp_attack_stages,
                opp_defense_stages: state.opp_defense_stages,
                player_badges: state.player_badges,
                battle_type: state.battle_type,
            };

            // Change the new state based upon the action (attack or switch in another monster)
            if action < 4 {
                // Action corresponding to a move

                // Calculate the new health of the opponent (player)
                let mut new_health = new_state.player_team[0].1
                    - monster::calculate_damage(monsters, &mut new_state, action);
                new_health = new_health.clamp(0.0, 100.0);
                new_state.player_team[0].1 = new_health;

                // Makes sure an active monster is still in front after attack
                new_state.player_team = battle::verify_team(&new_state.player_team);
            } else {
                // Action corresponding to a switch
                new_state.enemy_team.swap(0, action - 3);
                new_state.opp_attack_stages = 0;
                new_state.opp_defense_stages = 0;
                new_state.enemy_team = battle::verify_team(&new_state.enemy_team);
            }

            // Following our move, find out which one leads to the best payoff by traversing the game tree
            value = value.max(alphabeta(monsters, &mut new_state, depth - 1, alpha, beta, false).0);

            // Update the return value if value is updated
            if value != temp {
                ret = (value, Some(action))
            }

            // Prune remaining actions if possible
            if value >= beta {
                break; // (* β cutoff *)
            }

            // Update alpha (the best option so far for maximizing player)
            alpha = alpha.max(value);
        }
        return ret;
    } else {
        // Initialize the payoff as the WORST possible case for the minimizing player
        value = f64::INFINITY;

        // Go thru all actions for the player
        //   0..=3 being one of the current lead's 4 moves
        //   4..=(up to 8) being one of the possible (up to 5) other monsters to switch into
        for action in 0..=(3 + num_switchable_mons(&state.player_team)) {
            let temp = value;

            // Create a new state to update based upon the action taken
            let mut new_state = monster::BattleState {
                player_turn: !state.player_turn,
                player_team: state.player_team.clone(),
                enemy_team: state.enemy_team.clone(),
                self_attack_stages: state.self_attack_stages,
                self_defense_stages: state.self_defense_stages,
                opp_attack_stages: state.opp_attack_stages,
                opp_defense_stages: state.opp_defense_stages,
                player_badges: state.player_badges,
                battle_type: state.battle_type,
            };

            // Change the new state based upon the action (attack or switch in another monster)
            if action < 4 {
                // Action corresponding to a move
                // Calculate the new health of the opponent (AI)
                let mut new_health = new_state.enemy_team[0].1
                    - monster::calculate_damage(monsters, &mut new_state, action);
                new_health = new_health.clamp(0.0, 100.0);
                new_state.enemy_team[0].1 = new_health;
                
                // Makes sure an active monster is still in front after attack
                new_state.enemy_team = battle::verify_team(&new_state.enemy_team);
            } else {
                // Action corresponding to a switch
                new_state.player_team.swap(0, action - 3);
                new_state.self_attack_stages = 0;
                new_state.self_defense_stages = 0;
                new_state.player_team = battle::verify_team(&new_state.player_team);
            }
            // Following our move, find out which one leads to the best payoff by traversing the game tree
            value = value.min(alphabeta(monsters, &mut new_state, depth - 1, alpha, beta, true).0);
            // Update the return value if value is updated
            if value != temp {
                ret = (value, Some(action))
            }

            // Prune remaining actions if possible
            if value <= alpha {
                break; // (* α cutoff *)
            }

            // Update alpha (the best option so far for minimizing player)
            beta = beta.min(value);
        }
        return ret;
    }
}
