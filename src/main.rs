fn main() {
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    unsafe {
        // レジスタにベクトル（[0.,1.,2.,3.]）
        let vec_a: __m256d = _mm256_setr_pd(0., 1., 2., 3.);
        let vec_b: __m256d = _mm256_setr_pd(0., 1., 2., 3.);

        // 足し算
        // [0.0, 1.0, 2.0, 3.0] + [0.0, 1.0, 2.0, 3.0]
        // => [0.0, 2.0, 4.0, 6.0]
        let vec_sum = _mm256_add_pd(vec_a, vec_b);
        println!("sum: {:?} + {:?} = {:?}", vec_a, vec_b, vec_sum);

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

        // 水平足し算
        // [0.0, 1.0, 2.0, 3.0] + [0.0, 1.0, 2.0, 3.0]
        // => [1.0, 1.0, 5.0, 5.0]
        let vec_hsum = _mm256_hadd_pd(vec_a, vec_b);
        println!("hsum: {:?} + {:?} = {:?}", vec_a, vec_b, vec_hsum);

        // 水平引き算
        // [0.0, 2.0, 4.0, 6.0] - [0.0, 1.0, 4.0, 9.0]
        // => [-2.0, -1.0, -2.0, -5.0]
        let vec_hsub = _mm256_hsub_pd(vec_sum, vec_mul);
        println!("hsub: {:?} - {:?}  = {:?}", vec_sum, vec_mul, vec_hsub);

        // 位置入れ替え
        let vec_permute = _mm256_permute_pd(vec_a, 0);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 0, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 1);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 1, vec_permute);

        let vec_permute = _mm256_permute_pd(vec_a, 2);
        println!("permute: {:?}, {:b} => {:?}", vec_a, 2, vec_permute);

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

        let vec_a_128: __m128i = _mm_set_epi64x(1, 2);
        let vec_b_128: __m128i = _mm_set_epi64x(2, 3);

        let shuffled = _mm_shuffle_epi32::<3>(vec_a_128);
        println!("shuffled: {:?}, {:b} => {:?}", vec_a_128, 3, shuffled);

        let xored = _mm_xor_si128(vec_a_128, vec_b_128);
        println!("xor: {:?}, {:?} => {:?}", vec_a_128, vec_b_128, xored);

    }
}

fn simurate_permute(vec: [f64; 4], cmd: u8) -> [f64; 4] {
    let cmd1 = cmd & 0b1111;
    let cmd2 = (cmd & 0b11110000) << 4;

    println!("cmd1: {}", cmd1);
    println!("cmd2: {}", cmd2);

    fn part_of_simurate_permute(vec: [f64; 4], cmd: u8, base: usize) -> [f64; 2] {
        let mut res = [0., 0.];

        let zero = base;
        let one = base + 1;

        if cmd == 0 {
            res = [vec[zero], vec[zero]]
        }

        if cmd == 1 {
            res = [vec[one], vec[zero]]
        }

        if cmd == 10 {
            res = [vec[zero], vec[one]]
        }

        if cmd == 11 {
            res = [vec[one], vec[one]]
        }

        return res;
    }

    let res1 = part_of_simurate_permute(vec, cmd1, 0);
    let res2 = part_of_simurate_permute(vec, cmd2, 2);

    println!("res1: {:?}", res1);
    println!("res2: {:?}", res2);

    return [res1[0], res1[1], res2[0], res2[1]];
}

#[test]
fn test_simurate_permute() {
    let vec = [0., 1., 2., 3.];

    let res = simurate_permute(vec, 0b0000);
    println!("my permute: {:?}, {:b} => {:?}", vec, 0b0000, res);
}
