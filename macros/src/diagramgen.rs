use crate::parser::*;

/// Generates a string containing 'dot' syntax to generate a statemachine diagram with graphviz.
pub fn generate_diagram(sm: &ParsedStateMachine) -> String {
    let transitions = &sm.states_events_mapping;
    
    let diagram_states = sm.states.iter().map(|s| s.0);
    let mut diagram_events = vec![];
    let mut diagram_transitions = vec![];
    for (state, event) in transitions {
        for (_event, eventmapping) in event {
            diagram_events.push((
                eventmapping.event.to_string(),
                eventmapping.guard.as_ref().map(|i| i.to_string()).unwrap_or_else(|| "_".to_string()),
                eventmapping.action.as_ref().map(|i| i.to_string()).unwrap_or_else(|| "_".to_string()),
            ));
            diagram_transitions.push((
                state,
                eventmapping.out_state.to_string(),
                eventmapping.event.to_string(),
            ));
        }
    }

    let state_string = diagram_states.map(|s| {
        format!("\t{} [shape=box color=\"red\" fillcolor=\"#ffbb33\" style=filled]", s)
    }).collect::<Vec<String>>();
    let event_string = diagram_events.iter().map(|s| {
        format!("\t{0} [label=\"{0}\\n[{1}] / {2}\"]", s.0, s.1, s.2)
    }).collect::<Vec<String>>();
    let transition_string = diagram_transitions.iter().map(|t| {
            format!("\t{0} -> {2}  -> {1};", t.0, t.1, t.2)
    }).collect::<Vec<String>>();

format!("digraph G {{
    rankdir=\"LR\";
    node [fontname=Arial];
    s [shape=circle size=2 color=\"black\" style=filled]
    
    s -> {}
{}

{}

{}
}}",
sm.starting_state.to_string(),
state_string.join("\n"),
event_string.join("\n"),
transition_string.join("\n")
)
}