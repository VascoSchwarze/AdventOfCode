use std::collections::{HashMap, VecDeque};

use num::integer::lcm;

#[derive(Debug, PartialEq, Clone, Copy)]
enum PulseIntensity {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Pulse {
    intensity: PulseIntensity,
    source: String,
    target: String,
}

#[derive(Debug, PartialEq)]
enum FlipFlopMachineState {
    On,
    Off,
}

#[derive(Debug)]
enum MachineType {
    Conjunction,
    FlipFlop,
    Broadcaster,
}

#[derive(Debug)]
struct Machine {
    machine_type: MachineType,
    destination_machines: Vec<String>,
}

#[derive(Debug)]
struct ConjunctionMachineState {
    state: HashMap<String, PulseIntensity>,
}

#[derive(Debug)]
struct Factory {
    machines: HashMap<String, Machine>,
    conjunction_machines_state: HashMap<String, ConjunctionMachineState>,
    flip_flop_machines_state: HashMap<String, FlipFlopMachineState>,
}

impl Factory {
    fn send_pulse(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let target_machine = self.machines.get(&pulse.target);

        if target_machine.is_none() {
            return vec![];
        }
        let target_machine = target_machine.unwrap();

        let mut pulse_to_send: Option<PulseIntensity> = None;
        match target_machine.machine_type {
            MachineType::FlipFlop => {
                if pulse.intensity == PulseIntensity::Low {
                    if *self
                        .flip_flop_machines_state
                        .get(&pulse.target)
                        .expect("Should have a flip flop state")
                        == FlipFlopMachineState::Off
                    {
                        self.flip_flop_machines_state
                            .insert(pulse.target.clone(), FlipFlopMachineState::On);
                        pulse_to_send = Some(PulseIntensity::High);
                    } else {
                        self.flip_flop_machines_state
                            .insert(pulse.target.clone(), FlipFlopMachineState::Off);
                        pulse_to_send = Some(PulseIntensity::Low);
                    }
                }
            }
            MachineType::Conjunction => {
                self.conjunction_machines_state
                    .entry(pulse.target.clone())
                    .and_modify(|state| {
                        state.state.insert(pulse.source, pulse.intensity);
                    });
                if self
                    .conjunction_machines_state
                    .get(&pulse.target)
                    .expect("Should have a conjunction state")
                    .state
                    .iter()
                    .all(|(_, intensity)| *intensity == PulseIntensity::High)
                {
                    pulse_to_send = Some(PulseIntensity::Low);
                } else {
                    pulse_to_send = Some(PulseIntensity::High);
                }
            }
            MachineType::Broadcaster => {
                pulse_to_send = Some(pulse.intensity);
            }
        };
        let mut pulses_to_send = vec![];
        if let Some(pulse_intensity_to_send) = pulse_to_send {
            for destination in &target_machine.destination_machines {
                pulses_to_send.push(Pulse {
                    intensity: pulse_intensity_to_send,
                    source: pulse.target.clone(),
                    target: destination.clone(),
                });
            }
        }
        pulses_to_send
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let mut factory = Factory {
        machines: HashMap::new(),
        conjunction_machines_state: HashMap::new(),
        flip_flop_machines_state: HashMap::new(),
    };

    for line in input.lines() {
        let (machine, destinations) = line.split_once(" -> ").unwrap();
        match machine.chars().next().unwrap() {
            '%' => {
                let machine_name = String::from_iter(machine.chars().skip(1));
                factory.machines.insert(
                    machine_name.clone(),
                    Machine {
                        machine_type: MachineType::FlipFlop,
                        destination_machines: destinations
                            .split(", ")
                            .map(|d| d.to_string())
                            .collect(),
                    },
                );
                factory
                    .flip_flop_machines_state
                    .insert(machine_name, FlipFlopMachineState::Off);
            }
            '&' => {
                let machine_name = String::from_iter(machine.chars().skip(1));
                factory.machines.insert(
                    machine_name.clone(),
                    Machine {
                        machine_type: MachineType::Conjunction,
                        destination_machines: destinations
                            .split(", ")
                            .map(|d| d.to_string())
                            .collect(),
                    },
                );
                factory.conjunction_machines_state.insert(
                    machine_name,
                    ConjunctionMachineState {
                        state: HashMap::new(),
                    },
                );
            }
            'b' => {
                let machine_name = machine.to_string();
                factory.machines.insert(
                    machine_name.clone(),
                    Machine {
                        machine_type: MachineType::Broadcaster,
                        destination_machines: destinations
                            .split(", ")
                            .map(|d| d.to_string())
                            .collect(),
                    },
                );
            }
            _ => unreachable!(),
        }
    }

    // set input machines for all conjunction machines
    let conjunction_machines: Vec<String> =
        factory.conjunction_machines_state.keys().cloned().collect();
    for machine_name in conjunction_machines {
        let input_machines: Vec<(&String, &Machine)> = factory
            .machines
            .iter()
            .filter(|(_, machine)| machine.destination_machines.contains(&machine_name))
            .collect();
        for input_machine in input_machines {
            factory
                .conjunction_machines_state
                .entry(machine_name.clone())
                .and_modify(|state| {
                    state
                        .state
                        .insert(input_machine.0.clone(), PulseIntensity::Low);
                });
        }
    }


    let mut modules_to_rx: Vec<String> = vec!["rx".to_string()];
    while modules_to_rx.len() == 1 {
        modules_to_rx = factory
            .machines
            .iter()
            .filter_map(|(machine_name, machine)| {
                if machine.destination_machines.contains(&modules_to_rx[0]) {
                    Some(machine_name.clone())
                } else {
                    None
                }
            })
            .collect();
    }

    // By the nature of the input, those modules are conjunction modules. They lead into another module that then leads to rx.
    // These modules are in seperate cycles. When all of them send a 1 at the same time, a 0 will be sent to rx.
    // Therefore, we need to calculate the lcm of the button presses it takes for them to send a 1.

    let mut button_presses_until_module_to_rx_sends_high_pulse: HashMap<String, u64> =
        HashMap::new();
    let mut button_presses: u64 = 0;
    while modules_to_rx
        .iter()
        .any(|module| !button_presses_until_module_to_rx_sends_high_pulse.contains_key(module))
    {
        let mut pulses = VecDeque::from(vec![Pulse {
            intensity: PulseIntensity::Low,
            target: "broadcaster".to_string(),
            source: "button".to_string(),
        }]);
        button_presses += 1;
        while let Some(pulse) = pulses.pop_front() {
            if modules_to_rx.contains(&pulse.source) && pulse.intensity == PulseIntensity::High {
                button_presses_until_module_to_rx_sends_high_pulse
                    .insert(pulse.source.clone(), button_presses);
            }

            let new_pulses = factory.send_pulse(pulse);
            for new_pulse in new_pulses {
                pulses.push_back(new_pulse);
            }
        }
    }

    let result: u64 = button_presses_until_module_to_rx_sends_high_pulse
        .values()
        .cloned()
        .reduce(lcm)
        .unwrap();
    println!("{result}");
}
