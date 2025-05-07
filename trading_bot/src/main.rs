mod strategy;
mod risk;

use strategy::trader::{Trader, Signal};
use risk::risk_manager::TradeDecision;

use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use serde::Deserialize;
use rand::Rng;

#[derive(Debug, Deserialize)]
struct StockData {
    date: String,
    price: f64,
    open: f64,
    high: f64,
    low: f64,
    vol: f64,
    change_pct: f64,
}


#[derive(Debug)]
struct QLearningAgent {
    q_table: Vec<Vec<f64>>,  
    alpha: f64,              
    gamma: f64,              
    epsilon: f64,            
    actions: Vec<String>,    
}

impl QLearningAgent {
    fn new(num_states: usize, num_actions: usize, alpha: f64, gamma: f64, epsilon: f64) -> Self {
        let q_table = vec![vec![0.0; num_actions]; num_states];
        let actions = vec!["buy".to_string(), "hold".to_string(), "sell".to_string()];
        QLearningAgent {
            q_table,
            alpha,
            gamma,
            epsilon,
            actions,
        }
    }

    fn choose_action(&self, state: usize) -> usize {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.epsilon {
            rng.gen_range(0..self.actions.len())
        } else {
            self.q_table[state]
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(idx, _)| idx)
                .unwrap()
        }
    }

    fn update_q_table(&mut self, state: usize, action: usize, reward: f64, next_state: usize) {
        let max_future_q = *self.q_table[next_state]
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let current_q = self.q_table[state][action];
        self.q_table[state][action] = current_q
            + self.alpha * (reward + self.gamma * max_future_q - current_q);
    }
}

fn read_stock_data(file_path: &str) -> Result<Vec<StockData>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: StockData = result?;
        records.push(record);
    }

    Ok(records)
}

fn main() {
    let stock_data = read_stock_data("applestockhistoricaldata.csv").expect("Error reading CSV");

    let num_states = stock_data.len();
    let num_actions = 3; // Buy, Hold, Sell
    let mut agent = QLearningAgent::new(num_states, num_actions, 0.1, 0.9, 0.1);

    let trader = Trader::new(0.05, 0.1);

    let mut total_profit = 0.0;
    let mut entry_price = 0.0;
    let mut holding = false;

    for i in 0..stock_data.len() - 1 {
        let state = i;
        let next_state = i + 1;
        let current_price = stock_data[i].price;
        let next_price = stock_data[next_state].price;

        let action_idx = agent.choose_action(state);
        let action = agent.actions[action_idx].clone();

        match action.as_str() {
            "buy" => {
                if !holding {
                    entry_price = current_price;
                    holding = true;
                    println!("Bought at: {}", entry_price);
                }
            }
            "sell" => {
                if holding {
                    let reward = trader.evaluate_trade(entry_price, next_price);
                    total_profit += reward;
                    holding = false;
                    println!("Sold at: {}. Profit: {}", next_price, reward);
                }
            }
            "hold" => {}
            _ => {}
        }

        let reward = if holding {
            trader.evaluate_trade(entry_price, next_price)
        } else {
            0.0
        };
        agent.update_q_table(state, action_idx, reward, next_state);
    }

    println!("Total Profit: {}", total_profit);
}
//done