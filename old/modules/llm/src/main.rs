/// Performs forward encoding in an encoder model.
///
/// # Arguments
///
/// * `out` - The output tensor of shape (B, T, C), where B is the batch size, T is the sequence length, and C is the number of dimensions.
/// * `inp` - The input tensor of shape (B, T), holding the token ids at each position.
/// * `wte` - The token embeddings tensor of shape (V, C), where V is the vocabulary size.
/// * `wpe` - The positional embeddings tensor of shape (maxT, C), where maxT is the maximum sequence length.
/// * `b` - The batch size.
/// * `t` - The sequence length.
/// * `c` - The number of dimensions.
fn encoder_forward(
    out: &mut [f32],
    inp: &[i32],
    wte: &[f32],
    wpe: &[f32],
    b: usize,
    t: usize,
    c: usize,
) {
    for b_idx in 0..b {
        for t_idx in 0..t {
            let out_bt = &mut out[(b_idx * t + t_idx) * c..(b_idx * t + t_idx + 1) * c];
            let ix = inp[b_idx * t + t_idx] as usize;
            let wte_ix = &wte[ix * c..(ix + 1) * c];
            let wpe_t = &wpe[t_idx * c..(t_idx + 1) * c];
            for i in 0..c {
                out_bt[i] = wte_ix[i] + wpe_t[i];
            }
        }
    }
}

#[test]
fn test_valid_input() {
    // Initialize input tensors
    let mut out = vec![0.0; 6];  // This corresponds to (B=1, T=3, C=2)
    let inp = vec![1, 0, 2];     // Indices in WTE
    let wte = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];  // 3 words, each with 2 dimensions
    let wpe = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6];  // Max T=3, C=2

    // Invoke the function
    encoder_forward(&mut out, &inp, &wte, &wpe, 1, 3, 2);

    // Define the expected output
    let expected_out = vec![3.1, 4.2, 1.3, 2.4, 5.5, 6.6];

    // Check if the output matches the expected output
    assert_eq!(out, expected_out);
}
