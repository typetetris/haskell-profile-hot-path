use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Read};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct CostCentre {
    id: u64,
    label: String,
    module: String,
    src_loc: String,
    is_caf: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct Profile {
    #[serde(rename(deserialize = "id"))]
    cost_centre: u64,
    entries: u64,
    alloc: u64,
    ticks: u64,
    children: Vec<Profile>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct AnalysedProfile {
    label: String,
    module: String,
    src_loc: String,
    entries: u64,
    alloc: u64,
    ticks: u64,
    ticks_cumulative: u64,
    children: Vec<AnalysedProfile>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TopLevel<P> {
    program: String,
    arguments: Vec<String>,
    rts_arguments: Vec<String>,
    end_time: String,
    initial_capabilities: u32,
    total_time: f64,
    total_ticks: u64,
    tick_interval: u32,
    total_alloc: u64,
    cost_centres: Vec<CostCentre>,
    profile: P,
}

fn analyse_profile(input: Profile, cost_centres: &HashMap<u64, CostCentre>) -> AnalysedProfile {
    let mut our_children: Vec<AnalysedProfile> = input
        .children
        .iter()
        .cloned()
        .map(|c| analyse_profile(c, cost_centres))
        .collect();
    our_children.sort_by(|lhs, rhs| lhs.ticks_cumulative.cmp(&rhs.ticks_cumulative).reverse());

    let (label, module, src_loc) = cost_centres.get(&input.cost_centre).map_or(
        (
            "ERROR".to_string(),
            "ERROR".to_string(),
            "ERROR".to_string(),
        ),
        |c| (c.label.clone(), c.module.clone(), c.src_loc.clone()),
    );

    AnalysedProfile {
        label,
        module,
        src_loc,
        entries: input.entries,
        alloc: input.alloc,
        ticks: input.ticks,
        ticks_cumulative: our_children.iter().map(|c| c.ticks_cumulative).sum::<u64>() + input.ticks,
        children: our_children,
    }
}

fn analyse(input: TopLevel<Profile>) -> TopLevel<AnalysedProfile> {
    let cost_centres: HashMap<u64, CostCentre> = input.cost_centres.iter().cloned().map(|c| (c.id, c)).collect();
    TopLevel {
        program: input.program,
        arguments: input.arguments,
        rts_arguments: input.rts_arguments.clone(),
        end_time: input.end_time.clone(),
        initial_capabilities: input.initial_capabilities,
        total_time: input.total_time,
        total_ticks: input.total_ticks,
        tick_interval: input.tick_interval,
        total_alloc: input.total_alloc,
        cost_centres: input.cost_centres,
        profile: analyse_profile(input.profile, &cost_centres),
    }
}

fn main() -> anyhow::Result<()> {
    let mut data = Vec::new();
    let stdin = std::io::stdin();
    stdin.lock().read_to_end(&mut data)?;

    let mut deserializer = serde_json::Deserializer::from_slice(&data);
    deserializer.disable_recursion_limit();
    for input in deserializer.into_iter() {
        let stdout = std::io::stdout();
        serde_json::to_writer(stdout.lock(), &analyse(input?))?;
    }

    Ok(())
}
