fn main() {
    #[cfg(target_arch = "x86_64")]

    use std::arch::x86_64::*;

    unsafe {
        // レジスタにベクトル（[0.,1.,2.,3.]）
        let vec_a: __m256d = _mm256_setr_pd(0.,1.,2.,3.);
        let vec_b: __m256d = _mm256_setr_pd(0.,1.,2.,3.);

        // 足し算
        // [0.0, 1.0, 2.0, 3.0] + [0.0, 1.0, 2.0, 3.0]
        // => [0.0, 2.0, 4.0, 6.0]
        let vec_sum = _mm256_add_pd(vec_a, vec_b);
        print!("sum: {:?} + {:?} = {:?}\n", vec_a, vec_a, vec_sum);

        // 掛け算
        // [0.0, 1.0, 2.0, 3.0] * [0.0, 1.0, 2.0, 3.0]
        // => [0.0, 1.0, 4.0, 9.0]
        let vec_mul = _mm256_mul_pd(vec_a, vec_b);
        print!("mul: {:?} * {:?} = {:?}\n", vec_a, vec_b, vec_mul);

        // 引き算
        // [0.0, 2.0, 4.0, 6.0] - [0.0, 1.0, 4.0, 9.0]
        // => [0.0, 1.0, 0.0, -3.0]
        let vec_sub = _mm256_sub_pd(vec_sum, vec_mul);
        print!("sub: {:?} - {:?}  = {:?}\n", vec_sum, vec_mul, vec_sub);

        // 割り算
        // [0.0, 1.0, 4.0, 9.0] /  [0.0, 1.0, 2.0, 3.0]
        // => [NaN, 1.0, 2.0, 3.0]
        let vec_div = _mm256_div_pd(vec_mul, vec_a);
        print!("div: {:?} / {:?}  = {:?}\n", vec_mul, vec_a, vec_div);
    }

}
