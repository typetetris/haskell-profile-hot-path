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
struct AnalysedCostCentre {
    cost_centre: CostCentre,
    ticks: u64,
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
struct TopLevel<P, C> {
    program: String,
    arguments: Vec<String>,
    rts_arguments: Vec<String>,
    end_time: String,
    initial_capabilities: u32,
    total_time: f64,
    total_ticks: u64,
    tick_interval: u32,
    total_alloc: u64,
    cost_centres: Vec<C>,
    profile: P,
}

fn analyse_profile(
    input: Profile,
    cost_centres: &HashMap<u64, CostCentre>,
    cost_centres_ticks: &mut HashMap<u64, u64>,
) -> AnalysedProfile {
    let mut our_children: Vec<AnalysedProfile> = input
        .children
        .iter()
        .cloned()
        .map(|c| analyse_profile(c, cost_centres, cost_centres_ticks))
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

    {
        let cost_centre_ticks = cost_centres_ticks.entry(input.cost_centre).or_insert(0);
        *cost_centre_ticks += input.ticks;
    }

    AnalysedProfile {
        label,
        module,
        src_loc,
        entries: input.entries,
        alloc: input.alloc,
        ticks: input.ticks,
        ticks_cumulative: our_children.iter().map(|c| c.ticks_cumulative).sum::<u64>()
            + input.ticks,
        children: our_children,
    }
}

fn analyse(input: TopLevel<Profile, CostCentre>) -> TopLevel<AnalysedProfile, AnalysedCostCentre> {
    let cost_centres: HashMap<u64, CostCentre> = input
        .cost_centres
        .iter()
        .cloned()
        .map(|c| (c.id, c))
        .collect();
    let mut cost_centres_ticks: HashMap<u64, u64> = HashMap::new();
    let profile = analyse_profile(input.profile, &cost_centres, &mut cost_centres_ticks);
    let mut cost_centres: Vec<AnalysedCostCentre> = input
        .cost_centres
        .into_iter()
        .map(|cost_centre| {
            let id = cost_centre.id;
            AnalysedCostCentre {
                cost_centre,
                ticks: *cost_centres_ticks.get(&id).unwrap_or(&0),
            }
        })
        .collect();
    cost_centres.sort_by(|lhs, rhs| lhs.ticks.cmp(&rhs.ticks).reverse());
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
        cost_centres,
        profile,
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
