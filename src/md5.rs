const SHIFT_CONSTANTS: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

pub struct Md5 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Md5 {
    pub fn new() -> Md5 {
        Md5 {
            a: 0x67452301,
            b: 0xefcdab89,
            c: 0x98badcfe,
            d: 0x10325476,
        }
    }

    pub fn pad_input(user_input: &[u8]) -> Vec<u8> {
        // converting slice to Vec so fn has its Owned copy to modiy and then return
        let mut user_input_fn_copy = user_input.to_vec();
        // find total bits count of actual user input
        // eg: Talha = 5 bytes = 5x8 = 40 bits
        //  whatever count u get store that as 64 bit number
        // let user_input_bits_count = user_input.len() * 8; .len on string gives total bytes count
        // after padding with 1 and 0 step, so last 8 bytes that we reserve is for this
        let user_input_len_64bit = (user_input.len() * 8) as u64;
        // add 1 bit to the end
        // since we r dealing in bytes so will append 10000000 (128 in decimal) ASCCI equiv of 1 is 128
        user_input_fn_copy.push(128);
        while user_input_fn_copy.len() % 64 != 56 {
            // keep pushing 0 until 8 bytes less than 64 multiple (512 bit)
            // while padding step is 2000% similar to sha-256
            user_input_fn_copy.push(0);
        }
        // finally pushing ORIGINAL user input BITS count as u64 number (8 bytes)
        user_input_fn_copy.extend_from_slice(&user_input_len_64bit.to_le_bytes());
        return user_input_fn_copy;
    }

    // floor(abs(sin(i+1)) * 2^32)
    pub fn compute_t(i: usize) -> u32 {
        return (f64::sin((i + 1) as f64).abs() * 2.0_f64.powi(32)).floor() as u32;
    }

    // if b's bit is 1 take c's bit else take d's bit
    pub fn f(b: u32, c: u32, d: u32) -> u32 {
        return (b & c) | (!b & d);
    }
    // if d's bit is 1 take b's bit else take c's bit
    pub fn g(b: u32, c: u32, d: u32) -> u32 {
        return (b & d) | (c & !d);
    }
    // XOR is associative
    // suppose first bit of b c and d are as follow 1,1,0
    //  sum of 1 bit = 2 = even = output bit = 0
    //  1,0,0 sum of 1 bit = 1 = odd = outputbit = 1
    // "Maximum diffusion" means: every single one of B, C, D's bits contributes to the output.

    pub fn h(b: u32, c: u32, d: u32) -> u32 {
        return b ^ c ^ d;
    }
    pub fn i(b: u32, c: u32, d: u32) -> u32 {
        c ^ (b | !d)
    }

    pub fn compress(&mut self, block: &[u8; 64]) -> () {
        let block_32bits: [u32; 16] = std::array::from_fn(|i| {
            u32::from_le_bytes(block[(i * 4)..(i * 4 + 4)].try_into().unwrap())
        });
        let t_constants: [u32; 64] = std::array::from_fn(|i| Self::compute_t(i));
        let (aa, bb, cc, dd) = (self.a, self.b, self.c, self.d);
        let fns: [fn(u32, u32, u32) -> u32; 4] = [Self::f, Self::g, Self::h, Self::i];

        for i in 0..64 {
            let k = match i / 16 {
                0 => i,
                1 => (5 * i + 1) % 16,
                2 => (3 * i + 5) % 16,
                3 => (7 * i) % 16,
                _ => std::process::exit(1),
            };

            let injection = self
                .a
                .wrapping_add(fns[i / 16](self.b, self.c, self.d))
                .wrapping_add(block_32bits[k])
                .wrapping_add(t_constants[i]);

            self.a = self.d;
            self.d = self.c;
            self.c = self.b;
            self.b = self
                .b
                .wrapping_add(injection.rotate_left(SHIFT_CONSTANTS[i]))
        }

        self.a = self.a.wrapping_add(aa);
        self.b = self.b.wrapping_add(bb);
        self.c = self.c.wrapping_add(cc);
        self.d = self.d.wrapping_add(dd);
    }

    pub fn finalise(&self) -> [u8; 16] {
        let mut hash = [0u8; 16];
        hash[0..4].copy_from_slice(&self.a.to_le_bytes());
        hash[4..8].copy_from_slice(&self.b.to_le_bytes());
        hash[8..12].copy_from_slice(&self.c.to_le_bytes());
        hash[12..16].copy_from_slice(&self.d.to_le_bytes());

        return hash;
    }
}
