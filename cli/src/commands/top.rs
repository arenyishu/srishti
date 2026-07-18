use colored::Colorize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use serde_json::Value;

pub fn execute() -> anyhow::Result<()> {
    println!("{}", "Srishti OS - Agent Activity Monitor".cyan().bold());
    println!("{:-<50}", "");
    
    let file = match File::open("srishti_audit.jsonl") {
        Ok(f) => f,
        Err(_) => {
            println!("{} No audit log found. Are any agents running?", "Status:".yellow());
            return Ok(());
        }
    };

    let reader = BufReader::new(file);
    let mut starts: HashMap<String, i32> = HashMap::new();
    let mut finishes: HashMap<String, i32> = HashMap::new();
    let mut tools: HashMap<String, i32> = HashMap::new();
    let mut errors: HashMap<String, i32> = HashMap::new();
    let mut policies: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            if let Ok(val) = serde_json::from_str::<Value>(&l) {
                let agent = val["agent_name"].as_str().unwrap_or("Unknown").to_string();
                let event_type = val["event_type"].as_str().unwrap_or("");
                
                match event_type {
                    "AgentStarted" => *starts.entry(agent.clone()).or_insert(0) += 1,
                    "AgentFinished" => *finishes.entry(agent.clone()).or_insert(0) += 1,
                    "ToolCalled" => *tools.entry(agent.clone()).or_insert(0) += 1,
                    "ToolFailed" => *errors.entry(agent.clone()).or_insert(0) += 1,
                    "PolicyEnforced" => *policies.entry(agent.clone()).or_insert(0) += 1,
                    _ => {}
                }
            }
        }
    }

    println!("{:<20} | {:<8} | {:<8} | {:<8} | {:<8}", "AGENT", "ACTIVE", "TOOLS", "ERRORS", "BLOCKED");
    println!("{:-<60}", "");

    let mut all_agents: Vec<String> = starts.keys().cloned().collect();
    all_agents.sort();

    for agent in all_agents {
        let started = starts.get(&agent).unwrap_or(&0);
        let finished = finishes.get(&agent).unwrap_or(&0);
        let active = (*started).saturating_sub(*finished);
        let active_str = if active > 0 { active.to_string().green() } else { "0".normal() };
        
        let t = tools.get(&agent).unwrap_or(&0);
        let e = errors.get(&agent).unwrap_or(&0);
        let p = policies.get(&agent).unwrap_or(&0);
        
        let err_str = if *e > 0 { e.to_string().red() } else { "0".normal() };
        let pol_str = if *p > 0 { p.to_string().yellow() } else { "0".normal() };

        println!("{:<20} | {:<8} | {:<8} | {:<8} | {:<8}", 
            agent.bold(), 
            active_str, 
            t.to_string().cyan(), 
            err_str, 
            pol_str
        );
    }
    
    Ok(())
}
