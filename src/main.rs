use quest_rs::{Complex, ComplexMatrix2, ComplexMatrixN, QReal, QuReg, QuestEnv, Vector};

fn main() {
    let env = QuestEnv::new();
    let mut qubits = QuReg::new(3, &env);

    qubits.init_zero_state();

    println!("Out environment is:");
    qubits.report_params();
    env.report();

    // Set up the circuitry

    let unitary_alpha = Complex::new(0.5, 0.5);
    let unitary_beta = Complex::new(0.5, -0.5);

    let unitary_matrix = ComplexMatrix2 {
        real: [[0.5, 0.5], [0.5, 0.5]],
        imag: [[0.5, -0.5], [-0.5, 0.5]],
    };

    let mut toffoli_gate = ComplexMatrixN::new(3);
    for i in 0..6 {
        toffoli_gate.set_real(i, i, 1.0);
    }
    toffoli_gate.set_real(6, 7, 1.0);
    toffoli_gate.set_real(7, 6, 1.0);

    qubits
        .hadamard(0)
        .controlled_not(0, 1)
        .rotate_y(2, 0.1)
        .multi_controlled_phase_flip(vec![0, 1, 2])
        .unitary(0, unitary_matrix)
        .compact_unitary(1, unitary_alpha, unitary_beta)
        //.rotate_around_axis(2, (PI / 2) as QReal, Vector::new(1.0, 0.0, 0.0))
        .rotate_around_axis(2, (3.14 / 2.0) as QReal, Vector::new(1.0, 0.0, 0.0))
        .controlled_compact_unitary(0, 1, unitary_alpha, unitary_beta)
        .multi_controlled_unitary(vec![0, 1], 2, unitary_matrix)
        .multi_qubit_unitary(vec![0, 1, 2], toffoli_gate);

    // Study the output
    println!("Circuit output:");
    println!("---------------");

    let prob_amp_state_111 = qubits.probability_amplitude(0b111);
    println!("Probability amplitude of |111> is: {}", prob_amp_state_111);

    let prob_qubit_two_in_state_1 = qubits.calculate_probability_of_outcome(2, 1);
    println!(
        "Probability of qubit 2 being in state 1: {}",
        prob_qubit_two_in_state_1
    );

    println!("Qubit 0 was measured in state: {}", qubits.measure(0));
    let (outcome, outcome_probability) = qubits.measure_with_stats(2);
    println!(
        "Qubit 2 collapsed to {} with probability {}",
        outcome, outcome_probability
    );
}
