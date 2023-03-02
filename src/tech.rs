use petgraph::DiGraph;
/*
A tech tree is a directed acyclic graph (DAG) of techs.
When a tech is unlocked, it basically grants "permission" to access any function that depends on it
TODO: how does a function state and check its tech dependency? (a function only can have 1 tech dependency,
because techs don't duplicate unlocks)
Each player has there own tech tree
The UI needs to reference the tech tree so it knows what to display
The systems need to reference the tech tree so they know if they user is allowed to do that
Or the backend doesn't care, so the UI must make sure the user can't do anything they don't have tech for
*/


enum Ability {
    Build(Unit),
    Build(Structure),
    Upgrade(Visbility, i32),
}

struct Tech(String, Vec<Ability>);

fn tech_tree() -> tr::Tree<Tech> {
    let mut deps = DiGraph::<Tech, &str>::new();
    let spaceflight = deps.add_node(Tech("Spaceflight", vec![Ability::Build(Unit::Probe)]));
    let radar = deps.add_node(Tech("Radar", vec![Ability::Upgrade(Upgrade::Visibility(1))]));
    deps.extend_with_edges(&[(pg, fb), (pg, qc), (qc, rand), (rand, libc), (qc, libc)]);
}
