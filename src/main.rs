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
        println!("sum: {:?} + {:?} = {:?}", vec_a, vec_a, vec_sum);

        // 掛け算
        // [0.0, 1.0, 2.0, 3.0] * [0.0, 1.0, 2.0, 3.0]
        // => [0.0, 1.0, 4.0, 9.0]
        let vec_mul = _mm256_mul_pd(vec_a, vec_b);
        println!("mul: {:?} * {:?} = {:?}", vec_a, vec_b, vec_mul);

        // 引き算
        // [0.0, 2.0, 4.0, 6.0] - [0.0, 1.0, 4.0, 9.0]
        // => [0.0, 1.0, 0.0, -3.0]
        let vec_sub = _mm256_sub_pd(vec_sum, vec_mul);
        println!("sub: {:?} - {:?}  = {:?}", vec_sum, vec_mul, vec_sub);

        // 割り算
        // [0.0, 1.0, 4.0, 9.0] /  [0.0, 1.0, 2.0, 3.0]
        // => [NaN, 1.0, 2.0, 3.0]
        let vec_div = _mm256_div_pd(vec_mul, vec_a);
        println!("div: {:?} / {:?}  = {:?}", vec_mul, vec_a, vec_div);

        // 位置入れ替え
        let vec_permute = _mm256_permute_pd(vec_a, 0);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 0, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 1);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 1, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 2);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 2 , vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 3);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 3, vec_permute);
        
        let vec_permute = _mm256_permute_pd(vec_a, 4);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 4, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 5);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 5, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 6);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 6, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 7);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 7, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 8);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 8, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 9);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 9, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 10);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 10, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 11);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 11, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 12);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 12, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 13);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 13, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 14);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 14, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 15);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 15, vec_permute);
    }
}

